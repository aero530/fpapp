import { ipcRenderer } from 'electron';

const math = require('mathjs'); // load math.js (using node.js)

/**
 * This function generates an object from an array
 * @function arrayToObject
 * @param {Array.<Object>} array array to convert to object
 * @param {} value value to put as the default value in the object
 * @returns {Object} object version of the array data
 */
const arrayToObject = (array, value) => array.reduce((obj, item) => {
  if (value && Array.isArray(value)) {
    obj[item] = [...value];
  } else if (value && typeof value === 'object') {
    obj[item] = { ...value };
  } else {
    obj[item] = 0;
  }
  return obj;
}, {});

/**
 * 
 * @function accountComputation
 * @description This function performs the financial simulation
 * 
 * Program flow:
 * * Loop through accounts to determine what order they should be processed in
 * * initialize tables to the correct sizes
 * * Main loop to loop through each year
 *   * Initialize this year
 *   * Loop through accounts to make contributions and withdrawals
 *     * Initialize the value of the account for this year
 *     * Calculate earnings for savings, college, retirement, hsa, and income accounts
 *     * Add earnings to the account table for the year
 *     * Calculate interest for loan and mortgage
 *     * Add interest to the account table for the year
 *     * Calculate contribution amount if account has a yearlyContribution defined
 *       * Calculate contribution amount based on contribution type (fixed_with_inflation, fixed, percent_of_income)
 *       * Calculate the employer contribution
 *     * Add contribution and employerMatch to the account table for the year
 *     * Remove contribution from taxable income for the year based on taxStatus
 *     * Calculate payment if paymentType is defined
 *       * Calculate payment amount
 *     * Add payment to the account table for the year
 *     * Calculate withdrawal if withdrawalType is defined
 *       * Calculate withdrawal amount for col_frac_of_savings, fixed, fixed_with_inflation, and end_at_zero
 *       * Limit withdrawal amount to the current value of the account (do not allow an account to become overdrawn)
 *     * Calculate expense amount
 *       * Calculate expense amount for fixed and fixed_with_inflation
 *     * Add earnings to incomeTotalTaxableTable and incomeTotalTable for the year
 *     * Remove withdrawal from the account table for the year
 *     * Add withdrawal to income table for the year (withdrawal came from another account and it added to the income tables)
 *     * Add expense to the account table for the year
 *     * Remove healthcare expenses from linked HSA account
 *     * Add entry to expense total table
 *     * Add entry to savings total
 *   * Add Income to net account (subtract out paying for income tax)
 * * Return Results
 * 
 * All data points are in that years evaluation of dollars (ie data for 2030 is in 2030 dollars)  
 * http://www.bankrate.com/calculators/retirement/retirement-plan-calculator.aspx  
 *
 * @param {Array.<Object>} accounts array of operations
 * @param settings object of settings
 * @param settings.yearStart year to start the simulation
 * @param settings.yearBorn year you were born
 * @param settings.ageRetire age when you will retire
 * @param settings.ageDie age when you will die
 * @param settings.inflationBase inflation rate (%)
 * @param settings.taxIncome income tax rate (%)
 * @param settings.taxCapitalGains capital gains tax rate (%)
 * @param settings.retirementCostOfLiving percentage applied to cost of living expenses after retirement (100% = do not reduce living expenses)
 * @param settings.ssaBreakpointsLow
 * @param settings.ssaBreakpointsHigh
 * @param settings.ssaTaxableIncomePercentageLow
 * @param settings.ssaTaxableIncomePercentageHigh
 * @returns {Array} a bunch of output data
 */
