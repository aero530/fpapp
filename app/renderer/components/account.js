// cSpell: ignore Formaters, autofill, reorderable

/**
 * @class Account
 * @description Shows account page with inputs based on account type
 * @property {Object} classes material ui class
 * @property {Object} account account to show
 * @property {Array} incomeAccounts list of income accounts - used to populate dropdown list
 * @property {Array} hsaAccounts list of hsa accounts - used to populate dropdown list
 * @property {Function} onDelete function to call when this account is deleted
 * @property {Function} onUpdate function to call when this account is updated
 */

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
import Checkbox from '@material-ui/core/Checkbox';
import FormGroup from '@material-ui/core/FormGroup';
import FormControlLabel from '@material-ui/core/FormControlLabel';

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
} from '../constants/autofillTypeOptions';

import { show } from '../constants/accountStructure';
import { percentSuggestions, yearSuggestions } from '../constants/suggestions';

const styles = theme => ({
  root: {
    width: '100%',
    marginTop: theme.spacing(3),
  },
  paper: {
    width: '100%',
    backgroundColor: theme.palette.background.paper,
    boxShadow: theme.shadows[5],
    padding: theme.spacing(4),
    marginBottom: theme.spacing(4),
    overflow: 'visible',
  },
  margin: {
    margin: theme.spacing(1),
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
    bottom: theme.spacing(0),
    right: theme.spacing(0),
  },
});


class Account extends Component {
  constructor(props, context) {
    super(props, context);
    // initialize the state
    this.state = {
      account: { ...props.account },
      dialogOpen: false,
    };
  }

  handleDialogOpen = () => {
    this.setState({ dialogOpen: true });
  };

  handleDialogClose = () => {
    this.setState({ dialogOpen: false });
  };

  handleDeleteTrue = () => {
    const { onDelete } = this.props;
    this.handleDialogClose();
    onDelete();
  };

  /**
   * @function handleChange
   * @description when an input is changed, update the account value in state and call the onUpdate function provided to the account class.
   * @param {string} fieldNameInput name of field to update
   * @param {string} fieldValueInput new field value
   */
  handleChange = (fieldNameInput, fieldValueInput) => {
    const { onUpdate } = this.props;
    const { account } = this.state;
    const newAccount = { ...account, [fieldNameInput]: fieldValueInput };
    this.setState({
      account: newAccount,
    });
    onUpdate(newAccount);
  };


  /**
   * @function makeTableArray
   * @description Reorder the data coming from the analysis into something the table editor can use.
   * @param {Object} ObjectOfObjects Incoming data is an object of objects.  The primary key is the name/id of the account.
   * @returns {Array} Array of objects. Each object in the array represents data for one year. [{ year: 2025, account1: 3416, account2: 9846 }, { year: 2026, account1: 3877, account2: 6797 }]
   */
  makeTableArray = (ObjectOfObjects) => {
    const table = []; // returned value
    // The internally used tableObj is structured as key value pairs with the
    // key equal to the year and the value equal to the dollar amount value.
    // { "account1": {2017: 3416, 2018: 253672}, "account2": {2017: 3215, 2018: 9846} }
    const tableObj = {};
    //console.log(ObjectOfObjects);

    Object.keys(ObjectOfObjects).forEach((accountName) => {
      const object = ObjectOfObjects[accountName];
      if (typeof object === 'object') {
        Object.keys(object).sort((a, b) => a - b).forEach((yearKey) => {
          if (Object.hasOwnProperty.call(tableObj, yearKey)) {
            tableObj[yearKey] = { ...tableObj[yearKey], [accountName]: object[yearKey] };
          } else {
            tableObj[yearKey] = { [accountName]: object[yearKey] };
          }
        });
      }
    });
    Object.keys(tableObj).sort((a, b) => a - b).forEach((key) => {
      table.push({ year: key, ...tableObj[key] });
    });

    return table;
  }

  /**
   * @function arrayToObject
   * @description Reorder the data from an array to an object.
   * @param {Array} array Input must be an array of objects.  Each object must have a 'year' and 'value' key.
   * @returns {Object} object representation of the array 'value' by 'year'
   */
  arrayToObject = (array) => {
    const obj = {};
    array.forEach((row) => {
      obj[row.year] = row.value;
    });
    return obj;
  };

