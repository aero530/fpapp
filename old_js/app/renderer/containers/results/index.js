import React from 'react';
import compose from 'recompose/compose';
import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import Paper from '@material-ui/core/Paper';

import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

import { SET_APP_BAR_TITLE } from '../../actions/app';
import * as ResultsActions from '../../actions/results';

import { arraySum } from '../../utils';

const styles = theme => ({
  root: {
    width: '100%',
  },
  paper: {
    width: '100%',
    backgroundColor: theme.palette.background.paper,
    boxShadow: theme.shadows[5],
    padding: theme.spacing(4),
    marginBottom: theme.spacing(4),
    marginTop: theme.spacing(3),
    overflow: 'auto',
  },
  button: {
    margin: theme.spacing(1),
  },
});

class Results extends React.Component {
  componentDidMount() {
    const { setAppBarTitle, analyze } = this.props;
    setAppBarTitle('Results');
    analyze();
  }

  render() {
    const { classes, savings, expenses, incomeTaxable, incomeTotal, incomeAfterTax, net, year } = this.props;

    return (
      <div>
        <Paper className={classes.paper}>
          <Table className={classes.table} size="small">
            <TableHead>
              <TableRow>
                <TableCell align="right">Year</TableCell>
                <TableCell align="right">Net</TableCell>
                <TableCell align="right">Income</TableCell>
                <TableCell align="right">Taxable Income</TableCell>
                <TableCell align="right">Income After Taxes</TableCell>
                <TableCell align="right">Expenses</TableCell>
                <TableCell align="right">Total Savings (retirement, college, etc)</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {year.map(yearValue => (
                <TableRow key={yearValue} hover>
                  <TableCell component="th" scope="row">
                    {yearValue}
                  </TableCell>
                  <TableCell align="right">
                    {net[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                  </TableCell>
                  <TableCell align="right">
                    {incomeTotal[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                  </TableCell>
                  <TableCell align="right">
                    {incomeTaxable[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                  </TableCell>
                  <TableCell align="right">
                    {incomeAfterTax[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                  </TableCell>
                  <TableCell align="right">
                    {arraySum(Object.values(expenses[yearValue])).toLocaleString('en-US', { maximumFractionDigits: 0 })}
                  </TableCell>
                  <TableCell align="right">
                    {savings[yearValue].toLocaleString('en-US', { maximumFractionDigits: 0 })}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </Paper>
      </div>
    );
  }
}

Results.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
  analyze: PropTypes.func.isRequired,
  savings: PropTypes.objectOf(PropTypes.number),
  expenses: PropTypes.objectOf(PropTypes.object),
  incomeTaxable: PropTypes.objectOf(PropTypes.number),
  incomeTotal: PropTypes.objectOf(PropTypes.number),
  incomeAfterTax: PropTypes.objectOf(PropTypes.number),
  net: PropTypes.objectOf(PropTypes.number),
  year: PropTypes.arrayOf(PropTypes.number),
};

Results.defaultProps = {
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
