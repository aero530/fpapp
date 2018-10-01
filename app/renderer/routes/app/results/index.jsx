import React from 'react';
import compose from 'recompose/compose';
import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import classNames from 'classnames';

import Paper from '@material-ui/core/Paper';

import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

import { SET_APP_BAR_TITLE } from '../../../actions/app';
import * as ResultsActions from '../../../actions/results';

const styles = theme => ({
  root: {
    width: '100%',
  },
  paper: {
    width: '100%',
    backgroundColor: theme.palette.background.paper,
    boxShadow: theme.shadows[5],
    padding: theme.spacing.unit * 4,
    marginBottom: theme.spacing.unit * 4,
    marginTop: theme.spacing.unit * 3,
    overflow: 'auto',
  },
  button: {
    margin: theme.spacing.unit,
  },
});

class Results extends React.Component {
  componentDidMount() {
    const { setAppBarTitle, analyze } = this.props;
    setAppBarTitle('Results');
    analyze();
  }

  cumulativeSum = (a) => {
    let result = a[0];
    for (let i = 1; i < a.length; i += 1) {
      result += a[i];
    }
    return result;
  };

  render() {
    const { classes, accounts, savings, expenses, incomeTaxable, incomeTotal, incomeAfterTax, net, year } = this.props;

    return (
      <div>
        <Paper className={classes.paper}>
          <Table className={classes.table}>
            <TableHead>
              <TableRow>
                <TableCell numeric>Year</TableCell>
                <TableCell numeric>Net</TableCell>
                <TableCell numeric>Income</TableCell>
                <TableCell numeric>Taxable Income</TableCell>
                <TableCell numeric>Income After Taxes</TableCell>
                <TableCell numeric>Expenses</TableCell>
                <TableCell numeric>Total Savings (retirement, college, etc)</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {year.map((yearValue) => {
                return (
                  <TableRow key={yearValue} hover>
                    <TableCell component="th" scope="row" padding="dense">
                      {yearValue}
                    </TableCell>
                    <TableCell numeric padding="dense">
                      {net[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                    </TableCell>
                    <TableCell numeric padding="dense">
                      {incomeTotal[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                    </TableCell>
                    <TableCell numeric padding="dense">
                      {incomeTaxable[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                    </TableCell>
                    <TableCell numeric padding="dense">
                      {incomeAfterTax[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                    </TableCell>
                    <TableCell numeric padding="dense">
                      {this.cumulativeSum(Object.values(expenses[yearValue])).toLocaleString('en-US', { maximumFractionDigits: 0 })}
                    </TableCell>
                    <TableCell numeric padding="dense">
                      {savings[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                    </TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        </Paper>
        <Paper className={classes.paper}>
          <span>{JSON.stringify(net, undefined, 2)}</span>
        </Paper>
      </div>
    );
  }
}

Results.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
  accounts: PropTypes.objectOf(PropTypes.object),
  savings: PropTypes.objectOf(PropTypes.number),
  expenses: PropTypes.objectOf(PropTypes.object),
  incomeTaxable: PropTypes.objectOf(PropTypes.number),
  incomeTotal: PropTypes.objectOf(PropTypes.number),
  incomeAfterTax: PropTypes.objectOf(PropTypes.number),
  net: PropTypes.objectOf(PropTypes.number),
  year: PropTypes.arrayOf(PropTypes.number),
};

Results.defaultProps = {
  accounts: {},
  savings: {},
  expenses: {},
  incomeTaxable: {},
  incomeTotal: {},
  incomeAfterTax: {},
  net: {},
  year: [],
};

const mapStateToProps = state => ({
  accounts: state.results.accounts,
  savings: state.results.savings,
  expenses: state.results.expenses,
  incomeTaxable: state.results.incomeTaxable,
  incomeTotal: state.results.incomeTotal,
  incomeAfterTax: state.results.incomeAfterTax,
  net: state.results.net,
  year: state.results.year,
});

const mapDispatchToProps = dispatch => ({
  setAppBarTitle: titleInput => dispatch({ type: SET_APP_BAR_TITLE, title: titleInput }),
  ...bindActionCreators(ResultsActions, dispatch),
});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps,
  ),
)(Results);
