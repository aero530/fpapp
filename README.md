# financial planning app #

A financial planning & simulation application.

---

![screenshot_graphs](https://github.com/aero530/fpapp/raw/main/resources/screenshots/graphs.png "Graph")

![screenshot_loan](https://github.com/aero530/fpapp/raw/main/resources/screenshots/loan.png "Loan")

## Features ##

* Simulate income and expenses through retirement
* Track historic account balances
* Support multiple account types
  * Income
  * Retirement (IRA, Roth IRA, 401K)
  * Social Security
  * College Savings (529)
  * Expenses (such as grocery, car, utilities, insurance, entertainment, rent, etc.)
  * Loans (student, car, etc.)
  * Mortgage
  * Savings
  * Health Savings Account (HSA)
* Make pretty graphs
* Financial data saved locally as human readable json file

---

## Computation Flow ##

* Loop through accounts to determine what order they should be processed in
* initialize tables to the correct sizes
* Main loop to loop through each year
  * Initialize this year
  * Loop through accounts to make contributions and withdrawals
    * Initialize the value of the account for this year
    * Calculate earnings for savings, college, retirement, hsa, and income accounts
    * Add earnings to the account table for the year
    * Calculate interest for loan and mortgage
    * Add interest to the account table for the year
    * Calculate contribution amount if account has a yearlyContribution defined
      * Calculate contribution amount based on contribution type (fixed_with_inflation, fixed, percent_of_income)
      * Calculate the employer contribution
    * Add contribution and employerMatch to the account table for the year
    * Remove contribution from taxable income for the year based on taxStatus
    * Calculate payment if paymentType is defined
      * Calculate payment amount
    * Add payment to the account table for the year
    * Calculate withdrawal if withdrawalType is defined
      * Calculate withdrawal amount for col_frac_of_savings, fixed, fixed_with_inflation, and end_at_zero
      * Limit withdrawal amount to the current value of the account (do not allow an account to become overdrawn)
    * Calculate expense amount
      * Calculate expense amount for fixed and fixed_with_inflation
    * Add earnings to incomeTotalTaxableTable and incomeTotalTable for the year
    * Remove withdrawal from the account table for the year
    * Add withdrawal to income table for the year (withdrawal came from another account and it added to the income tables)
    * Add expense to the account table for the year
    * Remove healthcare expenses from linked HSA account
    * Add entry to expense total table
    * Add entry to savings total
  * Add Income to net account (subtract out paying for income tax)
* Return Results

---

## Development Setup ##

### Clone the repo via git ###

```cmd
git clone https://github.com/aero530/fpapp.git fpapp
```

## Packaging ##

Create a package for macOS, Windows, or Linux using one of the following commands:

```cmd
> cargo build --release
```

## Tests ##

```cmd
> cargo test
```

## Revision History ##

### v0.0.1 - 9.1.12 ###

Initial development and numbers done in Python.

### v0.0.2 - 12.22.12 ###

Update input numbers and set plan/budget for 2013. Reduced spending in retirement for retail and car.  Updated mortgate info with new loan amount.

### v0.0.3 - 10.27.13 ###

Update input numbers - http://money.msn.com/retirement/retirement-calculator.aspx

### v0.0.4 - 12.27.14 ###

Update input numbers

### v0.0.5 - 3/10/2017 ###

Update input numbers

### v1.0.0 - 10/5/2018 ###

Convert to JS / electron and release v1.0.0

### v1.0.1 - 10/9/2018 ###

Added social security account type

### v1.0.2 - 10/10/2018 ###

Update with new theme

### v1.0.3 - 11/14/2018 ###

Update to electron 3.0

### v1.0.4 - 12/29/2018 ###

* Update to babel 7
* Migrate from 2 package.json to single package.json
* Replace react-router-redux with connected-react-router
* Remove unused dependencies

### v2.0.0 - 10/3/2019 ###

Change to new project template. Update dependencies. Fix calculation bugs. Add social security income source.

### v2.1.0 - 10/17/2019 ###

Add file-new, refresh pages on file-open or file-new

### v2.1.1 - 12/9/2019 ###

Fix data type storage bug from MUI Editable table fields.

### v3.0.0 - ________ ###

Convert to Rust
