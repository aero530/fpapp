# Application View Window

## Folder Structure

**actions** actions  
**components** Visual components  
**constants** Constant definitions  
**reducers** redux reducers  
**routes** Main webpage structure  
**store** Redux store configuration  
**utils** Reusable utility functions  
**index.js** Main react app - ipc action routing, redux store, and history configuration  
**root.jsx** apply material ui theme to app

## Redux Store
The redux store maintains four primary objects:  

* **app** info about application state
  * *appBarTitle* current string that should be shown in titlebar
* **data** input data
  * *settings* input settings
    * *ageRetire* age you plan to retire
    * *ageDie* age you plan to die
    * *yearBorn* year you were born
    * *yearStart* year to start the simulation
    * *inflationBase* inflation rate
    * *taxIncome* income tax rate
    * *taxCapitalGains* capital gains tax rate
    * *retirementCostOfLiving* % of cost of living expenses to assume during retirement
  * *accounts* object of all input accounts
  * *incomeAccounts* computed lookup array of income accounts
  * *hsaAccounts* computed lookup array of hsa accounts
  * *filename* string full path of the opened file
  * *modified* bool if the input data has been modified since last open or save
* **results** simulation results
  * *accounts* object of accounts
  * *savings* table of total savings
* **router** react router location
