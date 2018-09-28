// cSpell: ignore Formaters, autofill

import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import classNames from 'classnames';

import Card from '@material-ui/core/Card';
import CardContent from '@material-ui/core/CardContent';
import CardHeader from '@material-ui/core/CardHeader';

import Dialog from '@material-ui/core/Dialog';
import DialogActions from '@material-ui/core/DialogActions';
import DialogContent from '@material-ui/core/DialogContent';
import DialogContentText from '@material-ui/core/DialogContentText';
import DialogTitle from '@material-ui/core/DialogTitle';

import Button from '@material-ui/core/Button';
import IconButton from '@material-ui/core/IconButton';
import DeleteIcon from '@material-ui/icons/Delete';

import Tooltip from '@material-ui/core/Tooltip';
import HelpIcon from '@material-ui/icons/HelpOutline';

import MenuItem from '@material-ui/core/MenuItem';
import TextField from '@material-ui/core/TextField';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import Select from '@material-ui/core/Select';

import MuiEditableTable from './muiEditableTable';

import SuggestedInput from './suggestedInput';

import {
  NumberFormatPercentage,
  NumberFormatDollarPercentage,
  NumberFormatDollar,
  NumberFormatYear,
} from './numberFormaters';

import {
  taxStatusTypeOptions,
  contributionTypeOptions,
  expenseTypeOptions,
  withdrawalTypeOptions,
  paymentTypeOptions,
} from './autofillTypeOptions';

import { show } from './accountStructure';

const percentSuggestions = [{ label: 'inflationBase' }];

const yearSuggestions = [
  { label: 'yearStart' },
  { label: 'yearRetire' },
  { label: 'yearDie' },
  { label: 'yearEnd' },
];

const styles = theme => ({
  root: {
    width: '100%',
    marginTop: theme.spacing.unit * 3,
  },
  paper: {
    width: '100%',
    backgroundColor: theme.palette.background.paper,
    boxShadow: theme.shadows[5],
    padding: theme.spacing.unit * 4,
    marginBottom: theme.spacing.unit * 4,
    overflow: 'visible',
  },
  margin: {
    margin: theme.spacing.unit,
  },
  textField: {
    flexBasis: 200,
  },
  textArrayField: {
    width: '100%',
  },
  textArrayFont: {
    fontFamily: '"roboto-mono", "Courier New", sans-serif',
    fontSize: '0.7rem',
  },
  deleteFloatingActionButton: {
    position: 'absolute',
    bottom: theme.spacing.unit * 0,
    right: theme.spacing.unit * 0,
  },
});


class Account extends Component {
  constructor(props, context) {
    super(props, context);
    // initialize the state
    this.state = {
      account: { ...props.account },
      open: false,
    };
  }

  handleDialogOpen = () => {
    this.setState({ open: true });
  };

  handleDialogClose = () => {
    this.setState({ open: false });
  };

  handleDeleteTrue = () => {
    const { onDelete } = this.props;
    this.handleDialogClose();
    onDelete();
  };

  handleChange = (fieldNameInput, fieldValueInput) => {
    const { onUpdate } = this.props;
    const { account } = this.state;
    const newAccount = { ...account, [fieldNameInput]: fieldValueInput };
    this.setState({
      account: newAccount,
    });
    onUpdate(newAccount);
  };

  makeTableArray = (object) => {
    const table = [];
    Object.keys(object).sort((a, b) => a - b).forEach((key) => {
      table.push({ year: key, value: object[key] });
    });
    return table;
  }

  arrayToObject = (array) => {
    const obj = {};
    array.forEach((row) => {
      obj[row.year] = row.value;
    });
    return obj;
  };

