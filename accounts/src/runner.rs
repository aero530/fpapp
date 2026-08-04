use crate::{Account, AccountType, Dates, Error, PlotDataSet, SimAccount, UserData, YearlyTotals};
use std::collections::HashMap;

/// Per-account plot series keyed by account uuid, plus the aggregate totals
pub type AnalysisOutput = (HashMap<String, Vec<PlotDataSet>>, YearlyTotals);

pub fn run(mut data: UserData<SimAccount>) -> Result<AnalysisOutput, Error> {
    // Reject configurations that would silently produce an empty or corrupt run
    data.settings.validate()?;

    // Build a deterministic simulation order: the fixed type sequence, then by
    // account name and uuid within a type (HashMap iteration order must never
    // influence the results)
    let mut account_order: Vec<String> = Vec::new();
    for type_id in AccountType::order().iter() {
        let mut group: Vec<(String, String)> = data
            .accounts
            .iter()
            .filter(|(_, account)| account.type_id() == *type_id)
            .map(|(uuid, account)| (account.name(), uuid.clone()))
            .collect();
        group.sort();
        account_order.extend(group.into_iter().map(|(_, uuid)| uuid));
    }

    // Inclusive range: year_end() is the user's final year of life, not a past-the-end sentinel
    let years: Vec<u32> = (data.settings.year_start()..=data.settings.year_end()).collect();
    let mut yearly_totals = YearlyTotals::new();

    // Init pass: resolve linked dates and seed analysis tables from historical data
    for uuid in &account_order {
        let linked_dates: Option<Dates> = match data
            .accounts
            .get(uuid)
            .ok_or_else(|| Error::internal(format!("account {} missing from map", uuid)))?
            .link_id()
        {
            Some(link_id) => match data.accounts.get(&link_id) {
                Some(linked) if linked.type_id() == AccountType::Income => Some(Dates {
                    year_in: linked.get_range_in(&data.settings, None),
                    year_out: linked.get_range_out(&data.settings, None),
                }),
                Some(linked) => {
                    // Same rule the year loop applies to linked values: only
                    // Income accounts can be linked, so don't resolve dates
                    // from a wrong-typed account either
                    log::warn!(
                        "income_link on '{}' points to {:?}, not Income — treating as unlinked",
                        data.accounts[uuid].name(),
                        linked.type_id()
                    );
                    None
                }
                None => {
                    log::warn!(
                        "linked account {} not found — treating as unlinked",
                        link_id
                    );
                    None
                }
            },
            None => None,
        };

        let account = data.accounts.get_mut(uuid).unwrap();
        account
            .init(linked_dates, &data.settings)
            .map_err(|e| e.with_context(format!("account '{}'", account.name())))?;
    }

    // Resolve link ids once instead of cloning them for every account-year
    let link_ids: HashMap<String, Option<String>> = account_order
        .iter()
        .map(|uuid| (uuid.clone(), data.accounts[uuid].link_id()))
        .collect();

    for year in years {
        yearly_totals.add_year(year, true)?;

        for uuid in account_order.iter() {
            let link_value = match &link_ids[uuid] {
                Some(id) => match data.accounts.get(id) {
                    Some(la) if la.type_id() == AccountType::Income => la.get_value(year),
                    Some(la) => {
                        log::warn!(
                            "income_link on '{}' points to {:?}, not Income",
                            data.accounts[uuid].name(),
                            la.type_id()
                        );
                        None
                    }
                    None => None,
                },
                None => None,
            };

            let account = data.accounts.get_mut(uuid).unwrap();
            let impact = account
                .simulate(year, &yearly_totals, &data.settings, link_value)
                .map_err(|e| Error::Simulation {
                    account: account.name(),
                    year,
                    message: e.message().to_string(),
                })?;
            yearly_totals.update(year, impact);
        }

        // The savings and hsa totals are the sum of the relevant account
        // balances.  Recomputing them from balances (rather than accumulating
        // per-account deltas) keeps them exact when historical table entries
        // override computed balances mid-simulation.  College accounts are
        // deliberately excluded from the savings pool: their balances are
        // earmarked for education, not retirement cost of living.
        let mut saving_total = 0_f64;
        let mut hsa_total = 0_f64;
        for account in data.accounts.values() {
            match account.type_id() {
                AccountType::Savings | AccountType::Retirement => {
                    saving_total += account.get_value(year).unwrap_or_default();
                }
                AccountType::Hsa => {
                    hsa_total += account.get_value(year).unwrap_or_default();
                }
                _ => {}
            }
        }
        yearly_totals.set_saving(year, saving_total);
        yearly_totals.set_hsa(year, hsa_total);

        yearly_totals.deposit_income_in_net(year);
        yearly_totals.pay_income_tax_from_net(
            year,
            data.settings.tax_income,
            data.settings.tax_capital_gains,
        );
        yearly_totals.pay_expenses_from_net(year);
        yearly_totals.pay_healthcare_expenses_from_net(year);
    }

    let mut plot_data: HashMap<String, Vec<PlotDataSet>> = HashMap::new();
    for (uuid, account) in data.accounts.iter() {
        plot_data.insert(uuid.clone(), account.get_plot_data());
    }

    Ok((plot_data, yearly_totals))
}

