# Computation Engine #

## Folder Structure ##

**index.html** Background processing entry page 
**background.js** listens for ipc call to run computation  
**accountComputation.js** performs computation / financial simulation  
**fpcalc.py** original python based simulation  

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