  render() {
    const { classes, incomeAccounts } = this.props;
    const { account } = this.state;

    const colSpec = [
      { title: 'Year', fieldName: 'year', inputType: 'TextField', width: 200 },
      { title: 'Value', fieldName: 'value', inputType: 'TextField', width: 200 },
    ];

    const onAccountTableChange = (dataTable) => {
      console.log(dataTable);
      console.log(this.arrayToObject(dataTable));
      this.handleChange('table', this.arrayToObject(dataTable));
    };

    return (
      <Card className={classes.paper}>
        <CardHeader
          title={account.name}
          action={(
            <IconButton onClick={this.handleDialogOpen}>
              <DeleteIcon />
            </IconButton>
          )}
        />

        <Dialog
          open={this.state.open}
          onClose={this.handleDialogClose}
          aria-labelledby="alert-dialog-title"
          aria-describedby="alert-dialog-description"
        >
          <DialogTitle id="alert-dialog-title">{account.name}</DialogTitle>
          <DialogContent>
            <DialogContentText id="alert-dialog-description">
              Are you sure you want to delete this account?
            </DialogContentText>
          </DialogContent>
          <DialogActions>
            <Button onClick={this.handleDeleteTrue} color="primary">
              Delete
            </Button>
            <Button onClick={this.handleDialogClose} color="primary" autoFocus>
              Cancel
            </Button>
          </DialogActions>
        </Dialog>

        <CardContent>
          <div>
            {show[account.type].name ? (
              <Tooltip title="String describing this income source">
                <TextField
                  id="name"
                  label="Account Name"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.name}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.value);
                  }}
                />
              </Tooltip>
            ) : null}

            {show[account.type].incomelink ? (
              <Select
                inputProps={{
                  name: 'incomelink',
                  id: 'incomelink',
                }}
                label="Income Link"
                className={classNames(classes.margin, classes.textField)}
                value={account.incomelink ? account.incomelink : 'none'}
                onChange={(event) => {
                  this.handleChange(event.target.name, event.target.value);
                }}
              >
                {incomeAccounts.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </Select>
            ) : null}
            {show[account.type].incomelink ? (
              <Tooltip title="Link this account to an income source">
                <HelpIcon />
              </Tooltip>
            ) : null}
          </div>

          <div>
            {show[account.type].employermatch ? (
              <Tooltip title="Percent of your contribution that your employer matches">
                <TextField
                  id="employermatch"
                  label="Employer Match %"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.employermatch}
                  disabled={
                    account.incomelink === 'none' || !('incomelink' in account)
                  }
                  InputProps={{ inputComponent: NumberFormatPercentage }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                  disabled={
                    account.incomelink === 'none' || !('incomelink' in account)
                  }
                  InputProps={{ inputComponent: NumberFormatPercentage }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}
          </div>

          <div>
            {show[account.type].startin ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={account.startin}
                id="startin"
                label="Begin Contributions"
                helperText="can be based on 'year' strings"
                title="Calendar year when money starts coming out of income and going into this account"
                titleLocation="right"
                suggestionsList={yearSuggestions}
                onInputChange={(fieldName, fieldValue) => {
                  this.handleChange(fieldName, fieldValue);
                }}
              />
            ) : null}

            {show[account.type].endin ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={account.endin}
                id="endin"
                label="End Contributions"
                helperText="can be based on 'year' strings"
                title="Calendar year when money no longer goes to this account (this is inclusive so it will generally be yearRetire-1)"
                titleLocation="right"
                suggestionsList={yearSuggestions}
                onInputChange={(fieldName, fieldValue) => {
                  this.handleChange(fieldName, fieldValue);
                }}
              />
            ) : null}
          </div>

          <div>
            {show[account.type].startout ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={account.startout}
                id="startout"
                label="Begin Withdrawals"
                helperText="can be based on 'year' strings"
                title="Calendar year when money starts coming out of this account and acts as income"
                titleLocation="right"
                suggestionsList={yearSuggestions}
                onInputChange={(fieldName, fieldValue) => {
                  this.handleChange(fieldName, fieldValue);
                }}
              />
            ) : null}
            {show[account.type].endout ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={account.endout}
                id="endout"
                label="End Withdrawals"
                helperText="can be based on 'year' strings"
                title="Calendar year when money no longer is taken out of this account"
                titleLocation="right"
                suggestionsList={yearSuggestions}
                onInputChange={(fieldName, fieldValue) => {
                  this.handleChange(fieldName, fieldValue);
                }}
              />
            ) : null}
          </div>

          <div>
            {show[account.type].yearlycontribution ? (
              <Tooltip title="Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage.">
                <TextField
                  id="yearlycontribution"
                  label="Yearly Contribution"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.yearlycontribution}
                  InputProps={
                    'incomelink' in account && account.incomelink !== 'none'
                      ? { inputComponent: NumberFormatDollarPercentage }
                      : { inputComponent: NumberFormatDollar }
                  }
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                onChange={(event) => {
                  this.handleChange('contributiontype', event.target.value);
                }}
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
                title={(
                  <List>
                    {contributionTypeOptions.map(option => (
                      <ListItem key={`contributionTypeToolTip${option.value}`}>
                        {option.value}
                        -
                        {option.description}
                      </ListItem>
                    ))}
                  </List>
                )}
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}

            {show[account.type].raise ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={account.raise}
                id="raise"
                label="% Raise"
                helperText="can be based on 'year' strings"
                title="Yearly increase in income as a percent"
                titleLocation="right"
                suggestionsList={percentSuggestions}
                onInputChange={(fieldName, fieldValue) => {
                  this.handleChange(fieldName, fieldValue);
                }}
              />
            ) : null}
          </div>

          <div>
            {show[account.type].table ? (
              <div>
                <MuiEditableTable
                  colSpec={colSpec}
                  rowData={this.makeTableArray(account.table)}
                  onChange={onAccountTableChange}
                  reorderable
                />
              </div>
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
                onChange={(event) => {
                  this.handleChange('withdrawaltype', event.target.value);
                }}
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
                title={(
                  <List>
                    {withdrawalTypeOptions.map(option => (
                      <ListItem key={`withdrawalTypeToolTip${option.value}`}>
                        {option.value}
                        -
                        {option.description}
                      </ListItem>
                    ))}
                  </List>
                )}
              >
                <HelpIcon />
              </Tooltip>
            ) : null}

            {show[account.type].withdrawalvalue ? (
              <Tooltip title="How much money should be take out per year (either as a percentage or a fixed dollar amount)">
                <TextField
                  id="withdrawalvalue"
                  label="Withdrawal Amount"
                  disabled={account.withdrawaltype === 'end_at_zero' || account.withdrawaltype === 'col_frac_of_savings'}
                  className={classNames(classes.margin, classes.textField)}
                  value={account.withdrawalvalue}
                  InputProps={{ inputComponent: NumberFormatDollar }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                onChange={(event) => {
                  this.handleChange('paymenttype', event.target.value);
                }}
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
                title={(
                  <List>
                    {paymentTypeOptions.map(option => (
                      <ListItem key={`paymentTypeToolTip${option.value}`}>
                        {option.value}
                        -
                        {option.description}
                      </ListItem>
                    ))}
                  </List>
                )}
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                onChange={(event) => {
                  this.handleChange('taxstatus', event.target.value);
                }}
              >
                {taxStatusTypeOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </TextField>
            ) : null}
            {show[account.type].taxstatus ? (
              <Tooltip
                title={(
                  <List>
                    {taxStatusTypeOptions.map(option => (
                      <ListItem key={`taxStatusToolTip${option.value}`}>
                        {option.value}
                        -
                        {option.description}
                      </ListItem>
                    ))}
                  </List>
                )}
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
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
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}
          </div>

          <div>
            {show[account.type].expensetype ? (
              <TextField
                select
                id="expensetype"
                label="Expense Type"
                className={classNames(classes.margin, classes.textField)}
                value={account.expensetype}
                onChange={(event) => {
                  this.handleChange('expensetype', event.target.value);
                }}
              >
                {expenseTypeOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </TextField>
            ) : null}
            {show[account.type].expensetype ? (
              <Tooltip
                title={(
                  <List>
                    {expenseTypeOptions.map(option => (
                      <ListItem key={`expenseTypeToolTip${option.value}`}>
                        {option.value}
                        -
                        {option.description}
                      </ListItem>
                    ))}
                  </List>
                )}
              >
                <HelpIcon />
              </Tooltip>
            ) : null}

            {show[account.type].expensevalue ? (
              <Tooltip title="Yearly cost of the expense">
                <TextField
                  id="expensevalue"
                  label="Expense Amount"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.expensevalue}
                  InputProps={{ inputComponent: NumberFormatDollar }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}


            {show[account.type].notes ? (
              <TextField
                id="notes"
                label="Notes"
                className={classes.textArrayField}
                InputProps={{
                  classes: {
                    input: classes.textArrayFont,
                  },
                }}
                multiline
                rows="8"
                value={account.notes}
                onChange={(event) => {
                  this.handleChange(event.target.id, event.target.value);
                }}
              />
            ) : null}

          </div>
        </CardContent>
      </Card>
    );
  }
}

Account.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  account: PropTypes.object.isRequired,
  incomeAccounts: PropTypes.arrayOf(PropTypes.object).isRequired,
  onDelete: PropTypes.func.isRequired,
  onUpdate: PropTypes.func.isRequired,
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
# account.endin : Calendar year when money no longer goes to this account (this is inclusive so it will generally be yearRetire-1)
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
