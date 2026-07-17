# Accounts Data Structures

The `accounts` crate is UI-agnostic.

## The `Account` trait

Every account type implements `Account`. The two methods that drive the simulation are:

**`init(linked_dates, settings)`** → `Result<()>`

Seeds internal year tables from user-supplied historical data, resolves the account's active date ranges, and validates configuration (e.g. a mortgage rejects `compound_time <= 0`, a college account rejects unsupported tax statuses, negative dollar inputs and historical values are rejected, and rates/returns must be greater than −100%). Must be called before `simulate`.

**`simulate(year, totals, settings, linked_value)`** → `Result<YearlyImpact>`

Computes the account's effect for one year. Called in chronological order. Returns a `YearlyImpact` that is immediately accumulated into `YearlyTotals`, making it available to accounts that run later in the same year.

Contribution / withdrawal / payment / expense amounts are computed by `value()` methods on the input option enums (`ContributionOptions`, `WithdrawalOptions`, `PaymentOptions`, `ExpenseOptions`) in `inputs/`.

## Account types and simulation order

Accounts are simulated in a fixed type order every year (ties within a type break by account name, then UUID, so results are deterministic). Order matters because later accounts can read totals accumulated by earlier ones.

| # | Type | Notes |
|---|---|---|
| 1 | `Income` | Salary, wages, or other recurring income. Sets `income` and `income_taxable` for the year. Historical table entries override the computed value. |
| 2 | `Expense` | Fixed or inflation-adjusted costs. Contributes to `expense` and `col`. Healthcare expenses set `healthcare_expense` instead. Historical table entries override the computed value. |
| 3 | `Hsa` | Must run after Expense so it can read `totals.healthcare_expense` and apply HSA funds to cover it. Employee contributions count as an expense and follow the account's tax status (normally a pre-tax deduction). |
| 4 | `Mortgage` | Amortized payment with escrow and PMI. PMI drops once LTV falls below the configured threshold. The payoff-year payment is capped at balance + escrow + PMI so the principal actually reaches zero. |
| 5 | `Loan` | Fixed or inflation-adjusted payment with amortized principal/interest split. Loans and mortgages log a warning when a payment fails to cover interest (negative amortization) or when the payment window ends with a balance outstanding. |
| 6 | `College` | 529-style savings. Only `ContributeTaxedEarningsUntaxedWhenUsed` is supported (validated at init). Balances stay out of the savings pool. |
| 7 | `Retirement` | 401k, IRA, Roth. Supports income-linked employer matching. Balance counts toward the savings pool. |
| 8 | `Savings` | General savings or brokerage account. Balance counts toward the savings pool. |
| 9 | `Ssa` | Social Security. Runs last so the taxable-benefit calculation (IRS-style two-tier formula over the configured breakpoints and percentages) sees all other income for the year, including retirement withdrawals. |

## Input types

### `YearInput`

Flexible year reference used for `start_in`, `end_in`, `start_out`, `end_out` on every account.

| Variant | Example JSON | Meaning |
|---|---|---|
| `ConstantInt(u32)` | `2045` | Literal calendar year. |
| `Suggested(YearSuggestion)` | `"yearRetire"` | Keyword resolved against `Settings` or linked-account dates. Options: `yearStart`, `yearRetire`, `yearDie`, `yearEnd`, `incomeLink`. `incomeLink` without a linked account falls back to the simulation bounds (with a logged warning). |
| `Calculate { base, delta }` | `{"base":"yearRetire","delta":-2}` | Keyword ± integer offset. Negative results clamp to 0 instead of wrapping. |

### `ContributionOptions`

| Variant | Behaviour |
|---|---|
| `Fixed` | Constant dollar amount in today's dollars (not inflation-adjusted). |
| `PercentOfIncome` | Percent of the linked income account's value for that year (or of total income so far this year when unlinked). |
| `FixedWithInflation` | Amount grows each year by `inflation_base` compounded from `year_start`. |

### `WithdrawalOptions`

| Variant | Behaviour |
|---|---|
| `Fixed` | Constant dollar withdrawal each year of the withdrawal window. |
| `FixedWithInflation` | Withdrawal amount grows with inflation from `year_start`. |
| `EndAtZero` | Amortizes the balance evenly to zero over the remaining withdrawal period. Amount re-computed each year from remaining balance and remaining years. |
| `ColFracOfSavings` | Withdrawal = `col × (account_balance / total_savings)` using the prior year's balances. Traditional (pre-tax) accounts gross the withdrawal up to cover the income tax due on it. Not meaningful for College accounts (their balances are outside the savings pool); the UI does not offer it there and `init` warns if a data file uses it. |