  render() {
    const { classes, incomeAccounts, hsaAccounts } = this.props;
    const { account, dialogOpen } = this.state;

    /**
     * @const {Array}
     * @description column info used to generate table of year / value pairs
     */
    const colSpecValue = [
      { title: 'Year', fieldName: 'year', inputType: 'TextField', width: 200 },
      { title: 'Value', fieldName: 'value', inputType: 'TextField', width: 200 },
    ];

    /**
     * @const {Array}
     * @description column info used to generate table of year, value, contribution, earnings
     */
    const colSpecCollege = [
      { title: 'Year', fieldName: 'year', inputType: 'TextField', width: 100 },
      { title: 'Value', fieldName: 'value', inputType: 'TextField', width: 100 },
      { title: 'Contribution', fieldName: 'contribution', inputType: 'TextField', width: 100 },
      { title: 'Earnings', fieldName: 'earnings', inputType: 'TextField', width: 100 },
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
          open={dialogOpen}
          onClose={this.handleDialogClose}
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

            {show[account.type].incomeLink ? (
              <Select
                inputProps={{
                  name: 'incomeLink',
                  id: 'incomeLink',
                }}
                label="Income Link"
                className={classNames(classes.margin, classes.textField)}
                value={account.incomeLink ? account.incomeLink : 'none'}
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
            {show[account.type].incomeLink ? (
              <Tooltip title="Link this account to an income source">
                <HelpIcon />
              </Tooltip>
            ) : null}


          </div>

          <div>
            {show[account.type].isHealthcare ? (
              <FormGroup row>
                <Tooltip title="This expense account is for healthcare costs.  If so it can be linked to pull first from an HSA account.">
                  <FormControlLabel
                    control={(
                      <Checkbox
                        id="isHealthcare"
                        value="isHealthcare"
                        checked={account.isHealthcare}
                        onChange={(event) => {
                          this.handleChange(event.target.id, event.target.checked);
                        }}
                      />
                    )}
                    label="Healthcare expense"
                  />
                </Tooltip>
                {account.isHealthcare ? (
                  <FormControlLabel
                    control={(
                      <Select
                        inputProps={{
                          name: 'hsaLink',
                          id: 'hsaLink',
                        }}
                        label="HSA Link"
                        className={classNames(classes.margin, classes.textField)}
                        value={account.hsaLink ? account.hsaLink : 'none'}
                        onChange={(event) => {
                          this.handleChange(event.target.name, event.target.value);
                        }}
                      >
                        {hsaAccounts.map(option => (
                          <MenuItem key={option.value} value={option.value}>
                            {option.label}
                          </MenuItem>
                        ))}
                      </Select>
                    )}
                    label={(
                      <Tooltip title="Link this account to an income source">
                        <HelpIcon />
                      </Tooltip>
                    )}
                  />
                ) : null}
              </FormGroup>

            ) : null}

          </div>


          <div>
            {show[account.type].employerMatch ? (
              <Tooltip title="Percent of your contribution that your employer matches">
                <TextField
                  id="employerMatch"
                  label="Employer Match %"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.employerMatch}
                  disabled={
                    account.incomeLink === 'none' || !('incomeLink' in account)
                  }
                  InputProps={{ inputComponent: NumberFormatPercentage }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}

            {show[account.type].matchLimit ? (
              <Tooltip title="Employer match applies to up to this percentage of your base income">
                <TextField
                  id="matchLimit"
                  label="Match Limit %"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.matchLimit}
                  disabled={
                    account.incomeLink === 'none' || !('incomeLink' in account)
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
            {show[account.type].startIn ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={(typeof account.startIn === 'number') ? account.startIn.toString() : account.startIn}
                id="startIn"
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

            {show[account.type].endIn ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={(typeof account.endIn === 'number') ? account.endIn.toString() : account.endIn}
                id="endIn"
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
            {show[account.type].startOut ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={(typeof account.startOut === 'number') ? account.startOut.toString() : account.startOut}
                id="startOut"
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
            {show[account.type].endOut ? (
              <SuggestedInput
                className={classNames(classes.margin, classes.textField)}
                value={(typeof account.endOut === 'number') ? account.endOut.toString() : account.endOut}
                id="endOut"
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
            {show[account.type].percentTaxed ? (
              <Tooltip title="Percent of this income that will be taxed">
                <TextField
                  id="percentTaxed"
                  label="Percent Taxed"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.percentTaxed}
                  InputProps={{ inputComponent: NumberFormatPercentage }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}
          </div>

          <div>
            {show[account.type].yearlyContribution ? (
              <Tooltip title="Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage.">
                <TextField
                  id="yearlyContribution"
                  label="Yearly Contribution"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.yearlyContribution}
                  InputProps={
                    'incomeLink' in account && account.incomeLink !== 'none'
                      ? { inputComponent: NumberFormatDollarPercentage }
                      : { inputComponent: NumberFormatDollar }
                  }
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}

            {show[account.type].contributionType ? (
              <TextField
                select
                id="contributionType"
                label="Contribution Type"
                className={classNames(classes.margin, classes.textField)}
                value={account.contributionType}
                onChange={(event) => {
                  this.handleChange('contributionType', event.target.value);
                }}
              >
                {contributionTypeOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </TextField>
            ) : null}
            {show[account.type].contributionType ? (
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

            {show[account.type].yearlyReturn ? (
              <Tooltip title="Percent interest earned each year">
                <TextField
                  id="yearlyReturn"
                  label="Interest %"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.yearlyReturn}
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
                value={(typeof account.raise === 'number') ? account.raise.toString() : account.raise}
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
                {(account.type === 'college' || account.type === 'retirement') ? (
                  <MuiEditableTable
                    colSpec={colSpecCollege}
                    rowData={this.makeTableArray({ value: account.table, contribution: account.contribution, earnings: account.earnings })}
                    onChange={onAccountTableChange}
                    reorderable
                  />
                ) : (
                  <MuiEditableTable
                    colSpec={colSpecValue}
                    rowData={this.makeTableArray({ value: account.table })}
                    onChange={onAccountTableChange}
                    reorderable
                  />
                )}
              </div>
            ) : null}
          </div>

          <div>
            {show[account.type].withdrawalType ? (
              <TextField
                select
                id="withdrawalType"
                label="Withdrawal Type"
                className={classNames(classes.margin, classes.textField)}
                value={account.withdrawalType}
                onChange={(event) => {
                  this.handleChange('withdrawalType', event.target.value);
                }}
              >
                {withdrawalTypeOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </TextField>
            ) : null}
            {show[account.type].withdrawalType ? (
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

            {show[account.type].withdrawalValue ? (
              <Tooltip title="How much money should be take out per year (either as a percentage or a fixed dollar amount)">
                <TextField
                  id="withdrawalValue"
                  label="Withdrawal Amount"
                  disabled={account.withdrawalType === 'end_at_zero' || account.withdrawalType === 'col_frac_of_savings'}
                  className={classNames(classes.margin, classes.textField)}
                  value={account.withdrawalValue}
                  InputProps={{ inputComponent: NumberFormatDollar }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}
          </div>

          <div>
            {show[account.type].paymentType ? (
              <TextField
                select
                id="paymentType"
                label="Payment Type"
                className={classNames(classes.margin, classes.textField)}
                value={account.paymentType}
                onChange={(event) => {
                  this.handleChange('paymentType', event.target.value);
                }}
              >
                {paymentTypeOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </TextField>
            ) : null}
            {show[account.type].paymentType ? (
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

            {show[account.type].paymentValue ? (
              <Tooltip title="How much money should be payed each year (either as a percentage or a fixed dollar amount)">
                <TextField
                  id="paymentValue"
                  label="Payment Amount"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.paymentValue}
                  InputProps={{ inputComponent: NumberFormatDollar }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}
          </div>

          <div>
            {show[account.type].taxStatus ? (
              <TextField
                select
                label="Tax Status"
                className={classNames(classes.margin, classes.textField)}
                value={account.taxStatus}
                onChange={(event) => {
                  this.handleChange('taxStatus', event.target.value);
                }}
              >
                {taxStatusTypeOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </TextField>
            ) : null}
            {show[account.type].taxStatus ? (
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
            {show[account.type].compoundTime ? (
              <Tooltip title="Number of times per year that interest is compounded. (1=yearly, 12=monthly) Used for MORTGAGE account types.">
                <TextField
                  id="compoundTime"
                  label="Compound Time"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.compoundTime}
                  InputProps={{ inputComponent: NumberFormatYear }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}
            {show[account.type].mortgageInsurance ? (
              <Tooltip title="Mortgage insurance payment expressed as a yearly fixed number in todays dollars">
                <TextField
                  id="mortgageInsurance"
                  label="Mortgage Insurance"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.mortgageInsurance}
                  InputProps={{ inputComponent: NumberFormatDollar }}
                  onChange={(event) => {
                    this.handleChange(event.target.id, event.target.floatValue);
                  }}
                />
              </Tooltip>
            ) : null}
            {show[account.type].ltvLimit ? (
              <Tooltip title="Loan to Value amount when mortgage insurance is no longer pulled from payment.  Since monthly payment does not change over time, after the insurance is done there is more money going to the principal each payment">
                <TextField
                  id="ltvLimit"
                  label="Loan to Value"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.ltvLimit}
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
            {show[account.type].expenseType ? (
              <TextField
                select
                id="expenseType"
                label="Expense Type"
                className={classNames(classes.margin, classes.textField)}
                value={account.expenseType}
                onChange={(event) => {
                  this.handleChange('expenseType', event.target.value);
                }}
              >
                {expenseTypeOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </TextField>
            ) : null}
            {show[account.type].expenseType ? (
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

            {show[account.type].expenseValue ? (
              <Tooltip title="Yearly cost of the expense">
                <TextField
                  id="expenseValue"
                  label="Expense Amount"
                  className={classNames(classes.margin, classes.textField)}
                  value={account.expenseValue}
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
  hsaAccounts: PropTypes.arrayOf(PropTypes.object).isRequired,
  onDelete: PropTypes.func.isRequired,
  onUpdate: PropTypes.func.isRequired,
};

Account.defaultProps = {};

export default withStyles(styles)(Account);
