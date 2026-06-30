# financial planning app #

A financial planning & simulation application.


![screenshot_loan](https://github.com/aero530/fpapp/raw/main/egui/screenshot.png "Retirement")

## Features ##

- Simulate income and expenses through retirement
- Track historic account balances
- Support multiple account types
  - Income
  - Retirement (IRA, Roth IRA, 401K)
  - Social Security
  - College Savings (529)
  - Expenses (such as grocery, car, utilities, insurance, entertainment, rent, etc.)
  - Loans (student, car, etc.)
  - Mortgage
  - Savings
  - Health Savings Account (HSA)
- Make pretty graphs
- Financial data saved locally as human readable json file


## Computation Flow ##

- Build simulation order: collect UUIDs grouped by AccountType in fixed sequence
  (Income → SSA → Expense → HSA → Mortgage → Loan → College → Retirement → Savings)
- For each account, call init(linked_dates, settings):
    - Seeds internal year tables from historical user data
    - Returns pre-existing (year, impact) pairs; apply these to YearlyTotals
      to set opening balances before the simulation loop
- Main loop over each year y in [year_start, year_die]:
  - Open year: initialize all accumulators; carry net, saving, hsa forward from y-1
  - For each account (in type order):
    - Resolve linked_value (income balance of the linked income account, if any)
    - Call account.simulate(y, &totals, &settings, linked_value) → YearlyImpact
      (all internal logic — earnings, contributions, withdrawals, payments,
       expense amounts, tax treatment — is encapsulated here)
    - Accumulate YearlyImpact into YearlyTotals immediately
      (later accounts in the same year see the updated totals)
  - End-of-year settlement:
    - Add income → net
    - Deduct income_taxable × tax_rate from net; record as tax_burden
    - Deduct expense from net
    - Deduct remaining healthcare_expense from net (zero it out;
      HSA already covered what it could during its simulate() call)
- Collect per-account plot data
- Return (plot_data, yearly_totals)


## To do

- [ ] Improve tests to verify calculations for all account types


## Revision History ##

### v0.0.1 - 8.3.12 ###

- Initial development in Octave

### v0.0.2 - 8.27.12 ###

- Convert to SciLab.

### v0.0.3 - 9.1.12 ###

- Update input numbers

### v0.0.4 - 12.22.12 ###

- Update input numbers

### v0.0.5 - 10.27.13 ###

- Update input numbers - <http://money.msn.com/retirement/retirement-calculator.aspx>

### v0.1.0 - 12.30.13 ###

- Convert to Python

### v0.1.1 - 6.1.14 ###

- Update input numbers

### v0.1.2 - 12.7.14 ###

- Update input numbers

### v0.1.3 - 12.1.15 ###

- Update input numbers

### v0.1.4 - 3.10.17 ###

- Update input numbers

### v1.0.0 - 10.5.18 ###

- Convert to JS / electron
- Save user data as json instead of at the beginning of the code file
- Release v1.0.0

### v1.0.1 - 10.9.18 ###

- Added social security account type

### v1.0.2 - 10.10.18 ###

- Update with new theme

### v1.0.3 - 11.14.18 ###

- Update to electron 3.0

### v1.0.4 - 12.29.18 ###

- Update to babel 7
- Migrate from 2 package.json to single package.json
- Replace react-router-redux with connected-react-router
- Remove unused dependencies

### v2.0.0 - 10.3.19 ###

- Change to new project template
- Update dependencies
- Fix calculation bugs
- Add social security income source

### v2.1.0 - 10.17.19 ###

- Add file-new
- Refresh pages on file-open or file-new

### v2.1.1 - 12.9.19 ###

- Fix data type storage bug from MUI Editable table fields

### v3.0.0 - ________ ###

- Convert to Rust & Tauri

### v4.0.0 - ________ ###

- Update to Tauri 2.0

### v5.0.0 - 6/30/26 ###

- Convert to egui
- Leave depricated tauri version in place for now