#[cfg(test)]
mod tests {
    use crate::{SimAccount, UserData, YearlyTotals};
    use float_cmp::assert_approx_eq;

    /// Build UserData from a JSON accounts fragment using compact settings:
    /// simulation 2020..=2030, no inflation, 20% income tax, 10% capital gains tax
    fn user_data(accounts_json: &str) -> UserData<SimAccount> {
        let json = format!(
            r#"{{
                "settings": {{
                    "ageRetire": 45,
                    "ageDie": 50,
                    "yearBorn": 1980,
                    "yearStart": 2020,
                    "inflationBase": 0.0,
                    "taxIncome": 20.0,
                    "taxCapitalGains": 10.0,
                    "retirementCostOfLiving": 100.0,
                    "ssa": {{
                        "breakpoints": {{ "low": 30000, "high": 40000 }},
                        "taxableIncomePercentage": {{ "low": 50, "high": 85 }}
                    }}
                }},
                "accounts": {{ {} }}
            }}"#,
            accounts_json
        );
        serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("test json failed to deserialize: {e}"))
    }

    fn income_json(name: &str, base: f64) -> String {
        format!(
            r#""{name}": {{
                "type": "income", "name": "{name}", "base": {base},
                "startIn": 2020, "endIn": 2030, "raise": 0.0, "notes": null, "table": {{}}
            }}"#
        )
    }

    fn expense_json(name: &str, value: f64) -> String {
        format!(
            r#""{name}": {{
                "type": "expense", "name": "{name}", "table": {{}},
                "startOut": 2020, "endOut": 2030,
                "expenseType": "fixed", "expenseValue": {value},
                "isHealthcare": false, "notes": null
            }}"#
        )
    }

    fn run_totals(accounts_json: &str) -> YearlyTotals {
        crate::run(user_data(accounts_json)).unwrap().1
    }

    #[test]
    fn net_carries_debt_forward() {
        // Regression test for A1: expenses exceed income, so net must go
        // negative and keep sinking — not snap back to an earlier positive value.
        let accounts = format!(
            "{},{}",
            income_json("job", 10000.0),
            expense_json("rent", 15000.0)
        );
        let totals = run_totals(&accounts);
        // per year: net += 10000 - 2000 tax - 15000 = -7000
        assert_approx_eq!(f64, totals.net.get(2020).unwrap(), -7000.0);
        assert_approx_eq!(f64, totals.net.get(2021).unwrap(), -14000.0);
        assert_approx_eq!(f64, totals.net.get(2025).unwrap(), -42000.0);
    }

    #[test]
    fn historical_entry_mid_simulation_preserves_net_continuity() {
        // Regression test for A2: a historical balance entry mid-simulation must
        // not reset accumulated net for that year.
        let accounts = format!(
            "{},{}",
            income_json("job", 10000.0),
            r#""sav": {
                "type": "savings", "name": "sav",
                "table": { "2020": 1000, "2025": 1000 },
                "contributions": null, "earnings": null, "withdrawals": null,
                "startIn": 2020, "endIn": 2030, "startOut": 2031, "endOut": 2031,
                "contributionValue": 0.0, "contributionType": "fixed",
                "yearlyReturn": 0.0, "withdrawalType": "other", "withdrawalValue": 0.0,
                "taxStatus": "contribute_taxed_earnings_untaxed_when_used", "notes": null
            }"#
        );
        let totals = run_totals(&accounts);
        // net accumulates 8000/year (10000 income - 20% tax) with no gaps
        for (i, year) in (2020..=2030).enumerate() {
            assert_approx_eq!(f64, totals.net.get(year).unwrap(), 8000.0 * (i + 1) as f64);
        }
    }

    #[test]
    fn account_rollforward_ignores_future_table_entries() {
        // Regression test for A3: the 2025 historical entry must not leak into 2021-2024.
        let accounts = r#""sav": {
            "type": "savings", "name": "sav",
            "table": { "2020": 100, "2025": 500 },
            "contributions": null, "earnings": null, "withdrawals": null,
            "startIn": 2020, "endIn": 2030, "startOut": 2031, "endOut": 2031,
            "contributionValue": 0.0, "contributionType": "fixed",
            "yearlyReturn": 0.0, "withdrawalType": "other", "withdrawalValue": 0.0,
            "taxStatus": "contribute_taxed_earnings_untaxed_when_used", "notes": null
        }"#;
        let totals = run_totals(accounts);
        // savings total is the account balance: 100 until the 2025 override
        assert_approx_eq!(f64, totals.saving.get(2021).unwrap(), 100.0);
        assert_approx_eq!(f64, totals.saving.get(2024).unwrap(), 100.0);
        assert_approx_eq!(f64, totals.saving.get(2025).unwrap(), 500.0);
        assert_approx_eq!(f64, totals.saving.get(2026).unwrap(), 500.0);
    }

    #[test]
    fn hsa_contribution_debits_net_and_reduces_tax() {
        // Regression test for A5 at the runner level.
        let accounts = format!(
            "{},{}",
            income_json("job", 50000.0),
            r#""hsa": {
                "type": "hsa", "name": "hsa", "table": { "2020": 0 },
                "startIn": 2020, "endIn": 2030, "startOut": 2020, "endOut": 2030,
                "contributionValue": 1000.0, "contributionType": "fixed",
                "employerContribution": 0.0, "yearlyReturn": 0.0,
                "taxStatus": "contribute_pretax_untaxed_when_used", "notes": null
            }"#
        );
        let totals = run_totals(&accounts);
        // taxable = 50000 - 1000 = 49000; tax = 9800; net = 50000 - 9800 - 1000 = 39200
        assert_approx_eq!(f64, totals.tax_burden.get(2020).unwrap(), 9800.0);
        assert_approx_eq!(f64, totals.net.get(2020).unwrap(), 39200.0);
        assert_approx_eq!(f64, totals.hsa.get(2020).unwrap(), 1000.0);
    }

    #[test]
    fn ssa_taxability_sees_retirement_withdrawals() {
        // Regression test for A6 (ordering): SSA benefits must be taxed against
        // income that includes retirement withdrawals from the same year.
        let accounts = r#"
            "ret": {
                "type": "retirement", "name": "ret", "table": { "2020": 500000 },
                "contributions": null, "earnings": null, "withdrawals": null,
                "employerContributions": null,
                "startIn": 2020, "endIn": 2020, "startOut": 2020, "endOut": 2030,
                "contributionValue": 0.0, "contributionType": "fixed",
                "yearlyReturn": 0.0, "withdrawalType": "fixed", "withdrawalValue": 50000.0,
                "taxStatus": "contribute_taxed_earnings_untaxed_when_used",
                "incomeLink": null, "matching": null, "notes": null
            },
            "ssa": {
                "type": "ssa", "name": "ssa", "base": 10000.0,
                "startIn": 2020, "endIn": 2030, "notes": null
            }"#;
        let totals = run_totals(accounts);
        // combined income = 50000 withdrawal + 5000 (half benefit) = 55000 > 40000
        // taxable benefit = min(0.85*(55000-40000) + min(0.5*10000, 0.5*10000), 0.85*10000) = 8500
        assert_approx_eq!(f64, totals.income_taxable.get(2020).unwrap(), 8500.0);
    }

    #[test]
    fn college_balance_stays_out_of_savings_pool() {
        // Regression test for A7.
        let accounts = r#""529": {
            "type": "college", "name": "529", "table": { "2020": 20000 },
            "contributions": null, "earnings": null, "withdrawals": null,
            "startIn": 2020, "endIn": 2030, "startOut": 2031, "endOut": 2031,
            "contributionValue": 0.0, "contributionType": "fixed",
            "yearlyReturn": 0.0, "withdrawalType": "other", "withdrawalValue": 0.0,
            "taxStatus": "contribute_taxed_earnings_untaxed_when_used", "notes": null
        }"#;
        let totals = run_totals(accounts);
        for year in 2020..=2030 {
            assert_approx_eq!(f64, totals.saving.get(year).unwrap(), 0.0);
        }
    }

    #[test]
    fn mortgage_pays_off_and_payments_stop() {
        // Regression test for A4 at the runner level.
        let accounts = r#""mort": {
            "type": "mortgage", "name": "mort", "table": { "2020": 1000 },
            "startOut": 2020, "endOut": 2030,
            "paymentType": "fixed", "paymentValue": 600.0,
            "rate": 0.0, "compoundTime": 12.0,
            "mortgageInsurance": 50.0, "ltvLimit": 0.0,
            "escrowValue": 50.0, "homeValue": 100000.0, "notes": null
        }"#;
        let totals = run_totals(accounts);
        assert_approx_eq!(f64, totals.expense.get(2020).unwrap(), 600.0);
        assert_approx_eq!(f64, totals.expense.get(2021).unwrap(), 600.0);
        // paid off after two years — no phantom payments
        for year in 2022..=2030 {
            assert_approx_eq!(f64, totals.expense.get(year).unwrap(), 0.0);
        }
    }

    #[test]
    fn malformed_table_year_is_an_error_not_a_panic() {
        // Regression test for the C-class crash vector: a bad year key in a
        // hand-edited file is a parse error, never a panic.
        let json = r#"{
            "settings": {
                "ageRetire": 45, "ageDie": 50, "yearBorn": 1980, "yearStart": 2020,
                "inflationBase": 0.0, "taxIncome": 20.0, "taxCapitalGains": 10.0,
                "retirementCostOfLiving": 100.0,
                "ssa": {
                    "breakpoints": { "low": 30000, "high": 40000 },
                    "taxableIncomePercentage": { "low": 50, "high": 85 }
                }
            },
            "accounts": {
                "bad": {
                    "type": "income", "name": "bad", "base": 100.0,
                    "startIn": 2020, "endIn": 2030, "raise": 0.0, "notes": null,
                    "table": { "20x0": 5.0 }
                }
            }
        }"#;
        let parsed: Result<UserData<SimAccount>, _> = serde_json::from_str(json);
        assert!(parsed.is_err());
    }

    #[test]
    fn misordered_ssa_breakpoints_are_rejected() {
        // Regression test: low > high used to feed a negative term into the
        // SSA taxable-benefit formula; now the run fails fast with a clear error.
        let json = r#"{
            "settings": {
                "ageRetire": 45, "ageDie": 50, "yearBorn": 1980, "yearStart": 2020,
                "inflationBase": 0.0, "taxIncome": 20.0, "taxCapitalGains": 10.0,
                "retirementCostOfLiving": 100.0,
                "ssa": {
                    "breakpoints": { "low": 50000, "high": 40000 },
                    "taxableIncomePercentage": { "low": 50, "high": 85 }
                }
            },
            "accounts": {}
        }"#;
        let data: UserData<SimAccount> = serde_json::from_str(json).unwrap();
        let err = crate::run(data).unwrap_err();
        assert!(err.to_string().contains("breakpoints"));
    }

    #[test]
    fn init_errors_carry_single_layer_of_context() {
        // Regression test: the banner used to read "invalid configuration:
        // account 'X': invalid configuration: HSA account 'X': ..." — the
        // account name and the variant prefix must each appear exactly once.
        let accounts = r#""h": {
            "type": "hsa", "name": "My HSA", "table": {},
            "startIn": 2020, "endIn": 2030, "startOut": 2020, "endOut": 2030,
            "contributionValue": 0.0, "contributionType": "fixed",
            "employerContribution": 0.0, "yearlyReturn": 0.0,
            "taxStatus": "contribute_pretax_taxed_when_used", "notes": null
        }"#;
        let err = crate::run(user_data(accounts)).unwrap_err();
        let text = err.to_string();
        assert_eq!(text.matches("invalid configuration").count(), 1, "{text}");
        assert_eq!(text.matches("My HSA").count(), 1, "{text}");
    }

    #[test]
    fn negative_dollar_inputs_are_rejected_at_init() {
        // A negative contribution used to drive the balance negative and
        // abort mid-simulation with an internal error; now it is a clear
        // config error before the run starts.
        let accounts = r#""sav": {
            "type": "savings", "name": "sav", "table": {},
            "contributions": null, "earnings": null, "withdrawals": null,
            "startIn": 2020, "endIn": 2030, "startOut": 2031, "endOut": 2031,
            "contributionValue": -100.0, "contributionType": "fixed",
            "yearlyReturn": 0.0, "withdrawalType": "other", "withdrawalValue": 0.0,
            "taxStatus": "contribute_taxed_earnings_untaxed_when_used", "notes": null
        }"#;
        let err = crate::run(user_data(accounts)).unwrap_err();
        assert!(err.to_string().contains("negative"), "{err}");
    }

    #[test]
    fn negative_historical_table_seed_is_rejected_at_init() {
        let accounts = r#""sav": {
            "type": "savings", "name": "sav", "table": { "2020": -500 },
            "contributions": null, "earnings": null, "withdrawals": null,
            "startIn": 2020, "endIn": 2030, "startOut": 2031, "endOut": 2031,
            "contributionValue": 0.0, "contributionType": "fixed",
            "yearlyReturn": 0.0, "withdrawalType": "other", "withdrawalValue": 0.0,
            "taxStatus": "contribute_taxed_earnings_untaxed_when_used", "notes": null
        }"#;
        let err = crate::run(user_data(accounts)).unwrap_err();
        assert!(err.to_string().contains("2020"), "{err}");
    }

    #[test]
    fn income_link_to_non_income_account_is_ignored_at_init() {
        // A retirement account whose incomeLink points at an expense must
        // still initialize and run (dates fall back, values unlinked) instead
        // of resolving dates from the wrong-typed account.
        let accounts = format!(
            "{},{}",
            expense_json("rent", 1000.0),
            r#""ret": {
                "type": "retirement", "name": "ret", "table": {},
                "contributions": null, "earnings": null, "withdrawals": null,
                "employerContributions": null,
                "startIn": "incomeLink", "endIn": "incomeLink",
                "startOut": 2031, "endOut": 2031,
                "contributionValue": 100.0, "contributionType": "fixed",
                "yearlyReturn": 0.0, "withdrawalType": "other", "withdrawalValue": 0.0,
                "taxStatus": "contribute_taxed_earnings_untaxed_when_used",
                "incomeLink": "rent", "matching": null, "notes": null
            }"#
        );
        let totals = run_totals(&accounts);
        // incomeLink falls back to the simulation bounds, so the fixed
        // contribution happens every year
        assert_approx_eq!(f64, totals.saving.get(2020).unwrap(), 100.0);
        assert_approx_eq!(f64, totals.saving.get(2030).unwrap(), 1100.0);
    }

    #[test]
    fn income_historical_actual_outside_window_still_counts() {
        // Same rule as expenses: a recorded actual is charged even when the
        // year falls outside the account's active window.
        let accounts = r#""inc": {
            "type": "income", "name": "inc", "base": 100.0,
            "startIn": 2025, "endIn": 2030, "raise": 0.0, "notes": null,
            "table": { "2020": 999.0 }
        }"#;
        let totals = run_totals(accounts);
        assert_approx_eq!(f64, totals.income.get(2020).unwrap(), 999.0);
        assert_approx_eq!(f64, totals.income.get(2021).unwrap(), 0.0);
        assert_approx_eq!(f64, totals.income.get(2025).unwrap(), 100.0);
    }

    #[test]
    fn single_year_settlement_contract() {
        // Locks the end-of-year settlement order for a year with all four
        // components: wages, a pretax retirement contribution, an ordinary
        // expense, and healthcare partially covered by an HSA.
        let accounts = format!(
            "{},{},{},{}",
            income_json("job", 50000.0),
            r#""ret": {
                "type": "retirement", "name": "ret", "table": { "2020": 0 },
                "contributions": null, "earnings": null, "withdrawals": null,
                "employerContributions": null,
                "startIn": 2020, "endIn": 2030, "startOut": 2031, "endOut": 2031,
                "contributionValue": 5000.0, "contributionType": "fixed",
                "yearlyReturn": 0.0, "withdrawalType": "other", "withdrawalValue": 0.0,
                "taxStatus": "contribute_pretax_taxed_when_used",
                "incomeLink": null, "matching": null, "notes": null
            }"#,
            expense_json("rent", 10000.0),
            r#""hc": {
                "type": "expense", "name": "hc", "table": {},
                "startOut": 2020, "endOut": 2030,
                "expenseType": "fixed", "expenseValue": 2000.0,
                "isHealthcare": true, "notes": null
            },
            "hsa": {
                "type": "hsa", "name": "hsa", "table": { "2020": 1500 },
                "startIn": 2021, "endIn": 2021, "startOut": 2020, "endOut": 2030,
                "contributionValue": 0.0, "contributionType": "fixed",
                "employerContribution": 0.0, "yearlyReturn": 0.0,
                "taxStatus": "contribute_pretax_untaxed_when_used", "notes": null
            }"#
        );
        let totals = run_totals(&accounts);
        // taxable = 50000 wages - 5000 pretax contribution = 45000 -> 9000 tax
        assert_approx_eq!(f64, totals.tax_burden.get(2020).unwrap(), 9000.0);
        // expense = 10000 rent + 5000 contribution outflow
        assert_approx_eq!(f64, totals.expense.get(2020).unwrap(), 15000.0);
        // healthcare: 2000 gross, 1500 covered by HSA, 500 charged to net
        assert_approx_eq!(
            f64,
            totals.healthcare_expense_total.get(2020).unwrap(),
            2000.0
        );
        assert_approx_eq!(f64, totals.hsa.get(2020).unwrap(), 0.0);
        // net = 50000 - 9000 tax - 15000 expenses - 500 residual healthcare
        assert_approx_eq!(f64, totals.net.get(2020).unwrap(), 25500.0);
        // savings pool holds the retirement balance
        assert_approx_eq!(f64, totals.saving.get(2020).unwrap(), 5000.0);
    }
}