export default function accountComputation(accounts, settings) {
  const {
    yearStart,
    yearBorn,
    ageRetire,
    ageDie,
    inflationBase,
    taxIncome,
    taxCapitalGains,
    retirementCostOfLiving,
    ssaBreakpointsLow,
    ssaBreakpointsHigh,
    ssaTaxableIncomePercentageLow,
    ssaTaxableIncomePercentageHigh,
  } = settings;

  let errors = [];

  if (!yearStart) { errors.push({ title: 'Beginning Year', message: 'not defined' }); }
  if (!yearBorn) { errors.push({ title: 'Birth Year', message: 'not defined' }); }
  if (!ageRetire) { errors.push({ title: 'Retirement Age', message: 'not defined' }); }
  if (!ageDie) { errors.push({ title: 'Termination Age', message: 'not defined' }); }
  if (!inflationBase) { errors.push({ title: 'Inflation %', message: 'not defined' }); }
  if (!taxIncome) { errors.push({ title: 'Income Tax Rate %', message: 'not defined' }); }
  if (!taxCapitalGains) { errors.push({ title: 'Capital Gains Tax Rate %', message: 'not defined' }); }
  if (!retirementCostOfLiving) { errors.push({ title: 'Cost of Living %', message: 'not defined' }); }
  if (!ssaBreakpointsLow) { errors.push({ title: 'SSA Income Break Point Low', message: 'not defined' }); }
  if (!ssaBreakpointsHigh) { errors.push({ title: 'SSA Income Break Point High', message: 'not defined' }); }
  if (!ssaTaxableIncomePercentageLow) { errors.push({ title: 'SSA Taxable Income % Low', message: 'not defined' }); }
  if (!ssaTaxableIncomePercentageHigh) { errors.push({ title: 'SSA Taxable Income % High', message: 'not defined' }); }

  const ageNow = yearStart - yearBorn;
  const yearRetire = yearBorn + ageRetire;
  const yearDie = yearBorn + ageDie; // not directly used in this file but can be an input from account setup
  let yearDelta = math.range(0, ageDie - ageNow + 1);
  let yearTable = math.add(yearDelta, yearStart);

  yearDelta = yearDelta.toArray();
  yearTable = yearTable.toArray();

  const yearEnd = yearTable[yearTable.length - 1];

  const evalSuggestions = (inputString) => {
    var array = [];
    var output = 0;
    if (inputString.includes("+")) {
      array = inputString.split("+");
      if (!isNaN(array[1])) { // test for isNumeric
        output = getStringValue(array[0]) + parseFloat(array[1]);
      } else {
        output =  parseFloat(array[0]) + getStringValue(array[1]);
      }
    } else if (inputString.includes("-")) {
      array = inputString.split("-");
      if (!isNaN(array[1])) { // test for isNumeric
        output = getStringValue(array[0]) - parseFloat(array[1]);
      } else {
        output = parseFloat(array[0]) - getStringValue(array[1]);
      }
    } else {
      output = getStringValue(inputString);
    }
    return output;
  };

  // Map the string name of each value to a number
  const getStringValue = (inputString) => {
    switch(inputString) {
      case 'yearStart':
        return yearStart;
      case 'yearRetire':
        return yearRetire;
      case 'yearDie':
        return yearDie;
      case 'yearEnd':
        return yearEnd;
      case 'inflationBase':
        return inflationBase;
      default:
        return parseFloat(inputString);
    }
  };



  // ----------------------------------------------------------------------
  // Define accounts used to run for loops
  // ----------------------------------------------------------------------
  const numberAccounts = Object.keys(accounts).length; // number of accounts defined in the account object

  // ----------------------------------------------------------------------
  // Loop through accounts to determine what order they should be processed in
  // ----------------------------------------------------------------------
  const accountOrderIndex = []; // [0]*numberAccounts // make accountOrderIndex an array of the correct size
  const accountOrderTable = [
    'income',
    'ssa',
    'hsa',
    'expense',
    'mortgage',
    'loan',
    'college',
    'retirement',
    'savings',
  ]; // define the order in which accounts should be processed

  console.log('Number of accounts ', numberAccounts);
  console.log('Number of account types ', accountOrderTable.length);

  accountOrderTable.forEach((accountType) => {
    // loop through each account type in order listed in accountOrderTable
    Object.entries(accounts).forEach(([accountName, accountData]) => {
      // for each account
      if (accountData.type === accountType) {
        // check if this account matches the current account type
        accountOrderIndex.push(accountName); // add the account name to the accountOrderTable
      }
    });
  });

  // ----------------------------------------------------------------------
  // initialize tables to the correct sizes
  // ----------------------------------------------------------------------

  // make object for expense account types
  const expenseAccountsYearlyObject = {};
  Object.keys(accounts).forEach((key) => {
    if (accounts[key].type === 'expense' | accounts[key].type === 'loan' | accounts[key].type === 'mortgage' | accounts[key].type === 'college' | accounts[key].type === 'savings' | accounts[key].type === 'retirement' | accounts[key].type === 'hsa' ) {
      expenseAccountsYearlyObject[key] = 0;
    }
  });

  const expenseTotal = arrayToObject(yearTable, expenseAccountsYearlyObject);

  /*
  // This code was introduced to add historic 'table' expense data to the expenses graphs.
  // It works but the graphs are misleading because the expense graphs also show
  // loan, mortgage, college, savings, retirement, and hsa.  Since these other types use the 
  // table object to track the account value (as a savings account) not the cost/expense of
  // the account through time all these accounts show up as '0' value which makes the 
  // total expense graphs arbitrarily low for the historic data.  So this effort was abandonded.
  const expenseTotalFuture = arrayToObject(yearTable, expenseAccountsYearlyObject);

  let yearMin = 99999;
  // initialize expense table with historic data
  Object.keys(accounts).forEach((key) => {
    if (accounts[key].type === 'expense') {
      if (Object.hasOwnProperty.call(accounts[key], 'table')) { // if there is a table object
        Object.keys(accounts[key].table).forEach(year => {
          if (year < yearMin) {
            yearMin = year;
          }
        });
      }
    }
  });

  let yearDeltaPast = math.range(yearMin, yearStart);
  let yearTablePast = yearDeltaPast.toArray();
  const expenseTotalPast = arrayToObject(yearTablePast, expenseAccountsYearlyObject);

  Object.keys(accounts).forEach((key) => {
    if (accounts[key].type === 'expense') {
      if (Object.hasOwnProperty.call(accounts[key], 'table')) { // if there is a table object
        Object.keys(accounts[key].table).forEach(year => {
          if (evalSuggestions(accounts[key].startOut) <= yearStart) {
            expenseTotalPast[year][key] = accounts[key].table[year];
          }
        });
      }
    }
  });
  let expenseTotal = {...expenseTotalPast, ...expenseTotalFuture};
  */

  const savingsTotalTable = arrayToObject(yearTable, 0);
  const incomeTotalTaxableTable = arrayToObject(yearTable, 0);
  const incomeTotalTable = arrayToObject(yearTable, 0);
  const incomeTotalAfterTaxTable = arrayToObject(yearTable, 0);
  const netTable = arrayToObject(yearTable, 0);
  const incomeDuringRetirement = arrayToObject(yearTable, 0);
  // console.log(JSON.stringify(netTable));

  Object.keys(accounts).forEach((accountID) => {
    const account = accounts[accountID];
    // loop through all account objects
    // ----------------------------------------------------------------------
    // Initialize internal tables to correct size
    // ----------------------------------------------------------------------
    if (!Object.hasOwnProperty.call(account, 'table')) { // if there is not a table object then create one
      account.table = {};
      account.table[yearStart] = null;
    } else if ((typeof account.table !== 'object')) { // if the table has not been initialized with the current year
      account.table = {};
      account.table[yearStart] = null;
    } else if ((typeof account.table === 'object') && !(yearStart in account.table)) { // if the table has not been initialized with the current year
      account.table[yearStart] = null;
    }

    // ----------------------------------------------------------------------
    // Initialize the interest table in LOAN
    // ----------------------------------------------------------------------
    if (account.type === 'loan') {
      account.interest = arrayToObject(yearTable, 0);
    }

    // ----------------------------------------------------------------------
    // Initialize the interest table in MORTGAGE
    // ----------------------------------------------------------------------
    if (account.type === 'mortgage') {
      account.interest = arrayToObject(yearTable, 0);
      account.escrow = arrayToObject(yearTable, 0);
    }

    // ----------------------------------------------------------------------
    // Initialize the earnings table in SAVINGS, COLLEGE, and RETIREMENT
    // ----------------------------------------------------------------------
    if (account.type === 'savings' || account.type === 'college' || account.type === 'retirement' || account.type === 'hsa') {
      let prevData = {};
      if (Object.prototype.hasOwnProperty.call(account, 'earnings')) {
        prevData = { ...account.earnings };
      }
      account.earnings = Object.assign(arrayToObject(yearTable, 0), prevData);

      account.withdrawal = arrayToObject(yearTable, 0);
    }

    // ----------------------------------------------------------------------
    // Initialize the contribution table
    // ----------------------------------------------------------------------
    if (Object.prototype.hasOwnProperty.call(account, 'yearlyContribution')) {
      let prevData = {};
      if (Object.prototype.hasOwnProperty.call(account, 'contribution')) {
        prevData = { ...account.contribution };
      }
      account.contribution = Object.assign(arrayToObject(yearTable, 0), prevData);
    }

    // ----------------------------------------------------------------------
    // Initialize the employer match table
    // ----------------------------------------------------------------------
    if (Object.prototype.hasOwnProperty.call(account, 'employerMatch') || Object.prototype.hasOwnProperty.call(account, 'employerContribution')) {
      account.employerContributionTable = arrayToObject(yearTable, 0);
    }

    // ----------------------------------------------------------------------
    // Initialize the payment table
    // ----------------------------------------------------------------------
    if (Object.prototype.hasOwnProperty.call(account, 'paymentType')) {
      account.payment = arrayToObject(yearTable, 0);
    }

    // ----------------------------------------------------------------------
    // Initialize input values based on computation
    // ----------------------------------------------------------------------
    if (Object.prototype.hasOwnProperty.call(account, 'startIn') && typeof account.startIn !== 'number') {
      if (account.startIn === 'incomeLink') {
        account.startIn = accounts[account.incomeLink].startIn;
      } else {
        account.startIn = evalSuggestions(account.startIn);
      }
    }
    if (Object.prototype.hasOwnProperty.call(account, 'endIn') && typeof account.endIn !== 'number') {
      if (account.endIn === 'incomeLink') {
        account.endIn = accounts[account.incomeLink].endIn;
      } else {
        account.endIn = evalSuggestions(account.endIn);
      }
    }
    if (Object.prototype.hasOwnProperty.call(account, 'startOut') && typeof account.startOut !== 'number') {
      if (account.startOut === 'incomeLink') {
        account.startOut = accounts[account.incomeLink].startOut;
      } else {
        account.startOut = evalSuggestions(account.startOut);
      }
    }
    if (Object.prototype.hasOwnProperty.call(account, 'endOut') && typeof account.endOut !== 'number') {
      if (account.endOut === 'incomeLink') {
        account.endOut = accounts[account.incomeLink].endOut;
      } else {
        account.endOut = evalSuggestions(account.endOut);
      }
    }
    if (Object.prototype.hasOwnProperty.call(account, 'raise') && typeof account.raise !== 'number') {
      account.raise = evalSuggestions(account.raise);
    }
    // console.log(JSON.stringify(account));
  });

  // console.log(yearTable);

  // ----------------------------------------------------------------------
  // Main loop to loop through each year
  // ----------------------------------------------------------------------
  console.log(yearTable);
  yearTable.forEach((yearCurrent) => { // loop through all years
    console.log('START A YEAR');
    // ----------------------------------------------------------------------
    // Initialize this year
    // ----------------------------------------------------------------------
    if (yearCurrent > yearStart) {
      netTable[yearCurrent] = netTable[yearCurrent - 1]; // initialize this year as the value from last year
    }

    // ----------------------------------------------------------------------
    // Loop through accounts to make contributions and withdrawals
    // ---------------------------------------------------------------------
    accountOrderIndex.forEach((accountID) => {
      
      // ----------------------------------------------------------------------
      // Initialize temp variables to zero
      // ----------------------------------------------------------------------
      let earnings = 0; // earnings is money that an account gains (ie interest for a savings account or retirement account.  for an income account earnings is the yearly income)
      let interest = 0; // interest is money that must be payed off (ie for a loan or mortgage)
      let contribution = 0; // contribution is money that goes from income to a savings type account (savings, college, retirement, etc)
      let employerMatch = 0; // set employerMatch to zero
      let payment = 0; // payment is money that must come out of income
      let withdrawal = 0; // withdrawal is money that may be considered income (dependIng on account type)
      let expense = 0;

      let account = accounts[accountID];
      console.log('INIT '+account.name);

      // ----------------------------------------------------------------------
      // Initialize the value of the account for this year
      // ----------------------------------------------------------------------
      if (account.type === 'expense' || account.type === 'income' || account.type === 'ssa') {
        // if this is an EXPENSE or INCOME or SSA account
        account.table[yearCurrent] = 0; // previous years value does not carry over (ie not an account that carries a balance)
      } else if (account.type === 'savings' || account.type === 'college' || account.type === 'retirement' || account.type === 'hsa' || account.type === 'loan' || account.type === 'mortgage') {
        // this account type should carry over the value from last year
        const sortedKeys = Object.keys(account.table).sort((a, b) => a - b); // sort the list of keys
        let mostRecentYear = sortedKeys[0]; // default to the first year of the table
        for (var year in sortedKeys) { // go through the sorted list of keys
          if (account.table[sortedKeys[year]] !== null) { // if the value is not null (ie it was defined in the input table)
            mostRecentYear = sortedKeys[year]; // set the most recent year to this year
          } else { // if there are no longer data defined then break out of the loop
            break;
          }
        }

        // pull the most recent year data forward to the current year
        if (mostRecentYear == yearCurrent) {
          console.log('MOST CURRENT YEAR');
          console.log(mostRecentYear);
          account.table[yearCurrent] = account.table[mostRecentYear];
        } else if (Object.hasOwnProperty.call(account.table, yearCurrent - 1)) {
          console.log('YEAR MINUS ONE');
          account.table[yearCurrent] = account.table[yearCurrent - 1];
        } else if (mostRecentYear < yearCurrent) {
          console.log('LESS THAN CURRENT YEAR');
          account.table[yearCurrent] = account.table[mostRecentYear];
        } else {
          console.log('WHO KNOWS');
          account.table[yearCurrent] = 0;
        }
      } else {
        errors.push({
          title: `${account.name} ${account.type}`,
          message: 'unknown account type',
        });
      }

      // ----------------------------------------------------------------------
      // Calculate earnings
      // ----------------------------------------------------------------------
      if (account.type === 'savings' || account.type === 'college' || account.type === 'retirement' || account.type === 'hsa') {
        // if this is a SAVINGS or COLLEGE etc account
        
        earnings = (account.table[yearCurrent] * account.yearlyReturn) / 100; // calculate earnings from interest
        account.earnings[yearCurrent] = earnings; // set account earnings to current earnings value
      } else if (account.type === 'income') {
        // Otherwise if this is an INCOME account
        if (account.startIn <= yearCurrent && account.endIn >= yearCurrent) {
          // if this income object is active this year
          earnings = account.base * ((1 + account.raise / 100) ** (yearCurrent - account.startIn)); // calculate this years income
        }
        if (yearCurrent >= yearRetire) {
          incomeDuringRetirement[yearCurrent] += earnings;
        }
      } else if (account.type === 'ssa') {
        if (account.startIn <= yearCurrent && account.endIn >= yearCurrent) {
          // if this ssa object is active this year
          earnings = account.base; // calculate this years income
        }
        if (yearCurrent >= yearRetire) {
          incomeDuringRetirement[yearCurrent] += earnings;
        }
      }
      
      
      if (isNaN(earnings) || earnings < 0) {
        console.log("NAN or 0");
        console.log(account.table[yearCurrent]);
        console.log(account.yearlyReturn);
        console.log(yearCurrent);
        console.log(yearCurrent-1);
        console.log(account.table);
        console.log(' ');
        earnings = 0;
        errors.push({
          title: `${account.name} ${yearCurrent}`,
          message: 'Earnings is NAN or less than 0',
        });
      }

      // ----------------------------------------------------------------------
      // Add earnings to the account for the year
      // ----------------------------------------------------------------------
      account.table[yearCurrent] += earnings;

      // ----------------------------------------------------------------------
      // Calculate interest
      // ----------------------------------------------------------------------
      if (account.type === 'loan') {
        // if this is a LOAN account
        interest = (account.table[yearCurrent] * account.rate) / 100.0;
        account.interest[yearCurrent] = interest;
      } else if (account.type === 'mortgage') {
        // Otherwise if this is a MORTGAGE account
        if ((account.table[yearCurrent] * 100.0) / account.value > account.ltvLimit) {
          // if the current loan to value is more than the cutoff limit
          interest = account.table[yearCurrent] * ((1 + account.rate / 100.0 / account.compoundTime) ** (account.compoundTime)) + account.mortgageInsurance + account.escrowValue - account.table[yearCurrent]; // add this years interest to the mortgage then decrease mortgage by payment amount but reduce payment by the escrow and mortgage insurance values
        } else {
          // otherwise if the current loan to value is less than the cutoff limit
          interest = account.table[yearCurrent] * ((1 + account.rate / 100.0 / account.compoundTime) ** (account.compoundTime)) + account.escrowValue - account.table[yearCurrent]; // add this years interest to the mortgage at the make payment on mortgage but but reduce payment by escrow value
          account.interest[yearCurrent] = interest;
          account.escrow[yearCurrent] = account.escrowValue;
        }
      }

      // ----------------------------------------------------------------------
      // Add interest to the account for the year
      // ----------------------------------------------------------------------
      account.table[yearCurrent] += interest;


      // ----------------------------------------------------------------------
      // Calculate contribution amount
      // ----------------------------------------------------------------------
      if (Object.prototype.hasOwnProperty.call(account, 'yearlyContribution')) {
        // if there contribution values are defined
        if (account.startIn <= yearCurrent && account.endIn >= yearCurrent) {
          // if making contribution this year
          // ----------------------------------------------------------------------
          // Calculate contribution amount based on contribution type
          // ----------------------------------------------------------------------

          if (account.contributionType === 'fixed_with_inflation') {
            // if inflation needs to be accounted for in the contribution
            contribution = account.yearlyContribution * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // increase the value by inflation
          } else if (account.contributionType === 'fixed') {
            // otherwise if the contribution is a fixed value
            contribution = account.yearlyContribution; // set the contribution amount to the value input
          } else if (account.contributionType === 'percent_of_income') {
            // otherwise if the contribution is a percent of income
            if (Object.prototype.hasOwnProperty.call(account, 'incomeLink') && account.incomeLink.length > 0) {
              // and if the account has an income object linked to it
              contribution = accounts[account.incomeLink].table[yearCurrent] * (account.yearlyContribution / 100); // calculate the contribution value using that income account and the percentage input
            } else {
              // otherwise
              contribution = incomeTotalTaxableTable[yearCurrent] * (account.yearlyContribution / 100); // calculate the contribution using the total income for the year
            }
          } else {
            errors.push({
              title: `${account.name} ${account.contributionType} ${yearCurrent}`,
              message: 'unknown contribution type',
            });
          }

          account.contribution[yearCurrent] = contribution; // set account contribution value to current contribution value

          if (contribution < 0) {
            console.log('Error contribution < 0');
            errors.push({
              title: `${account.name} ${yearCurrent}`,
              message: 'contribution < 0',
            });
          }

          // ----------------------------------------------------------------------
          // Calculate the employer contribution
          // ----------------------------------------------------------------------

          if (Object.prototype.hasOwnProperty.call(account, 'incomeLink') && Object.prototype.hasOwnProperty.call(account, 'employerMatch') && Object.prototype.hasOwnProperty.call(account, 'matchLimit')) {
            if (account.incomeLink.length >= 0) {
              // if there is an incomeLink for this account
              if (typeof account.employerMatch === 'number') {
                // if employerMatch is not a list
                // let tmp = ;
                account.employerMatch = [account.employerMatch];
              }
              if (typeof account.matchLimit === 'number') {
                // if matchLimit is not a list
                // let tmp = ;
                account.matchLimit = [account.matchLimit];
              }

              if (account.employerMatch[0] >= 0 && account.matchLimit[0] >= 0) {
                if (account.matchLimit.length > 1) {
                  // and if it is a complex employer matching (more than one level)
                  if (contribution >= (account.matchLimit[0] / 100 + account.matchLimit[1] / 100) * accounts[account.incomeLink].table[yearCurrent]) {
                    // and if the contribution is above the highest employer matching level
                    employerMatch = accounts[account.incomeLink].table[yearCurrent] * ((account.employerMatch[1] / 100) * (account.matchLimit[1] / 100) + (account.employerMatch[0] / 100) * (account.matchLimit[0] / 100)); // calculate the employer matching based on the match limits
                  } else if (contribution >= (account.matchLimit[0] / 100) * accounts[account.incomeLink].table[yearCurrent]) {
                    // otherwise if the contribution is between the employer matching levels ) {
                    employerMatch = accounts[account.incomeLink].table[yearCurrent] * ((account.employerMatch[0] / 100) * (account.matchLimit[0] / 100) + (account.employerMatch[1] / 100) * (account.matchLimit[1] / 100) * (contribution / accounts[account.incomeLink].table[yearCurrent] - account.matchLimit[0] / 100)); // calculate the employer matching with all the first level and part of the second level
                  } else {
                    employerMatch = contribution * (account.employerMatch[0] / 100); // the employer contribution is computed based on the entire contribution
                  }
                } else {
                  // if it is a simple employer matching (only one level)
                  if (contribution >= account.matchLimit[0] * accounts[account.incomeLink].table[yearCurrent]) {
                    // and if the contribution is above the highest employer matching level
                    employerMatch = accounts[account.incomeLink].table[yearCurrent] * (account.employerMatch[0] / 100) * (account.matchLimit[0] / 100); // calculate the employer matching based on the match limits
                  } else {
                    // otherwise  if below the employer match limit
                    employerMatch = contribution * (account.employerMatch[0] / 100); // the employer contribution is computed based on the entire contribution
                  }
                }
              }
              account.employerContributionTable[yearCurrent] = employerMatch;
            } else {
              console.log('Employer Match defined for account but incomeLink length <= 0 ', account.name);
              errors.push({ title: `${account.name} ${yearCurrent}`, message: 'employer match defined for account but incomeLink length <= 0' });
            }
          } else if (Object.prototype.hasOwnProperty.call(account, 'employerContribution') && yearCurrent <= yearRetire) {
            if (account.contributionType === 'fixed_with_inflation') {
              // if inflation needs to be accounted for in the contribution
              employerMatch = account.employerContribution * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // increase the value by inflation
            } else if (account.contributionType === 'fixed') {
              // otherwise if the contribution is a fixed value
              employerMatch = account.employerContribution; // set the contribution amount to the value input
            } else {
              console.log('Employer Contribution type not implemented');
              errors.push({ title: `${account.name} ${yearCurrent}`, message: 'employer contribution type not implemented' });
            }
            account.employerContributionTable[yearCurrent] = employerMatch;
          }
        }

        // ----------------------------------------------------------------------
        // Add contribution and employerMatch to the account for the year
        // ----------------------------------------------------------------------
        account.table[yearCurrent] = account.table[yearCurrent] + contribution + employerMatch;
      }

      // ----------------------------------------------------------------------
      // Calculate payment
      // ----------------------------------------------------------------------

      if (Object.prototype.hasOwnProperty.call(account, 'paymentType')) {
        // if there is a payment defined
        if (account.startOut <= yearCurrent && account.endOut >= yearCurrent) {
          // if making a payment this year
          // ----------------------------------------------------------------------
          // Calculate payment amount
          // ----------------------------------------------------------------------
          if (account.paymentType === 'fixed') {
            // otherwise if type is a fixed value
            payment = account.paymentValue; // set withdrawal to the value
          } else if (account.paymentType === 'fixed_with_inflation') {
            // otherwise if type is a fixed number but should be compensated for with inflation
            payment = account.paymentValue * ((1 + inflationBase / 100) ** (yearCurrent - account.startOut)); // set withdrawal to the value multiplied by an increase due to inflation
          } else {
            // otherwise if a different type is specified
            payment = 0; // set withdrawal to zero (this is for accounts that you dont remove money from such as expense accounts)
            errors.push({ title: `${account.name} ${yearCurrent}`, message: 'unknown paymentType' });
          }
          if (payment > account.table[yearCurrent]) {
            payment = account.table[yearCurrent];
          }
          account.payment[yearCurrent] = payment; // add payment to payment table
        }
        if (payment < 0) {
          console.log('Error payment < 0');
          errors.push({ title: `${account.name} ${yearCurrent}`, message: 'payment < 0' });
        }
      }

      // ----------------------------------------------------------------------
      // Add payment to the account for the year
      // ----------------------------------------------------------------------
      account.table[yearCurrent] -= payment;


      // ----------------------------------------------------------------------
      // Calculate withdrawal
      // ----------------------------------------------------------------------
      if (Object.prototype.hasOwnProperty.call(account, 'withdrawalType')) {
        if (account.startOut <= yearCurrent && account.endOut >= yearCurrent) {
          // if taking money out this year

          // ----------------------------------------------------------------------
          // Calculate withdrawal amount
          // ----------------------------------------------------------------------
          if (account.withdrawalType === 'col_frac_of_savings') {
            // otherwise if type is cost of living fraction of total savings
            if (yearCurrent > yearStart) {
              // withdrawal = costOfLiving['table'][yearIndex] * account[accountIndex].table(yearIndex-1)./savingsTotal.table(yearIndex-1)
              // account for retirement cost of living and for capital gains in this line...its a hack and probably not very correct
              if (account.table[yearCurrent - 1] > 0) {
                // if there is money left in the account
                // withdrawal from this account = total expenses this year  * fraction of total savings this account represents
                // total expenses this year is reduced by the income during retirement for the year.
                // incomeDuringRetirement is tracked because withdrawals from retirement accounts go into the income table but we want to
                // pay for expenses from money earned in this year before pulling from retirement accounts.
                const totalExpensesThisYear = Object.values(expenseTotal[yearCurrent]).reduce((acc, cur) => acc + cur, 0) - incomeDuringRetirement[yearCurrent];
                withdrawal = (totalExpensesThisYear * account.table[yearCurrent - 1]) / savingsTotalTable[yearCurrent - 1];
                if (Object.prototype.hasOwnProperty.call(account, 'taxStatus') && account.taxStatus === 3) {
                  withdrawal *= (taxIncome / 100 + 1); // add extra to amount withdrawal value to account for taxes.
                }
              }
            } else {
              console.log('ERROR - Can not compute withdrawal amount');
              errors.push({ title: `${account.name} ${yearCurrent}`, message: 'can not compute withdrawal amount < 0' });
            }
          } else if (account.withdrawalType === 'fixed') {
            // otherwise if type is a fixed value
            withdrawal = account.withdrawalValue; // set withdrawal to the value
          } else if (account.withdrawalType === 'fixed_with_inflation') {
            // otherwise if type is a fixed number but should be compensated for with inflation
            withdrawal = account.withdrawalValue * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // set withdrawal to the value multiplied by an increase due to inflation
          } else if (account.withdrawalType === 'end_at_zero') {
            // otherwise if the type is end at zero
            if (account.endOut >= yearCurrent) {
              // and if the year to stop taking money out of the account is beyond or equal to the current year
              withdrawal = account.table[yearCurrent] / (account.endOut - yearCurrent + 1); // calculate the fraction of the account balance to withdraw
            }
          } else {
            // otherwise if a different type is specified
            console.log('Invalid withdrawal type');
            errors.push({ title: `${account.name} ${account.withdrawalType} ${yearCurrent}`, message: 'withdrawalType unknown' });
            withdrawal = 0; // set withdrawal to zero (this is for accounts that you dont remove money from such as expense accounts)
          }
        }

        if (withdrawal > account.table[yearCurrent]) {
          // dont allow an account to become overdrawn
          withdrawal = account.table[yearCurrent];
        }
        if (withdrawal < 0) {
          console.log('Error withdrawal < 0');
          errors.push({ title: `${account.name} ${yearCurrent}`, message: 'withdrawal < 0' });
        }
        account.withdrawal[yearCurrent] = withdrawal;
      }

      // ----------------------------------------------------------------------
      // Calculate expense amount
      // ----------------------------------------------------------------------
      if (account.type === 'expense') {
        // if there is a expense type defined
        if (account.startOut <= yearCurrent && account.endOut >= yearCurrent) {
          // if this expense applies this year

          // ----------------------------------------------------------------------
          // Calculate expense amount for fixed, fixed_with_inflation
          // ----------------------------------------------------------------------
          if (account.expenseType === 'fixed') {
            // otherwise if type is a fixed value
            expense = account.expenseValue; // set expense to the value
          } else if (account.expenseType === 'fixed_with_inflation') {
            // otherwise if type is a fixed number but should be compensated for with inflation
            expense = account.expenseValue * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // set expense to the value multiplied by an increase due to inflation
          } else {
            // otherwise if a different type is specified
            expense = 0; // set expense to zero (this is for accounts that you dont remove money from such as expense accounts)
            errors.push({ title: `${account.name} ${account.expenseType} ${yearCurrent}`, message: 'unknown expense type' });
          }
          if (yearCurrent >= yearRetire) {
            expense *= retirementCostOfLiving / 100;
          }
        }
        if (expense < 0) {
          console.log('Error expense < 0');
          errors.push({ title: `${account.name} ${yearCurrent}`, message: 'expense < 0' });
        }
      }

      // ----------------------------------------------------------------------
      // Remove contribution from taxable income for the year based on taxStatus
      // ----------------------------------------------------------------------
      if (Object.prototype.hasOwnProperty.call(account, 'taxStatus') && (account.taxStatus === 3 || account.taxStatus === 4)) {
        // if contributions should be taken out of taxable income for the year
        // contribute pretax income
        incomeTotalTaxableTable[yearCurrent] -= contribution; // take the contribution value out of taxable income for the year
      }

      // ----------------------------------------------------------------------
      // Add earnings to incomeTotalTaxableTable and incomeTotalTable for the year
      // ----------------------------------------------------------------------
      if (account.type === 'income') {
        incomeTotalTaxableTable[yearCurrent] += account.table[yearCurrent]; // increase this years taxable income by the withdrawal amount
        incomeTotalTable[yearCurrent] += account.table[yearCurrent]; // increase this years income by the withdrawal amount
      }
      if (account.type === 'ssa') {
        if ((incomeTotalTaxableTable[yearCurrent]+account.table[yearCurrent]) > ssaBreakpointsHigh) {
          incomeTotalTaxableTable[yearCurrent] += (account.table[yearCurrent] * settings.ssaTaxableIncomePercentageHigh / 100);
          incomeTotalTable[yearCurrent] += account.table[yearCurrent]; // increase this years income by the withdrawal amount
        } else if ((incomeTotalTaxableTable[yearCurrent]+account.table[yearCurrent]) > ssaBreakpointsLow) {
          incomeTotalTaxableTable[yearCurrent] += (account.table[yearCurrent] * settings.ssaTaxableIncomePercentageLow / 100);
          incomeTotalTable[yearCurrent] += account.table[yearCurrent]; // increase this years income by the withdrawal amount
        }
        
        
      }

      // ----------------------------------------------------------------------
      // Add capital gains earnings to taxable income
      // ----------------------------------------------------------------------
      //        if 'taxStatus' in account[accountIndex] and account[accountIndex]['taxStatus']==1 : payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed
      //            incomeTotalTaxable_table[yearIndex] = incomeTotalTaxable_table[yearIndex] + earnings * tax_capitalGains/100  increase this years taxable income by the capital gains for this account


      // ----------------------------------------------------------------------
      // Remove withdrawal from the account table for the year
      // ----------------------------------------------------------------------
      account.table[yearCurrent] -= withdrawal;

      // ----------------------------------------------------------------------
      // Add withdrawal to income table for the year (withdrawal came from another account and it added to the income tables)
      // ----------------------------------------------------------------------
      if (account.type !== 'college') {
        // dont put college withdrawals into income because they go to kids not me
        if (Object.prototype.hasOwnProperty.call(account, 'taxStatus') && account.taxStatus === 3) {
          // if the withdrawal should be counted as taxable income for the year
          incomeTotalTaxableTable[yearCurrent] += withdrawal; // increase this years taxable income by the withdrawal amount
          incomeTotalTable[yearCurrent] += withdrawal; // increase this years income by the withdrawal amount
        } else {
          incomeTotalTable[yearCurrent] += withdrawal; // increase this years income by the withdrawal amount
        }
      }

      // ----------------------------------------------------------------------
      // Add expense to the account table for the year
      // ----------------------------------------------------------------------
      account.table[yearCurrent] += expense;

      // ----------------------------------------------------------------------
      // Remove healthcare expenses from linked HSA account
      // ----------------------------------------------------------------------
      if (Object.prototype.hasOwnProperty.call(account, 'isHealthcare') && account.isHealthcare) {
        // pull from HSA
        if (Object.prototype.hasOwnProperty.call(account, 'hsaLink') && account.hsaLink !== 'none') {
          // if there is an HSA account linked
          // print('{0:s} {1:.2f} {2:.2f}'.format(account[account[accountIndex]['hsaLink']]['name'],account[account[accountIndex]['hsaLink']]['table'][yearCurrent], expense))
          if (accounts[account.hsaLink].table[yearCurrent] >= expense) {
            // if there is enough money in the HSA savings account to pay for healthcare expenses
            accounts[account.hsaLink].table[yearCurrent] -= expense;
            accounts[account.hsaLink].withdrawal[yearCurrent] += expense;

            // calculating expense accounts happens after calculating hsa accounts for the year so the withdrawal made must also be applied to income.
            if (accounts[account.hsaLink].taxStatus === 4) {
              incomeTotalTable[yearCurrent] += expense; // hsa is probably a taxStatus = 4 account
            } else if (accounts[account.hsaLink].taxStatus === 3) {
              incomeTotalTable[yearCurrent] += expense;
              incomeTotalTaxableTable[yearCurrent] += expense;
              errors.push({ title: `${account.name} ${yearCurrent}`, message: 'HSA is set with tax status === contribute pretax income - taxed as income when used' });
            }
          } else {
            // otherwise drain the HSA account then reset expense to represent the remaining balance of the expense
            const tmpCurrentVal = accounts[account.hsaLink].table[yearCurrent]; // only this much of the expense can be covered by the hsa
            accounts[account.hsaLink].withdrawal[yearCurrent] = tmpCurrentVal;

            // calculating expense accounts happens after calculating hsa accounts for the year so the withdrawal made must also be applied to income.
            if (accounts[account.hsaLink].taxStatus === 4) {
              incomeTotalTable[yearCurrent] += expense; // hsa is probably a taxStatus = 4 account
            } else if (accounts[account.hsaLink].taxStatus === 3) {
              incomeTotalTable[yearCurrent] += expense;
              incomeTotalTaxableTable[yearCurrent] += expense;
              errors.push({ title: `${account.name} ${yearCurrent}`, message: 'HSA is set with tax status === contribute pretax income - taxed as income when used' });
            }

            accounts[account.hsaLink].table[yearCurrent] = 0;
          }
        } else {
          console.log('Account is healthcare but does not have HSA link');
          errors.push({ title: `${account.name} ${yearCurrent}`, message: 'account is healthcare but does not have HSA link' });
        }
      }

      // personal healthcare expenses do not show up in expenses graph because those are covered by the hsa account


      // ----------------------------------------------------------------------
      // Add entry to expense total table
      // ----------------------------------------------------------------------
      if (account.type === 'expense') {
        // and if type is EXPENSE
        expenseTotal[yearCurrent][accountID] = expense; // add withdrawal to the expense table
      } else if (account.type === 'loan') {
        // otherwise if type is LOAN
        expenseTotal[yearCurrent][accountID] = payment; // add withdrawal to the expense table
      } else if (account.type === 'mortgage') {
        // otherwise if type is mortgage
        expenseTotal[yearCurrent][accountID] = payment; // add withdrawal to the expense table
      } else if (account.type === 'college') {
        // otherwise if type is college
        expenseTotal[yearCurrent][accountID] = contribution; // add contribution to the expense table
      } else if (account.type === 'savings') {
        // otherwise if type is a SAVINGS account
        expenseTotal[yearCurrent][accountID] = contribution; // add contribution to the expense table
      } else if (account.type === 'retirement') {
        // otherwise if type is a SAVINGS account
        expenseTotal[yearCurrent][accountID] = contribution; // add contribution to the expense table
      } else if (account.type === 'hsa') {
        // otherwise if type is a HSA account
        expenseTotal[yearCurrent][accountID] = contribution; // add contribution to the expense table
      }

      // ----------------------------------------------------------------------
      // Add entry to savings total
      // ----------------------------------------------------------------------
      if (account.type === 'savings' || account.type === 'retirement') {
        // if this is a savings account
        savingsTotalTable[yearCurrent] += account.table[yearCurrent];
      }
    }); // end for each account loop

    // ----------------------------------------------------------------------
    // Add Income to net account (subtract out paying for income tax)
    // ----------------------------------------------------------------------
    const totalExpensesThisYear = Object.values(expenseTotal[yearCurrent]).reduce((acc, cur) => acc + cur, 0);
    // console.log(totalExpensesThisYear);

    netTable[yearCurrent] = netTable[yearCurrent] + incomeTotalTable[yearCurrent] - incomeTotalTaxableTable[yearCurrent] * (taxIncome / 100) - totalExpensesThisYear;
    incomeTotalAfterTaxTable[yearCurrent] = incomeTotalTable[yearCurrent] - incomeTotalTaxableTable[yearCurrent] * (taxIncome / 100);
  }); // end for each year loop

  ipcRenderer.send('analysisErrors', errors);

  // ----------------------------------------------------------------------
  // Return Results
  // ----------------------------------------------------------------------
  const result = {
    year: yearTable,
    savings: savingsTotalTable,
    expenses: expenseTotal,
    incomeTaxable: incomeTotalTaxableTable,
    incomeTotal: incomeTotalTable,
    incomeAfterTax: incomeTotalAfterTaxTable,
    net: netTable,
    accounts,
  };
  return result;
}
