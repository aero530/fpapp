# Accounts Data Structures

The `accounts` crate is UI-agnostic.

## The `Account` trait

Every account type implements `Account`. The two methods that drive the simulation are:

**`init(linked_dates, settings)`** → `Result<Vec<(u32, YearlyImpact)>>`

Seeds internal year tables from user-supplied historical data. Returns pre-existing `(year, impact)` pairs that are applied to `YearlyTotals` before the main simulation loop starts — allowing past account activity to set opening balances.

**`simulate(year, totals, settings, linked_value)`** → `Result<YearlyImpact>`

Computes the account's effect for one year. Called in chronological order. Returns a `YearlyImpact` that is immediately accumulated into `YearlyTotals`, making it available to accounts that run later in the same year.

Three sub-traits add optional behaviour: `AccountSavings` (contribution + withdrawal amounts), `AccountExpense` (expense amount), `AccountPayment` (loan/mortgage payment amount). Each is implemented by the corresponding proc-macro derive crate (`account_savings_derive`, `account_expense_derive`, `account_payment_derive`).

## Account types and simulation order

Accounts are simulated in a fixed type order every year. Order matters because later accounts can read totals accumulated by earlier ones.

| # | Type | Notes |
|---|---|---|
| 1 | `Income` | Salary, wages, or other recurring income. Sets `income` and `income_taxable` for the year. |
| 2 | `Ssa` | Social Security. Taxable fraction interpolated from SSA settings breakpoints based on total income. |
| 3 | `Expense` | Fixed or inflation-adjusted costs. Contributes to `expense` and `col`. Healthcare expenses set `healthcare_expense` instead. |
| 4 | `Hsa` | Must run after Expense so it can read `totals.healthcare_expense` and apply HSA funds to cover it. Withdrawals zero out `healthcare_expense` before end-of-year settlement. |
| 5 | `Mortgage` | Amortized payment with escrow and PMI. PMI drops once LTV falls below the configured threshold. |
| 6 | `Loan` | Fixed or inflation-adjusted payment with amortized principal/interest split. |
| 7 | `College` | 529-style savings. Uses `ContributePretaxUntaxedWhenUsed`. Writes neither to `saving` nor `hsa` pools. |
| 8 | `Retirement` | 401k, IRA, Roth. Supports income-linked employer matching. Writes to the `saving` pool. |
| 9 | `Savings` | General savings or brokerage account. Writes to the `saving` pool. |

## Input types

### `YearInput`

Flexible year reference used for `start_in`, `end_in`, `start_out`, `end_out` on every account.

| Variant | Example JSON | Meaning |
|---|---|---|
| `ConstantInt(u32)` | `2045` | Literal calendar year. |
| `Suggested(YearSuggestion)` | `"yearRetire"` | Keyword resolved against `Settings` or linked-account dates. Options: `yearStart`, `yearRetire`, `yearDie`, `yearEnd`, `incomeLink`. |
| `Calculate { base, delta }` | `{"base":"yearRetire","delta":-2}` | Keyword ± integer offset. |

### `ContributionOptions`

| Variant | Behaviour |
|---|---|
| `Fixed` | Constant dollar amount in today's dollars (not inflation-adjusted). |
| `PercentOfIncome` | Fraction of the linked income account's value for that year. |
| `FixedWithInflation` | Amount grows each year by `inflation_base` compounded from `year_start`. |

### `WithdrawalOptions`

| Variant | Behaviour |
|---|---|
| `Fixed` | Constant dollar withdrawal each year of the withdrawal window. |
| `FixedWithInflation` | Withdrawal amount grows with inflation from `year_start`. |
| `EndAtZero` | Amortizes the opening balance of the withdrawal window evenly to zero over the withdrawal period. Amount re-computed each year from remaining balance and remaining years. |
| `ColFracOfSavings` | Withdrawal = `col × (account_balance / total_savings)`. Scales spending proportionally to this account's share of all savings. |

### `TaxStatus`

