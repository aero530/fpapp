use std::collections::HashMap;
use std::error::Error;

use accounts::{Account, AccountType, AccountWrapper, Dates, PlotDataSet, UserData, YearlyTotals};
use serde_json::Value;

pub fn run_analysis(
    data: &Value,
) -> Result<(HashMap<String, Vec<PlotDataSet>>, YearlyTotals), String> {
    let user_data: UserData<AccountWrapper> =
        serde_json::from_value(data.clone()).map_err(|e| format!("Parse error: {}", e))?;
    let boxed: UserData<Box<dyn Account>> = user_data.into();
    analyze(boxed).map_err(|e| e.to_string())
}

fn analyze(
    mut data: UserData<Box<dyn Account>>,
) -> Result<(HashMap<String, Vec<PlotDataSet>>, YearlyTotals), Box<dyn Error>> {
    let mut account_order: Vec<String> = Vec::new();
    for type_id in AccountWrapper::order().iter() {
        for (uuid, account) in data.accounts.iter() {
            if account.type_id() == *type_id {
                account_order.push(uuid.clone());
            }
        }
    }

    // Inclusive range: year_end() is the user's final year of life, not a past-the-end sentinel
    let years: Vec<u32> = (data.settings.year_start()..=data.settings.year_end()).collect();
    let mut yearly_totals = YearlyTotals::new();

    for uuid in &account_order {
        let linked_dates: Option<Dates> = match data.accounts
            .get(uuid)
            .ok_or_else(|| format!("account {} missing from map", uuid))?
            .link_id()
        {
            Some(link_id) => {
                match data.accounts.get(&link_id) {
                    Some(linked) => Some(Dates {
                        year_in: linked.get_range_in(&data.settings, None),
                        year_out: linked.get_range_out(&data.settings, None),
                    }),
                    None => {
                        log::warn!("linked account {} not found — treating as unlinked", link_id);
                        None
                    }
                }
            }
            None => None,
        };

        let impacts = data.accounts
            .get_mut(uuid)
            .ok_or_else(|| format!("account {} missing from map", uuid))?
            .init(linked_dates, &data.settings)?;

        let sim_start = data.settings.year_start();
        let sim_end = data.settings.year_end();
        for (year, impact) in &impacts {
            if *year < sim_start || *year > sim_end {
                continue; // skip historical entries outside the simulation window
            }
            if !yearly_totals.contains_year(*year) {
                yearly_totals.add_year(*year, false).ok();
            }
            yearly_totals.update(*year, *impact);
        }
    }

    for year in years {
        // Register year if not already pre-seeded by historical table data
        if !yearly_totals.contains_year(year) {
            yearly_totals.add_year(year, true).ok();
        }
        // Always simulate every year in the configured range
        for uuid in account_order.iter() {
            let link_id = data.accounts.get(uuid).unwrap().link_id();
            let link_value = match link_id {
                Some(ref id) => match data.accounts.get(id) {
                    Some(la) if la.type_id() == AccountType::Income => la.get_value(year),
                    Some(la) => {
                        log::warn!(
                            "income_link on '{}' points to {:?}, not Income",
                            data.accounts.get(uuid).unwrap().name(),
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
                .map_err(|e| {
                    format!("Account '{}', year {}: {}", account.name(), year, e)
                })?;
            yearly_totals.update(year, impact);
        }

        yearly_totals.deposit_income_in_net(year);
        yearly_totals.pay_income_tax_from_net(year, data.settings.tax_income);
        yearly_totals.pay_expenses_from_net(year);
        yearly_totals.pay_healthcare_expenses_from_net(year);
    }

    let mut plot_data: HashMap<String, Vec<PlotDataSet>> = HashMap::new();
    for (uuid, account) in data.accounts.iter() {
        plot_data.insert(uuid.clone(), account.get_plot_data());
    }

    Ok((plot_data, yearly_totals))
}
