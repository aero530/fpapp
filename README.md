# financial planning app #

A financial planning & simulation application.

---

To do:

- [ ] Cleanup / improve look & style of layout
- [ ] Improve tests to verify calculations for all account types
- [ ] Tooltips fall off edge of window on Settings page.  This is also why the Settings page ends up with a bottom scroll bar.
- [x] Chart tooltip circle should match the color of the line
- [x] Remove chart lines / legend value for types that are always zero / not used (such as employer contributions on college savings accounts). This goes back to how plot data is generated in `table_groups.rs`.  All `savings` account types use the same simulation result table type which has optional employer_match.  When plot data is generated if that option is `None` default values of `0` are inserted.
- [x] Add menu link for Loans
- [x] Add use tips to dashboard page if there is no data loaded
- [x] Add employer matching to retirement account UI
- [x] Add account link to retirement employer match
- [x] Update chart domain on data change
- [x] Make charts update on data change
- [x] Convert backend from JS to Rust
- [x] Convert UI from Electron to Tauri
- [x] Convert web front end from React to Svelte
- [X] Convert charts to D3
- [X] Generate overall visualizations / charts for dashboard

---

![screenshot_loan](https://github.com/aero530/fpapp/raw/main/resources/screenshots/retirement.png "Retirement")

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

---

## Computation Flow ##

- Loop through accounts to determine what order they should be processed in
- initialize tables to the correct sizes
- Main loop to loop through each year
  - Initialize this year
  - Loop through accounts to make contributions and withdrawals
    - Initialize the value of the account for this year
    - Calculate earnings for savings, college, retirement, hsa, and income accounts
    - Add earnings to the account table for the year
    - Calculate interest for loan and mortgage
    - Add interest to the account table for the year
    - Calculate contribution amount if account has a yearlyContribution defined
      - Calculate contribution amount based on contribution type (fixed_with_inflation, fixed, percent_of_income)
      - Calculate the employer contribution
    - Add contribution and employerMatch to the account table for the year
    - Remove contribution from taxable income for the year based on taxStatus
    - Calculate payment if paymentType is defined
      - Calculate payment amount
    - Add payment to the account table for the year
    - Calculate withdrawal if withdrawalType is defined
      - Calculate withdrawal amount for col_frac_of_savings, fixed, fixed_with_inflation, and end_at_zero
      - Limit withdrawal amount to the current value of the account (do not allow an account to become overdrawn)
    - Calculate expense amount
      - Calculate expense amount for fixed and fixed_with_inflation
    - Add earnings to incomeTotalTaxableTable and incomeTotalTable for the year
    - Remove withdrawal from the account table for the year
    - Add withdrawal to income table for the year (withdrawal came from another account and it added to the income tables)
    - Add expense to the account table for the year
    - Remove healthcare expenses from linked HSA account
    - Add entry to expense total table
    - Add entry to savings total
  - Add Income to net account (subtract out paying for income tax)
- Return Results

---

## Development Setup ##

### Clone the repo via git ###

```cmd
git clone https://github.com/aero530/fpapp.git fpapp
```

## Update TypeScript Bindings ##

The accounts rust module uses ts-rs to automatically create TS bindings for use in the UI. Currently these
need to manually generated if the accounts module changes.

```cmd
> cd src-tauri/src/accounts; cargo test; cd ../../../
```

## Dev ##

Start app in dev mode:

```cmd
> npm run tauri dev
```

## Packaging ##

Create a package for macOS, Windows, or Linux using one of the following commands:

```cmd
> npm run tauri build
```

<!-- ```cmd
> cargo build --release
``` -->

## Tests ##

```cmd
> cargo test
```

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
