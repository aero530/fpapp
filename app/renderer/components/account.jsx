import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import classNames from 'classnames';

import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

import Tooltip from '@material-ui/core/Tooltip';
import HelpIcon from '@material-ui/icons/HelpOutline';

import MenuItem from '@material-ui/core/MenuItem';
import TextField from '@material-ui/core/TextField';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';

import NumberFormat from 'react-number-format';

const styles = theme => ({
  root: {
    width: '100%',
    marginTop: theme.spacing.unit * 3
  },
  paper: {
    width: '100%',
    backgroundColor: theme.palette.background.paper,
    boxShadow: theme.shadows[5],
    padding: theme.spacing.unit * 4,
    marginBottom: theme.spacing.unit * 4
  },
  margin: {
    margin: theme.spacing.unit
  },
  withoutLabel: {
    marginTop: theme.spacing.unit * 3
  },
  textField: {
    flexBasis: 200
  }
});

function NumberFormatPercentage(props) {
  const { inputRef, onChange, ...other } = props;

  return (
    <NumberFormat
      {...other}
      getInputRef={inputRef}
      onValueChange={(values, event) => {
        onChange({
          target: {
            value: values.value,
            floatValue: values.floatValue,
            id: event.target.id
          }
        });
      }}
      suffix="%"
    />
  );
}

NumberFormatPercentage.propTypes = {
  inputRef: PropTypes.func.isRequired,
  onChange: PropTypes.func.isRequired
};

function NumberFormatDollar(props) {
  const { inputRef, onChange, ...other } = props;

  return (
    <NumberFormat
      {...other}
      getInputRef={inputRef}
      onValueChange={(values, event) => {
        onChange({
          target: {
            value: values.value,
            floatValue: values.floatValue,
            id: event.target.id
          }
        });
      }}
      thousandSeparator
      prefix="$"
    />
  );
}

NumberFormatDollar.propTypes = {
  inputRef: PropTypes.func.isRequired,
  onChange: PropTypes.func.isRequired
};

function NumberFormatYear(props) {
  const { inputRef, onChange, ...other } = props;

  return (
    <NumberFormat
      {...other}
      getInputRef={inputRef}
      onValueChange={(values, event) => {
        onChange({
          target: {
            value: values.value,
            floatValue: values.floatValue,
            id: event.target.id
          }
        });
      }}
      format="####"
    />
  );
}

NumberFormatYear.propTypes = {
  inputRef: PropTypes.func.isRequired,
  onChange: PropTypes.func.isRequired
};

const taxStatusOptions = [
  {
    value: 0,
    label: 'contribute taxed income - earnings taxed deferred',
    description:
      'payed with taxed income, earnings are tax deferred, withdrawals are not taxed'
  },
  {
    value: 1,
    label: 'contribute taxed income - earings are capital gains',
    description:
      'payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)'
  },
  {
    value: 2,
    label: 'not implemented',
    description:
      'NOT IMPLEMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed'
  },
  {
    value: 3,
    label: 'contribute pretax income - taxed as income when used',
    description: 'payed pretax and taxed in year of use as income'
  },
  {
    value: 4,
    label: 'contribute pretax income - withdrawal not taxed as income (HSA)',
    description: 'payed pretax and not taxed as income (use with HSA)'
  }
];

const contributionTypeOptions = [
  {
    value: 'fixed',
    label: 'fixed',
    description: 'fixed dollar amount'
  },
  {
    value: 'percent_of_income',
    label: 'percent of income',
    description: 'percent of cost of current living'
  },
  {
    value: 'fixed_with_inflation',
    label: 'fixed with inflation',
    description:
      'fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)'
  }
];

const withdrawalTypeOptions = [
  {
    value: 'end_at_zero',
    label: 'end at zero',
    description:
      'take money out in equal amounts each year such that the balance at endout is zero'
  },
  {
    value: 'fixed',
    label: 'fixed',
    description: 'Take out a fixed dollar amount'
  },
  {
    value: 'COL_fraction_of_total_savings',
    label: 'fixed with inflation',
    description:
      'Take out the current cost of living * (this accounts value / total savings)'
  }
];

