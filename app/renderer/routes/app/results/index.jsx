import React from 'react';
import compose from 'recompose/compose';
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

const styles = theme => ({
  root: {
    width: '100%'
  },
  paper: {
    width: '100%',
    backgroundColor: theme.palette.background.paper,
    boxShadow: theme.shadows[5],
    padding: theme.spacing.unit * 4,
    marginBottom: theme.spacing.unit * 4,
    marginTop: theme.spacing.unit * 3,
    overflow: 'auto'
  },
  button: {
    margin: theme.spacing.unit
  }
});

// Year    Net     Income  Taxable Income  Income After Taxes      Expenses        Total Savings (retirement, college, etc)

// 2017      -610.13       133636.00       118917.84       109852.43       110462.56       234411.32
// 2018      -173.43       137645.08       122485.38       113148.00       112711.30       260961.04
// 2019      3728.02       141774.43       126159.94       116542.45       112641.00       289283.13
// 2020      9966.59       146027.67       129944.73       120038.72       113800.15       319479.54
// 2021     17361.20       150408.50       133843.08       123639.88       116245.27       351657.74
// 2022     25934.14       154920.75       137858.37       127349.08       118776.13       385930.97
// 2023     31422.06       154195.14       136620.88       126870.96       121383.04       422418.57
// 2024     39874.84       158820.99       140719.51       130677.09       122224.30       461246.29
// 2025     49598.94       163585.62       144941.10       134597.40       124873.31       502546.60
// 2026     60516.40       168493.19       149289.33       138635.32       127717.87       546459.06
// 2027     64057.89       162796.65       143016.68       134193.32       130651.82       593130.70

class Results extends React.Component {
  constructor(props) {
    super(props);

    this.state = {};

    props.setAppBarTitle('Results');
  }

  cumulativeSum = a => {
    console.log(a);
    let result = a[0];
    for (let i = 1; i < a.length; i += 1) {
      result += a[i];
    }
    return result;
  };

  render() {
    const {
      classes,
      accounts,
      savings,
      expenses,
      incomeTaxable,
      incomeTotal,
      incomeAfterTax,
      net,
      year
    } = this.props;

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
                <TableCell numeric>
                  Total Savings (retirement, college, etc)
                </TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {year.map((yearValue, index) => {
                return (
                  <TableRow key={yearValue}>
                    <TableCell component="th" scope="row">
                      {yearValue}
                    </TableCell>
                    <TableCell numeric>
                      {net[index].toLocaleString('en-US', {
                        maximumFractionDigits: 0
                      })}
                    </TableCell>
                    <TableCell numeric>
                      {incomeTotal[index].toLocaleString('en-US', {
                        maximumFractionDigits: 0
                      })}
                    </TableCell>
                    <TableCell numeric>
                      {incomeTaxable[index].toLocaleString('en-US', {
                        maximumFractionDigits: 0
                      })}
                    </TableCell>
                    <TableCell numeric>
                      {incomeAfterTax[index].toLocaleString('en-US', {
                        maximumFractionDigits: 0
                      })}
                    </TableCell>
                    <TableCell numeric>
                      {this.cumulativeSum(
                        Object.values(expenses[index])
                      ).toLocaleString('en-US', { maximumFractionDigits: 0 })}
                    </TableCell>
                    <TableCell numeric>
                      {savings[index].toLocaleString('en-US', {
                        maximumFractionDigits: 0
                      })}
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
  setAppBarTitle: PropTypes.func.isRequired
};

Results.defaultProps = {};

const mapStateToProps = state => ({
  accounts: state.results.accounts,
  savings: state.results.savings,
  expenses: state.results.expenses,
  incomeTaxable: state.results.incomeTaxable,
  incomeTotal: state.results.incomeTotal,
  incomeAfterTax: state.results.incomeAfterTax,
  net: state.results.net,
  year: state.results.year
});

const mapDispatchToProps = dispatch => ({
  setAppBarTitle: titleInput =>
    dispatch({ type: SET_APP_BAR_TITLE, title: titleInput })
});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps
  )
)(Results);