All withdrawals are capped at the account balance.

### `TaxStatus`

| Variant | Treatment |
|---|---|
| `ContributeTaxedEarningsUntaxedWhenUsed` | **Roth.** Contributions from after-tax dollars; earnings and withdrawals are non-taxable. |
| `ContributeTaxedEarningsTaxed` | **Taxed both ways.** Contributions do not reduce taxable income; earnings are taxed each year at the capital-gains rate. |
| `ContributePretaxTaxedWhenUsed` | **Traditional 401k / IRA.** Contributions reduce taxable income; withdrawals counted as taxable income. |
| `ContributePretaxUntaxedWhenUsed` | **HSA.** Contributions are pre-tax (a deduction); withdrawals are non-taxable. |

## Simulation flow

Runs once per analysis request over the full year range `[year_start, year_die]`.
Errors are reported through the crate's `Error` enum (`Data` for malformed files,
`Config` for invalid plans, `Simulation` for per-account/year failures).

### Phase 0 — Validation

`Settings::validate()` rejects configurations that would silently corrupt the run:
an empty year range, year arithmetic that overflows, and misordered SSA breakpoints
or taxable percentages.

### Phase 1 — Initialisation (before the year loop)

1. **Determine simulation order** — build a UUID list by iterating the fixed `AccountType::order()` sequence, sorted by account name within each type.
2. **Call `account.init()` for each account** — resolves linked-account dates, seeds internal tables from historical user data, and validates configuration.

### Phase 2 — Year loop

For each year `y` in `[year_start, year_die]`:

1. **Open the year** — `YearlyTotals::add_year(y, pull_forward=true)` initialises all accumulators for `y` and carries `net` forward from the most recent prior year (zero and negative balances included). All other fields start at zero.
2. **Simulate each account** — for each UUID in order: resolve `linked_value` (the linked income account's balance for year `y`, if any), call `account.simulate(y, &totals, &settings, linked_value)`, and immediately apply the returned `YearlyImpact` to `totals`. Later accounts in the same year see the updated totals.
3. **Recompute pool totals** — `totals.saving` is set to the sum of Savings + Retirement account balances for `y`; `totals.hsa` to the sum of HSA balances. Summing balances (rather than accumulating deltas) keeps the pools exact when historical table entries override computed balances mid-simulation. College balances are excluded from both pools.
4. **End-of-year settlement** — after all accounts have run:
   - Add `income` to `net`
   - Deduct `max(0, income_taxable) × tax_income + max(0, capital_gains) × tax_capital_gains` from `net`; record as `tax_burden`. Deductions can offset other income but a negative total is not a refund.
   - Deduct `expense` from `net`
   - Deduct any remaining `healthcare_expense` from `net` (zeroing the field — the HSA account already paid what it could in step 2)

## `YearlyImpact` — per account, per year

Returned by `simulate()`. All fields default to zero; each account fills only the fields it affects.

| Field | Meaning |
|---|---|
| `expense` | Cash outflow paid from `net` at end of year (loan/mortgage payments, expense amounts, and contributions into savings-type accounts). |
| `healthcare_expense` | Medical costs. HSA covers as much as possible (negative impact); remainder is charged to `net`. |
| `col` | Cost-of-living contribution. Tracks total lifestyle spend; used in the `ColFracOfSavings` withdrawal strategy. |
| `income_taxable` | Portion of income subject to the flat income-tax rate. Traditional withdrawals and taxable SSA benefits land here; pre-tax contributions subtract from it. |
| `capital_gains` | Earnings taxed at the capital-gains rate (`ContributeTaxedEarningsTaxed` accounts). |
| `income` | Total income including non-taxable amounts. Deposited into `net` at end of year. |

## `YearlyTotals` — accumulated across all accounts

A collection of year-keyed `Table<u32>` time series.

| Field | Source | Notes |
|---|---|---|
| `net` | carried forward | Liquid cash balance. Income flows in, taxes and expenses flow out at end of year. Debt (negative net) rolls forward like any other balance. |
| `saving` | recomputed each year | Sum of Retirement + Savings account balances. |
| `hsa` | recomputed each year | Sum of HSA account balances. Kept separate from `saving` because HSA funds have different tax treatment. |
| `income` / `income_taxable` / `capital_gains` | reset each year | Accumulated from account impacts. |
| `expense` / `healthcare_expense` / `col` | reset each year | Accumulated from account impacts. |
| `tax_burden` | reset each year | Written during end-of-year settlement. |