const paymentTypeOptions = [
  {
    value: 'fixed',
    label: 'fixed',
    description: 'fixed dollar amount'
  },
  {
    value: 'fixed_with_inflation',
    label: 'fixed with inflation',
    description:
      'fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)'
  }
];

class Account extends Component {
  constructor(props, context) {
    super(props, context);
    // initialize the state
    this.state = {};
  }

  render() {
    const { classes, account, onAccountChange } = this.props;

    const show = {
      income: {
        name: true,
        table: true,
        startin: true,
        endin: true,
        raise: true,
        base: true
      },
      retirement: {
        name: true,
        table: true,
        startin: true,
        endin: true,
        startout: true,
        endout: true,
        yearlycontribution: true,
        contributiontype: true,
        yearlyreturn: true,
        withdrawaltype: true,
        withdrawalvalue: true,
        taxstatus: true,
        incomelink: true,
        employermatch: true,
        matchlimit: true
      },
      hsa: {
        name: true,
        table: true,
        startin: true,
        endin: true,
        startout: true,
        endout: true,
        yearlycontribution: true,
        employercontribution: true,
        contributiontype: true,
        yearlyreturn: true,
        taxstatus: true
      },
      college: {
        name: true,
        table: true,
        startin: true,
        endin: true,
        startout: true,
        endout: true,
        yearlycontribution: true,
        contributiontype: true,
        yearlyreturn: true,
        withdrawaltype: true,
        withdrawalvalue: true,
        taxstatus: true
      },
      expense: {
        name: true,
        table: true,
        startout: true,
        endout: true,
        expensetype: true,
        expensevalue: true
      },
      loan: {
        name: true,
        table: true,
        startout: true,
        endout: true,
        paymenttype: true,
        paymentvalue: true,
        rate: true
      },
      mortgage: {
        name: true,
        table: true,
        startout: true,
        endout: true,
        paymenttype: true,
        paymentvalue: true,
        rate: true,
        compoundtime: true,
        mortgageinsurance: true,
        ltvlimit: true,
        escrowvalue: true,
        value: true
      }
    };

    return (
      <Paper className={classes.paper}>
        <Typography variant="title" id="modal-title">
          {account.name}
        </Typography>

        {show[account.type].name ? (
          <Tooltip title="String describing this income source">
            <TextField
              id="name"
              label="Account Name"
              className={classNames(classes.margin, classes.textField)}
              value={account.name}
              onChange={event => {
                onAccountChange(
                  account.name,
                  event.target.id,
                  event.target.value
                );
              }}
            />
          </Tooltip>
        ) : null}

        <div>
          {show[account.type].incomelink ? (
            <Tooltip title="Link this account to an income source">
              <TextField
                id="incomelink"
                label="Income Link"
                className={classNames(classes.margin, classes.textField)}
                value={account.incomelink}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.value
                  );
                }}
              />
            </Tooltip>
          ) : null}

          {show[account.type].employermatch ? (
            <Tooltip title="Percent of your contribution that your employer matches">
              <TextField
                id="employermatch"
                label="Employer Match %"
                className={classNames(classes.margin, classes.textField)}
                value={account.employermatch}
                InputProps={{ inputComponent: NumberFormatPercentage }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}

          {show[account.type].matchlimit ? (
            <Tooltip title="Employer match applies to up to this percentage of your base income">
              <TextField
                id="matchlimit"
                label="Match Limit %"
                className={classNames(classes.margin, classes.textField)}
                value={account.matchlimit}
                InputProps={{ inputComponent: NumberFormatPercentage }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
        </div>

        <div>
          {show[account.type].startin ? (
            <Tooltip title="Calendar year when money starts coming out of income and going into this account">
              <TextField
                id="startin"
                label="Begin Contributions"
                className={classNames(classes.margin, classes.textField)}
                value={account.startin}
                InputProps={{ inputComponent: NumberFormatYear }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}

          {show[account.type].endin ? (
            <Tooltip title="Calendar year when money no longer goes to this account (this is inclusive so it will generally be year_retire-1)">
              <TextField
                id="endin"
                label="End Contributions"
                className={classNames(classes.margin, classes.textField)}
                value={account.endin}
                InputProps={{ inputComponent: NumberFormatYear }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
        </div>

        <div>
          {show[account.type].startout ? (
            <Tooltip title="Calendar year when money starts coming out of this account and acts as income">
              <TextField
                id="startout"
                label="Begin Withdrawals"
                className={classNames(classes.margin, classes.textField)}
                value={account.startout}
                InputProps={{ inputComponent: NumberFormatYear }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
          {show[account.type].endout ? (
            <Tooltip title="Calendar year when money no longer is taken out of this account">
              <TextField
                id="endout"
                label="End Withdrawals"
                className={classNames(classes.margin, classes.textField)}
                value={account.endout}
                InputProps={{ inputComponent: NumberFormatYear }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
        </div>

        <div>
          {show[account.type].yearlycontribution ? (
            <Tooltip title="Amount put into this account every year">
              <TextField
                id="yearlycontribution"
                label="Yearly Contribution"
                className={classNames(classes.margin, classes.textField)}
                value={account.yearlycontribution}
                InputProps={{ inputComponent: NumberFormatDollar }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}

          {show[account.type].contributiontype ? (
            <TextField
              select
              id="contributiontype"
              label="Contribution Type"
              className={classNames(classes.margin, classes.textField)}
              value={account.contributiontype}
              onChange={() => {}}
            >
              {contributionTypeOptions.map(option => (
                <MenuItem key={option.value} value={option.value}>
                  {option.label}
                </MenuItem>
              ))}
            </TextField>
          ) : null}
          {show[account.type].contributiontype ? (
            <Tooltip
              title={
                <List>
                  {contributionTypeOptions.map(option => (
                    <ListItem key={`contributionTypeToolTip${option.value}`}>
                      {option.value} - {option.description}
                    </ListItem>
                  ))}
                </List>
              }
            >
              <HelpIcon />
            </Tooltip>
          ) : null}

          {show[account.type].yearlyreturn ? (
            <Tooltip title="Percent interest earned each year">
              <TextField
                id="yearlyreturn"
                label="Interest %"
                className={classNames(classes.margin, classes.textField)}
                value={account.yearlyreturn}
                InputProps={{ inputComponent: NumberFormatPercentage }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}

          {show[account.type].base ? (
            <Tooltip title="Base pay (with bonuses)">
              <TextField
                id="base"
                label="Base"
                className={classNames(classes.margin, classes.textField)}
                value={account.base}
                InputProps={{ inputComponent: NumberFormatDollar }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}

          {show[account.type].raise ? (
            <Tooltip title="Yearly increase in income as a percent">
              <TextField
                id="raise"
                label="Raise"
                className={classNames(classes.margin, classes.textField)}
                value={account.raise}
                InputProps={{ inputComponent: NumberFormatPercentage }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
        </div>

        <div>
          {show[account.type].withdrawaltype ? (
            <TextField
              select
              id="withdrawaltype"
              label="Withdrawal Type"
              className={classNames(classes.margin, classes.textField)}
              value={account.withdrawaltype}
              onChange={() => {}}
            >
              {withdrawalTypeOptions.map(option => (
                <MenuItem key={option.value} value={option.value}>
                  {option.label}
                </MenuItem>
              ))}
            </TextField>
          ) : null}
          {show[account.type].withdrawaltype ? (
            <Tooltip
              title={
                <List>
                  {withdrawalTypeOptions.map(option => (
                    <ListItem key={`withdrawalTypeToolTip${option.value}`}>
                      {option.value} - {option.description}
                    </ListItem>
                  ))}
                </List>
              }
            >
              <HelpIcon />
            </Tooltip>
          ) : null}

          {show[account.type].withdrawalvalue ? (
            <Tooltip title="How much money should be take out per year (either as a percentage or a fixed dollar amount)">
              <TextField
                id="withdrawalvalue"
                label="Withdrawal Amount"
                className={classNames(classes.margin, classes.textField)}
                value={account.withdrawalvalue}
                InputProps={{ inputComponent: NumberFormatDollar }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
        </div>

        <div>
          {show[account.type].paymenttype ? (
            <TextField
              select
              id="paymenttype"
              label="Payment Type"
              className={classNames(classes.margin, classes.textField)}
              value={account.paymenttype}
              onChange={() => {}}
            >
              {paymentTypeOptions.map(option => (
                <MenuItem key={option.value} value={option.value}>
                  {option.label}
                </MenuItem>
              ))}
            </TextField>
          ) : null}
          {show[account.type].paymenttype ? (
            <Tooltip
              title={
                <List>
                  {paymentTypeOptions.map(option => (
                    <ListItem key={`paymentTypeToolTip${option.value}`}>
                      {option.value} - {option.description}
                    </ListItem>
                  ))}
                </List>
              }
            >
              <HelpIcon />
            </Tooltip>
          ) : null}

          {show[account.type].paymentvalue ? (
            <Tooltip title="How much money should be payed each year (either as a percentage or a fixed dollar amount)">
              <TextField
                id="paymentvalue"
                label="Payment Amount"
                className={classNames(classes.margin, classes.textField)}
                value={account.paymentvalue}
                InputProps={{ inputComponent: NumberFormatDollar }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
        </div>

        <div>
          {show[account.type].taxstatus ? (
            <TextField
              select
              label="Tax Status"
              className={classNames(classes.margin, classes.textField)}
              value={account.taxstatus}
              onChange={() => {}}
            >
              {taxStatusOptions.map(option => (
                <MenuItem key={option.value} value={option.value}>
                  {option.label}
                </MenuItem>
              ))}
            </TextField>
          ) : null}
          {show[account.type].taxstatus ? (
            <Tooltip
              title={
                <List>
                  {taxStatusOptions.map(option => (
                    <ListItem key={`taxStatusToolTip${option.value}`}>
                      {option.value} - {option.description}
                    </ListItem>
                  ))}
                </List>
              }
            >
              <HelpIcon />
            </Tooltip>
          ) : null}
        </div>

        <div>
          {show[account.type].rate ? (
            <Tooltip title="Interest rate on borrowed money. This is an APR this is then compounded based on the compound time setting.  Used for LOAN and MORTGAGE account types.">
              <TextField
                id="rate"
                label="Interest Rate %"
                className={classNames(classes.margin, classes.textField)}
                value={account.rate}
                InputProps={{ inputComponent: NumberFormatPercentage }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
          {show[account.type].compoundtime ? (
            <Tooltip title="Number of times per year that interest is compounded. (1=yearly, 12=monthly) Used for MORTGAGE account types.">
              <TextField
                id="compoundtime"
                label="Compound Time"
                className={classNames(classes.margin, classes.textField)}
                value={account.compoundtime}
                InputProps={{ inputComponent: NumberFormatYear }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
          {show[account.type].mortgageinsurance ? (
            <Tooltip title="Mortgage insurance payment expressed as a yearly fixed number in todays dollars">
              <TextField
                id="mortgageinsurance"
                label="Mortgage Insurance"
                className={classNames(classes.margin, classes.textField)}
                value={account.mortgageinsurance}
                InputProps={{ inputComponent: NumberFormatDollar }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
          {show[account.type].ltvlimit ? (
            <Tooltip title="Loan to Value amount when mortgage insurance is no longer pulled from payment.  Since monthly payment does not change over time, after the insurance is done there is more money going to the principal each payment">
              <TextField
                id="ltvlimit"
                label="Loan to Value"
                className={classNames(classes.margin, classes.textField)}
                value={account.ltvlimit}
                InputProps={{ inputComponent: NumberFormatPercentage }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
          {show[account.type].escrow ? (
            <Tooltip title="Amount of money going into escrow every year to pay for property tax.  This number is currently assumed to be constant (ie property taxes do not increase)">
              <TextField
                id="escrow"
                label="Escrow"
                className={classNames(classes.margin, classes.textField)}
                value={account.escrow}
                InputProps={{ inputComponent: NumberFormatDollar }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}

          {show[account.type].value ? (
            <Tooltip title="Current value of the home.  This is used to compute loan to value">
              <TextField
                id="value"
                label="Value"
                className={classNames(classes.margin, classes.textField)}
                value={account.value}
                InputProps={{ inputComponent: NumberFormatDollar }}
                onChange={event => {
                  onAccountChange(
                    account.name,
                    event.target.id,
                    event.target.floatValue
                  );
                }}
              />
            </Tooltip>
          ) : null}
        </div>
      </Paper>
    );
  }
}

Account.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  account: PropTypes.object.isRequired,
  onAccountChange: PropTypes.func.isRequired
};

Account.defaultProps = {};

export default withStyles(styles)(Account);

/*
# account.type : Type of account
#      SAVINGS : Savings accounts should be used for any account where you 
#            accumulate wealth such as a bank savings account, money market account,
#            Roth IRA, or 401K.  Withdrawals from this account go into income
#            or net depending on what the tax status setting is.  Can have employer 
#            matching on these accounts
#      EXPENSE : Expense accounts account for daily expenses such as grocery, car 
#            insurance, clothes, travel, entertainment, etc.  Money to pay for these
#            things comes out of after tax income then out of net.
#      MORTGAGE : Home mortgage
#      LOAN : General loan type.  Can be used for things like car loans or school 
#            loans...anything with a balance due, interest rate, and consistent payments
#      COLLEGE : 529 account.  Withdrawals from this account will not go toward income or net
# account.name : String describing this income source
# account.table(1) : Starting balance.  For LOAN and MORTGAGE this should be a negative number (money is owed)
# account.startin : Calendar year when money starts coming out of income and going into this account
# account.endin : Calendar year when money no longer goes to this account (this is inclusive so it will generally be year_retire-1)
# account.startout : Calendar year when money starts coming out of this account and acts as income
# account.endout : Calendar year when money no longer is taken out of this account
# account.yearlycontribution : Amount put into this account every year
# account.contributiontype : Type of contribution
#      fixed : fixed dollar amount
#      percent_of_income : percent of cost of current living
#      fixed_with_inflation : fixed dollar amount compensated for inflation from year 
#            start (ie dollar amount is in current dollars)
# account.yearlyreturn : Percent interest earned each year
# account.withdrawaltype : How money should be removed from the account
#      end_at_zero : take money out in equal amounts each year such that the 
#            balance at endout is zero
#      fixed : Take out a fixed dollar amount
#      COL_fraction_of_total_savings : Take out the current cost of living * (this accounts value / total savings)
# account.withdrawalvalue : How much money should be take out per year 
#      (either as a percentage or a fixed dollar amount)
# account.paymenttype : How money should be removed from the account
#      fixed : fixed dollar amount
#      fixed_with_inflation : fixed dollar amount compensated for inflation from year 
#            start (ie dollar amount is in current dollars)
# account.paymentvalue : How much money should be payed each year
#      (either as a percentage or a fixed dollar amount)
# account.taxstatus : 
#      0=payed with taxed income, earnings are tax deferred, withdrawals are not taxed
#      1=payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed
#            (tax free as long as used for intended purpose)
#      ## NOT IMPLIMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed
#      3=payed pretax and taxed in year of use as income
#      4=payed pretax and not taxed as income (use with HSA)
# account.rate : Interest rate on borrowed money. This is an APR this is 
#      then compounded based on the compound time setting.  Used for LOAN and 
#      MORTGAGE account types.
# account.compoundtime : Number of times per year that interest
#       is compounded. (1=yearly, 12=monthly) Used for MORTGAGE account types.
# account.mortgageinsurance : Mortgage insurance payment expressed as 
#      a yearly fixed number in today's dollars
# account.ltvlimit : Loan to Value amount when mortgage insurance is no 
#      longer pulled from payment.  Since monthly payment does not change over 
#      time, after the insurance is done there is more money going to the 
#      principal each payment
# account.escrow : Amount of money going into escrow every year to pay 
#      for property tax.  This number is currently assumed to be constant 
#      (ie property taxes do not increase)
# account.value : Current value of the home.  This is used to compute loan to value
# account.raise : Yearly increase in income as a percent
*/