| Variant | Treatment |
|---|---|
| `ContributeTaxedEarningsUntaxedWhenUsed` | **Roth.** Contributions from after-tax dollars; withdrawals are non-taxable (`income_taxable = 0`). |
| `ContributeTaxedEarningsTaxed` | **Taxed both ways.** Contributions do not reduce taxable income; withdrawals are fully taxable. |
| `ContributePretaxTaxedWhenUsed` | **Traditional 401k / IRA.** Contributions reduce taxable income; withdrawals counted as taxable income. |
| `ContributePretaxUntaxedWhenUsed` | **HSA / 529.** Contributions are pre-tax; withdrawals are non-taxable. HSA writes to `hsa` pool; College writes to neither pool. |

## Simulation flow

Runs once per analysis request over the full year range `[year_start, year_die]`.

### Phase 1 — Initialisation (before the year loop)

1. **Determine simulation order** — build a UUID list by iterating the fixed `AccountType::order()` sequence and collecting UUIDs for each type. This order governs the inner loop in Phase 2.
2. **Call `account.init()` for each account** — resolves linked-account dates and seeds internal tables from historical user data. Pre-existing `(year, impact)` pairs are applied to `YearlyTotals`, setting opening balances before the forward simulation begins.

### Phase 2 — Year loop

For each year `y` in `[year_start, year_die]`:

1. **Open the year** — `YearlyTotals::add_year(y, pull_forward=true)` initialises all accumulators for `y` and carries `net`, `saving`, and `hsa` balances forward from year `y − 1`. All other fields start at zero.
2. **Simulate each account** — for each UUID in type order: resolve `linked_value` (the linked income account's balance for year `y`, if any), call `account.simulate(y, &totals, &settings, linked_value)`, and immediately apply the returned `YearlyImpact` to `totals`. Later accounts in the same year see the updated totals.
3. **End-of-year settlement** — after all accounts have run:
   - Add `income` to `net`
   - Deduct `income_taxable × tax_rate` from `net`; record as `tax_burden`
   - Deduct `expense` from `net`
   - Deduct any remaining `healthcare_expense` from `net` (zeroing the field — the HSA account already paid what it could in step 2)

## `YearlyImpact` — per account, per year

Returned by `simulate()`. All fields default to zero; each account fills only the fields it affects.

| Field | Meaning |
|---|---|
| `expense` | Cash outflow paid from `net` at end of year. |
| `healthcare_expense` | Medical costs. HSA covers as much as possible; remainder is charged to `net`. |
| `col` | Cost-of-living contribution. Tracks total lifestyle spend; used as denominator in the `ColFracOfSavings` withdrawal strategy. |
| `saving` | Net delta to the savings pool. Positive on contribution years, negative on withdrawal years. HSA and College accounts must leave this zero. |
| `hsa` | Net delta to the HSA pool. Only HSA accounts write here; all other accounts leave this zero. |
| `income_taxable` | Portion of income subject to the flat income-tax rate. Traditional withdrawals and SSA income land here. |
| `income` | Total income including non-taxable amounts. Deposited into `net` at end of year. |

**Pool routing invariants:**
- HSA accounts → `hsa` only (`saving = 0`)
- College accounts → neither pool (`saving = 0`, `hsa = 0`)
- Retirement & Savings → `saving` only (`hsa = 0`)

## `YearlyTotals` — accumulated across all accounts

A collection of year-keyed `Table<u32>` time series. Three fields carry their values forward year-to-year; the rest reset to zero at the start of each year.

| Field | Carries forward | Notes |
|---|---|---|
| `net` | yes | Liquid cash balance. Income flows in, taxes and expenses flow out at end of year. |
| `saving` | yes | Aggregate savings balance (retirement + savings accounts). |
| `hsa` | yes | Aggregate HSA balance. Kept separate from `saving` because HSA funds have different tax treatment. |
| `income` / `income_taxable` | no | Accumulated from Income, SSA, and retirement withdrawal accounts. |
| `expense` / `healthcare_expense` / `col` | no | Accumulated from Expense accounts. |
| `tax_burden` | no | Written once during end-of-year settlement as `income_taxable × tax_rate`. |