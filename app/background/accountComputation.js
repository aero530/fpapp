import { ipcRenderer } from 'electron';

const math = require('mathjs'); // load math.js (using node.js)

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
  } = settings;

  const ageNow = yearStart - yearBorn;
  const yearRetire = yearBorn + ageRetire;
  const yearDie = yearBorn + ageDie;
  let yearDelta = math.range(0, ageDie - ageNow + 1);
  let yearTable = math.add(yearDelta, yearStart);

  yearDelta = yearDelta.toArray();
  yearTable = yearTable.toArray();

  // # ======================================================================
  // # Main Program
  // #
  // # net_table[yearCurrent] : running sum of un allocated money after income tax (ie checking account)
  // # expensetotal.table(yearCurrent,accountindex) : initialize table for expenses
  // # incometotaltaxable['table'][yearCurrent] : total of all the running incomes
  // # ======================================================================

  // # Define counts used to run for loops
  const numberAccounts = Object.keys(accounts).length; // number of accounts defined in the account object
  const numberYears = yearDelta.length; // number of years to run the simulation

  // # ----------------------------------------------------------------------
  // # Loop through accounts to determine what order they should be processed in
  // # ----------------------------------------------------------------------
  const accountOrderIndex = []; // [0]*numberAccounts // make accountOrderIndex an array of the correct size
  const accountOrderTable = [
    'income',
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

  // # ----------------------------------------------------------------------
  // # initialize tables to the correct sizes
  // # ----------------------------------------------------------------------
  const emptyYearTable = Array.from(yearDelta, () => 0);
  const accountsArray = Object.keys(accounts);
  const accountsYearlyObject = arrayToObject(accountsArray, 0);

  // const expenseTotal = Array.from(yearDelta, () => {
  //   return { ...accountsYearlyObject };
  // });
  const expenseTotal = arrayToObject(yearTable, accountsYearlyObject);

  const savingsTotalTable = arrayToObject(yearTable, 0);
  const incomeTotalTaxableTable = arrayToObject(yearTable, 0);
  const incomeTotalTable = arrayToObject(yearTable, 0);
  const incomeTotalAfterTaxTable = arrayToObject(yearTable, 0);
  const netTable = arrayToObject(yearTable, 0);

  Object.keys(accounts).forEach((accountName) => {
    const account = accounts[accountName];
    // Object.entries(accounts).forEach(([accountName, account]) => {
    // loop through all account objects
    // # ----------------------------------------------------------------------
    // # Initialize internal tables to correct size
    // # ----------------------------------------------------------------------
    let tmp = 0;
    if (Object.hasOwnProperty.call(account, 'table')) { // if there is already a table object
      if (Object.hasOwnProperty.call(account.table, yearStart)) {
        // if the table of the account has a value for the defined yearStart
        tmp = account.table[yearStart]; // use that value for the tables value for this year
      }
    } else { // if there is not a table object then create one
      account.table = {};
    }
    account.table[yearStart] = tmp;

    // # ----------------------------------------------------------------------
    // # Initialize the interest table in LOAN
    // # ----------------------------------------------------------------------
    if (account.type === 'loan') {
      account.interest = arrayToObject(yearTable, 0);
    }

    // # ----------------------------------------------------------------------
    // # Initialize the interest table in MORTGAGE
    // # ----------------------------------------------------------------------
    if (account.type === 'mortgage') {
      account.interest = arrayToObject(yearTable, 0);
      account.escrow = arrayToObject(yearTable, 0);
    }

    // # ----------------------------------------------------------------------
    // # Initialize the earnings table in SAVINGS, COLLEGE, and RETIREMENT
    // # ----------------------------------------------------------------------
    if (account.type === 'savings' || account.type === 'college' || account.type === 'retirement' || account.type === 'hsa') {
      account.earnings = arrayToObject(yearTable, 0);
      account.withdrawal = arrayToObject(yearTable, 0);
    }

    // # ----------------------------------------------------------------------
    // # Initialize the contribution table
    // # ----------------------------------------------------------------------
    if (Object.prototype.hasOwnProperty.call(account, 'yearlycontribution')) {
      account.contribution = arrayToObject(yearTable, 0);
    }

    // # ----------------------------------------------------------------------
    // # Initialize the contribution table
    // # ----------------------------------------------------------------------
    if (Object.prototype.hasOwnProperty.call(account, 'paymenttype')) {
      account.payment = arrayToObject(yearTable, 0);
    }

    // # ----------------------------------------------------------------------
    // # Initialize input values based on computation
    // # ----------------------------------------------------------------------
    if (Object.prototype.hasOwnProperty.call(account, 'startin') && typeof account.startin !== 'number') {
      if (account.startin === 'incomelink') {
        account.startin = accounts[account.incomelink].startin;
      } else {
        account.startin = eval(account.startin);
      }
    }
    if (Object.prototype.hasOwnProperty.call(account, 'endin') && typeof account.endin !== 'number') {
      if (account.endin === 'incomelink') {
        account.endin = accounts[account.incomelink].endin;
      } else {
        account.endin = eval(account.endin);
      }
    }
    if (Object.prototype.hasOwnProperty.call(account, 'startout') && typeof account.startout !== 'number') {
      if (account.startout === 'incomelink') {
        account.startout = accounts[account.incomelink].startout;
      } else {
        account.startout = eval(account.startout);
      }
    }
    if (Object.prototype.hasOwnProperty.call(account, 'endout') && typeof account.endout !== 'number') {
      if (account.endout === 'incomelink') {
        account.endout = accounts[account.incomelink].endout;
      } else {
        account.endout = eval(account.endout);
      }
    }
    if (Object.prototype.hasOwnProperty.call(account, 'raise') && typeof account.raise !== 'number') {
      account.raise = eval(account.raise);
    }
    // console.log(JSON.stringify(account));
  });

  // # ----------------------------------------------------------------------
  // # Main loop to loop through each year
  // # ----------------------------------------------------------------------
  yearTable.forEach((yearCurrent) => { // loop through all years
    // # ----------------------------------------------------------------------
    // # Initialize this year
    // # ----------------------------------------------------------------------
    if (yearCurrent > yearStart) {
      netTable[yearCurrent] = netTable[yearCurrent - 1]; // initialize this year as the value from last year
    }


    // # ----------------------------------------------------------------------
    // # Loop through accounts to make contributions and withdrawals
    // # ---------------------------------------------------------------------
    accountOrderIndex.forEach((accountName) => {
      // # ----------------------------------------------------------------------
      // # Initialize temp variables to zero
      // # ----------------------------------------------------------------------
      let earnings = 0; // earnings is money that an account gains (ie interest for a savings account or retirement account.  for an income account earnings is the yearly income)
      let interest = 0; // interest is money that must be payed off (ie for a loan or mortgage)
      let contribution = 0; // contribution is money that goes from income to a savings type account (savings, college, retirement, etc)
      let employermatch = 0; // set employermatch to zero
      let payment = 0; // payment is money that must come out of income
      let withdrawal = 0; // withdrawal is money that may be considered income (depending on account type)
      let expense = 0;

      let account = accounts[accountName];

      // # ----------------------------------------------------------------------
      // # Initialize the value of the account for this year
      // # ----------------------------------------------------------------------
      if (account.type === 'expense' || account.type === 'income') {
        // if this is an EXPENSE or INCOME account
        account.table[yearCurrent] = 0; // previous years value does not carry over (ie not an account that carries a balance)
      } else {
        // this account type should carry over the value from last year
        if (yearCurrent > yearStart) {
          account.table[yearCurrent] = account.table[yearCurrent - 1];
        }
      }

      // # ----------------------------------------------------------------------
      // # Calculate earnings
      // # ----------------------------------------------------------------------
      if (
        account.type === 'savings'
        || account.type === 'college'
        || account.type === 'retirement'
        || account.type === 'hsa'
      ) {
        // if this is a SAVINGS or COLLEGE etc account
        earnings = (account.table[yearCurrent] * account.yearlyreturn) / 100; // calculate earnings from interest
        account.earnings[yearCurrent] = earnings; // set account earnings to current earnings value
      } else if (account.type === 'income') {
        // Otherwise if this is an INCOME account
        if (account.startin <= yearCurrent && account.endin >= yearCurrent) {
          // if this income object is active this year
          earnings = account.base * ((1 + account.raise / 100) ** (yearCurrent - account.startin)); // calculate this years income
        }
      }

      // # ----------------------------------------------------------------------
      // # Calculate interest
      // # ----------------------------------------------------------------------

      if (account.type === 'loan') {
        // if this is a LOAN account
        interest = (account.table[yearCurrent] * account.rate) / 100.0;
        account.interest[yearCurrent] = interest;
      } else if (account.type === 'mortgage') {
        // Otherwise if this is a MORTGAGE account
        if ((account.table[yearCurrent] * 100.0) / account.value > account.ltvlimit) {
          // if the current loan to value is more than the cutoff limit
          interest = account.table[yearCurrent] * ((1 + account.rate / 100.0 / account.compoundtime) ** (account.compoundtime)) + account.mortgageinsurance + account.escrowvalue - account.table[yearCurrent]; // add this years interest to the mortgage then decrease mortgage by payment amount but reduce payment by the escrow and mortgage insurance values
        } else {
          // otherwise if the current loan to value is less than the cutoff limit
          interest = account.table[yearCurrent] * ((1 + account.rate / 100.0 / account.compoundtime) ** (account.compoundtime)) + account.escrowvalue - account.table[yearCurrent]; // add this years interest to the mortgage athe make payment on mortgage but but reduce payment by escrow value
          account.interest[yearCurrent] = interest;
          account.escrow[yearCurrent] = account.escrowvalue;
        }
      }

      // # ----------------------------------------------------------------------
      // # Calculate contribution amount
      // # ----------------------------------------------------------------------
      if (Object.prototype.hasOwnProperty.call(account, 'yearlycontribution')) {
        // if there contribution values are defined
        if (account.startin <= yearCurrent && account.endin >= yearCurrent) {
          // if making contribution this year
          // # ----------------------------------------------------------------------
          // # Calculate contribution amount based on contribution type
          // # ----------------------------------------------------------------------

          if (account.contributiontype === 'fixed_with_inflation') {
            // if inflation needs to be accounted for in the contribution
            contribution = account.yearlycontribution * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // increase the value by inflation
          } else if (account.contributiontype === 'fixed') {
            // otherwise if the contribution is a fixed value
            contribution = account.yearlycontribution; // set the contribution amount to the value input
          } else if (account.contributiontype === 'percent_of_income') {
            // otherwise if the contribution is a percent of income
            if (Object.prototype.hasOwnProperty.call(account, 'incomelink') && account.incomelink.length > 0) {
              // and if the account has an income object linked to it
              contribution = accounts[account.incomelink].table[yearCurrent] * (account.yearlycontribution / 100); // calculate the contribution value using that income account and the percentage input
            } else {
              // otherwise
              contribution = incomeTotalTaxableTable[yearCurrent] * (account.yearlycontribution / 100); // calculate the contribution using the total income for the year
            }
          }

          account.contribution[yearCurrent] = contribution; // set account contribution value to current contribution value

          if (contribution < 0) {
            console.log('Error contribution < 0');
            ipcRenderer.send('analysisError', {
              title: `${accountName} ${yearCurrent}`,
              message: 'contribution < 0',
            });
          }

          // # ----------------------------------------------------------------------
          // # Calculate the employer contribution
          // # ----------------------------------------------------------------------

          if (
            Object.prototype.hasOwnProperty.call(account, 'incomelink')
            && Object.prototype.hasOwnProperty.call(account, 'employermatch')
            && Object.prototype.hasOwnProperty.call(account, 'matchlimit')
          ) {
            if (account.incomelink.length >= 0) {
              // if there is an incomelink for this account
              if (typeof account.employermatch === 'number') {
                // if employermatch is not a list
                // let tmp = ;
                account.employermatch = [account.employermatch];
              }
              if (typeof account.matchlimit === 'number') {
                // if matchlimit is not a list
                // let tmp = ;
                account.matchlimit = [account.matchlimit];
              }

              if (account.employermatch[0] >= 0 && account.matchlimit[0] >= 0) {
                if (account.matchlimit.length > 1) {
                  // and if it is a complex employer matching (more than one level)
                  if (contribution >= (account.matchlimit[0] / 100 + account.matchlimit[1] / 100) * accounts[account.incomelink].table[yearCurrent]) {
                    // and if the contribution is above the highest employer matching level
                    employermatch = accounts[account.incomelink].table[yearCurrent] * ((account.employermatch[1] / 100) * (account.matchlimit[1] / 100) + (account.employermatch[0] / 100) * (account.matchlimit[0] / 100)); // calculate the employer matching based on the match limits
                  } else if (contribution >= (account.matchlimit[0] / 100) * accounts[account.incomelink].table[yearCurrent]) {
                    // otherwise if the contribution is between the employer matching levels ) {
                    employermatch = accounts[account.incomelink].table[yearCurrent] * ((account.employermatch[0] / 100) * (account.matchlimit[0] / 100) + (account.employermatch[1] / 100) * (account.matchlimit[1] / 100) * (contribution / accounts[account.incomelink].table[yearCurrent] - account.matchlimit[0] / 100)); // calculate the employer matching with all the first level and part of the second level
                  } else {
                    employermatch = contribution * (account.employermatch[0] / 100); // the employer contribution is computed based on the entire contribution
                  }
                } else {
                  // if it is a simple employer matching (only one level)
                  if (contribution >= account.matchlimit[0] * accounts[account.incomelink].table[yearCurrent]) {
                    // and if the contribution is above the highest employer matching level
                    employermatch = accounts[account.incomelink].table[yearCurrent] * (account.employermatch[0] / 100) * (account.matchlimit[0] / 100); // calculate the employer matching based on the match limits
                  } else {
                    // otherwise  if below the employer match limit
                    employermatch = contribution * (account.employermatch[0] / 100); // the employer contribution is computed based on the entire contribution
                  }
                }
              }
            } else {
              console.log(
                'Employer Match defined for account but incomelink length <= 0 ',
                account.name,
              );
              ipcRenderer.send('analysisError', {
                title: `${accountName} ${yearCurrent}`,
                message:
                  'employer match defined for account but incomelink length <= 0',
              });
            }
          } else if (Object.prototype.hasOwnProperty.call(account, 'employercontribution') && yearCurrent <= yearRetire) {
            if (account.contributiontype === 'fixed_with_inflation') {
              // if inflation needs to be accounted for in the contribution
              employermatch = account.employercontribution * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // increase the value by inflation
            } else if (account.contributiontype === 'fixed') {
              // otherwise if the contribution is a fixed value
              employermatch = account.employercontribution; // set the contribution amount to the value input
            } else {
              console.log('Employer Contribution type not implimented');
              ipcRenderer.send('analysisError', { title: `${accountName} ${yearCurrent}`, message: 'employer contribution type not implemented' });
            }
          }
        }
      }

      // # ----------------------------------------------------------------------
      // # Calculate payment
      // # ----------------------------------------------------------------------

      if (Object.prototype.hasOwnProperty.call(account, 'paymenttype')) {
        // if there is a payment defined
        if (account.startout <= yearCurrent && account.endout >= yearCurrent) {
          // if making a payment this year
          // # ----------------------------------------------------------------------
          // # Calculate payment amount
          // # ----------------------------------------------------------------------_
          if (account.paymenttype === 'fixed') {
            // otherwise if type is a fixed value
            payment = account.paymentvalue; // set withdrawal to the value
          } else if (account.paymenttype === 'fixed_with_inflation') {
            // otherwise if type is a fixed number but should be compensated for with inflation
            payment = account.paymentvalue * ((1 + inflationBase / 100) ** (yearCurrent - account.startout)); // set withdrawal to the value multiplied by an increase due to inflation
          } else {
            // otherwise if a different type is specified
            payment = 0; // set withdrawal to zero (this is for accounts that you dont remove money from such as expense accounts)
          }
          if (payment > account.table[yearCurrent]) {
            payment = account.table[yearCurrent];
          }
          account.payment[yearCurrent] = payment; // add payment to payment table
        }
        if (payment < 0) {
          console.log('Error payment < 0');
          ipcRenderer.send('analysisError', { title: `${accountName} ${yearCurrent}`, message: 'payment < 0' });
        }
      }

      // # ----------------------------------------------------------------------
      // # Calculate withdrawal
      // # ----------------------------------------------------------------------
      if (Object.prototype.hasOwnProperty.call(account, 'withdrawaltype')) {
        if (account.startout <= yearCurrent && account.endout >= yearCurrent) {
          // if taking money out this year

          // # ----------------------------------------------------------------------
          // # Calculate withdrawal amount
          // # ----------------------------------------------------------------------_
          if (account.withdrawaltype === 'col_frac_of_savings') {
            // otherwise if type is cost of living fraction of total savings
            if (yearCurrent > yearStart) {
              // withdrawal = costofliving['table'][yearIndex] * account[accountindex].table(yearIndex-1)./savingstotal.table(yearIndex-1)
              // account for retirement cost of living and for capital gains in this line...its a hack and probably not very correct
              if (account.table[yearCurrent - 1] > 0) {
                // if there is money left in the account (python gives error on zero / anything)
                // withdrawal from this account = total expenses this year  * fraction of total savings this account represents
                const totalExpensesThisYear = Object.values(expenseTotal[yearCurrent]).reduce((acc, cur) => acc + cur, 0);
                withdrawal = (totalExpensesThisYear * account.table[yearCurrent - 1]) / savingsTotalTable[yearCurrent - 1];
                if (Object.prototype.hasOwnProperty.call(account, 'taxstatus') && account.taxstatus === 3) {
                  withdrawal *= (taxIncome / 100 + 1); // add extra to amount withdrawal value to account for taxes.
                }
              }
            } else {
              console.log('ERROR - Can not compute withdrawal amount');
              ipcRenderer.send('analysisError', {
                title: `${accountName} ${yearCurrent}`,
                message: 'can not compute withdrawal amount < 0',
              });
            }
          } else if (account.withdrawaltype === 'fixed') {
            // otherwise if type is a fixed value
            withdrawal = account.withdrawalvalue; // set withdrawal to the value
          } else if (account.withdrawaltype === 'fixed_with_inflation') {
            // otherwise if type is a fixed number but should be compensated for with inflation
            withdrawal = account.withdrawalvalue * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // set withdrawal to the value multiplied by an increase due to inflation
          } else if (account.withdrawaltype === 'end_at_zero') {
            // otherwise if the type is end at zero
            if (account.endout >= yearCurrent) {
              // and if the year to stop taking money out of the account is beyond or equal to the current year
              withdrawal = account.table[yearCurrent] / (account.endout - yearCurrent + 1); // calculate the fraction of the account balance to withdraw
            }
          } else {
            // otherwise if a different type is specified
            console.log('Invalid withdrawal type');
            ipcRenderer.send('analysisError', {
              title: `${accountName} ${yearCurrent}`,
              message: 'invalid withdrawal type',
            });
            withdrawal = 0; // set withdrawal to zero (this is for accounts that you dont remove money from such as expense accounts)
          }
        }

        if (withdrawal > account.table[yearCurrent]) {
          // dont allow an account to become overdrawn
          withdrawal = account.table[yearCurrent];
        }
        if (withdrawal < 0) {
          console.log('Error withdrawal < 0');
          ipcRenderer.send('analysisError', {
            title: `${accountName} ${yearCurrent}`,
            message: 'withdrawal < 0',
          });
        }
        account.withdrawal[yearCurrent] = withdrawal;
      }

      // # ----------------------------------------------------------------------
      // # Calculate expense amount
      // # ----------------------------------------------------------------------
      if (account.type === 'expense') {
        // if there is a expense type defined
        if (account.startout <= yearCurrent && account.endout >= yearCurrent) {
          // if this expense applies this year

          // # ----------------------------------------------------------------------
          // # Calculate expense amount
          // # ----------------------------------------------------------------------_
          if (account.expensetype === 'fixed') {
            // otherwise if type is a fixed value
            expense = account.expensevalue; // set expense to the value
          } else if (account.expensetype === 'fixed_with_inflation') {
            // otherwise if type is a fixed number but should be compensated for with inflation
            expense = account.expensevalue * ((1 + inflationBase / 100) ** (yearCurrent - yearStart)); // set expense to the value multiplied by an increase due to inflation
          } else {
            // otherwise if a different type is specified
            expense = 0; // set expense to zero (this is for accounts that you dont remove money from such as expense accounts)
          }
          if (yearCurrent >= yearRetire) {
            expense *= retirementCostOfLiving / 100;
          }
        }
        if (expense < 0) {
          console.log('Error expense < 0');
          ipcRenderer.send('analysisError', {
            title: `${accountName} ${yearCurrent}`,
            message: 'expense < 0',
          });
        }
      }

      // # #      0=payed with taxed income, earnings are tax deferred, withdrawals are not taxed
      // # #      1=payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed
      // # #            (tax free as long as used for intended purpose)
      // # #      2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed
      // # #      3=payed pretax and taxed in year of use as income

      // # ----------------------------------------------------------------------
      // # Add earnings to the account for the year
      // # ----------------------------------------------------------------------

      account.table[yearCurrent] += earnings;
      // # ----------------------------------------------------------------------
      // # Add earnings to income tables for the year
      // # ----------------------------------------------------------------------
      if (account.type === 'income') {
        incomeTotalTaxableTable[yearCurrent] += account.table[yearCurrent]; // increase this years taxable income by the withdrawal amount
        incomeTotalTable[yearCurrent] += account.table[yearCurrent]; // increase this years income by the withdrawal amount
      }

      // # ----------------------------------------------------------------------
      // # Add capital gains earnings to taxable income
      // # ----------------------------------------------------------------------
      // #        if 'taxstatus' in account[accountindex] and account[accountindex]['taxstatus']==1 : # payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed
      // #            incometotaltaxable_table[yearIndex] = incometotaltaxable_table[yearIndex] + earnings * tax_capitalgains/100  # increase this years taxable income by the capital gains for this account

      // # ----------------------------------------------------------------------
      // # Add interest to the account for the year
      // # ----------------------------------------------------------------------

      account.table[yearCurrent] += interest;
      // # ----------------------------------------------------------------------
      // # Add contribution and employermatch to the account for the year
      // # ----------------------------------------------------------------------

      account.table[yearCurrent] = account.table[yearCurrent] + contribution + employermatch;
      if (Object.prototype.hasOwnProperty.call(account, 'taxstatus') && (account.taxstatus === 3 || account.taxstatus === 4)) {
        // if contributions should be taken out of taxable income for the year
        incomeTotalTaxableTable[yearCurrent] -= contribution; // take the contribution value out of taxable income for the year
      }

      // # ----------------------------------------------------------------------
      // # Add payment to the account for the year
      // # ----------------------------------------------------------------------
      account.table[yearCurrent] -= payment;

      // # ----------------------------------------------------------------------
      // # Add withdrawal to the account for the year
      // # ----------------------------------------------------------------------
      account.table[yearCurrent] -= withdrawal;

      if (account.type !== 'college') {
        // dont put college withdrawals into income because they go to kids not me
        if (
          Object.prototype.hasOwnProperty.call(account, 'taxstatus') && account.taxstatus === 3) {
          // if the withdrawal should be counted as taxable income for the year
          incomeTotalTaxableTable[yearCurrent] += withdrawal; // increase this years taxable income by the withdrawal amount
          incomeTotalTable[yearCurrent] += withdrawal; // increase this years taxable income by the withdrawal amount
        } else {
          incomeTotalTable[yearCurrent] += withdrawal; // increase this years taxable income by the withdrawal amount
        }
      }

      // # ----------------------------------------------------------------------
      // # Remove earnings value from net in year taken out
      // # earnings are taxed in year taken out as capital gains, withdrawals are not taxed
      // # ----------------------------------------------------------------------
      // #if 'taxstatus' in account[accountindex] and account[accountindex]['taxstatus']==2 :
      // #    net_table[yearCurrent] = net_table[yearCurrent] - sum(account[accountindex]['earnings']) / sum(account[accountindex]['contribution']) * withdrawal * tax_capitalgains/100
      // #    yearlybudget_table[yearCurrent] = yearlybudget_table[yearCurrent] - sum(account[accountindex]['earnings']) / sum(account[accountindex]['contribution']) * withdrawal * tax_capitalgains/100

      // # ----------------------------------------------------------------------
      // # Add expense to the account for the year
      // # ----------------------------------------------------------------------
      if (
        Object.prototype.hasOwnProperty.call(account, 'taxstatus') && (account.taxstatus === 3 || account.taxstatus === 4)) {
        // if contributions should be taken out of taxable income for the year
        // this is really just paying for health insurance as it is the only expense that has a taxstatus = 3 or 4
        incomeTotalTaxableTable[yearCurrent] -= expense; // take the expense value out of taxable income for the year
      }

      account.table[yearCurrent] += expense;

      if (
        Object.prototype.hasOwnProperty.call(account, 'ishealthcare') && account.ishealthcare === 1) {
        // pull from HSA
        if (Object.prototype.hasOwnProperty.call(account, 'hsalink')) {
          // if there is an HSA account linked

          // print('{0:s} {1:.2f} {2:.2f}'.format(account[account[accountindex]['hsalink']]['name'],account[account[accountindex]['hsalink']]['table'][yearCurrent], expense))
          if (accounts[account.hsalink].table[yearCurrent] >= expense) {
            // if there is enough money in the HSA savings account to pay for healthcare expenses
            accounts[account.hsalink].table[yearCurrent] -= expense;
            accounts[account.hsalink].withdrawal[yearCurrent] += expense;
          } else {
            // otherwise drain the HSA account then reset expense to represent the remaining balance of the expense
            accounts[account.hsalink].withdrawal[yearCurrent] = accounts[account.hsalink].table[yearCurrent];
            const tmp = expense - accounts[account.hsalink].table[yearCurrent];
            accounts[account.hsalink].table[yearCurrent] = 0;
            expense = tmp;
          }
        }
      }

      // # ----------------------------------------------------------------------
      // # Add entry to expense total table
      // # ----------------------------------------------------------------------
      if (account.type === 'expense') {
        // and if type is EXPENSE
        expenseTotal[yearCurrent][accountName] = expense; // add withdrawal to the expense table
      } else if (account.type === 'loan') {
        // otherwise if type is LOAN
        expenseTotal[yearCurrent][accountName] = payment; // add withdrawal to the expense table
      } else if (account.type === 'mortgage') {
        // otherwise if type is LOAN
        expenseTotal[yearCurrent][accountName] = payment; // add withdrawal to the expense table
      } else if (account.type === 'college') {
        // otherwise if type is LOAN
        expenseTotal[yearCurrent][accountName] = contribution; // add contribution to the expense table
      } else if (account.type === 'savings') {
        // otherwise if type is a SAVINGS account
        expenseTotal[yearCurrent][accountName] = contribution; // add contribution to the expense table
      } else if (account.type === 'retirement') {
        // otherwise if type is a SAVINGS account
        expenseTotal[yearCurrent][accountName] = contribution; // add contribution to the expense table
      } else if (account.type === 'hsa') {
        // otherwise if type is a HSA account
        expenseTotal[yearCurrent][accountName] = contribution; // add contribution to the expense table
      }

      // # ----------------------------------------------------------------------
      // # Add entry to savings total
      // # ----------------------------------------------------------------------
      if (account.type === 'savings' || account.type === 'retirement') {
        // if this is a savings account
        savingsTotalTable[yearCurrent] += account.table[yearCurrent];
      }
    }); // end for each account loop

    // # ----------------------------------------------------------------------
    // # Add Income to net account (subtract out paying for income tax)
    // # ----------------------------------------------------------------------
    const totalExpensesThisYear = Object.values(expenseTotal[yearCurrent]).reduce((acc, cur) => acc + cur, 0);

    netTable[yearCurrent] = netTable[yearCurrent] + incomeTotalTable[yearCurrent] - incomeTotalTaxableTable[yearCurrent] * (taxIncome / 100) - totalExpensesThisYear;
    incomeTotalAfterTaxTable[yearCurrent] = incomeTotalTable[yearCurrent] - incomeTotalTaxableTable[yearCurrent] * (taxIncome / 100);
  }); // end for each year loop

  console.log('accounts after main loop');
  console.log(accounts);

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
