import React from 'react';
import compose from 'recompose/compose';
import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import {
  Area,
  ResponsiveContainer,
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ComposedChart,
} from 'recharts';

import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

import * as ResultsActions from '../../../actions/results';

import { SET_APP_BAR_TITLE } from '../../../actions/app';

const colors = ['#e91e63', '#2196f3', '#4caf50', '#ff9800', '#9c27b0', '#cddc39', '#ff5722', '#009688', '#ffeb3b'];

const chartHeight = 600;

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
  },
  button: {
    margin: theme.spacing.unit,
  },
});


class GraphsDetail extends React.Component {
  componentDidMount() {
    const { setAppBarTitle, analyze } = this.props;
    setAppBarTitle('Detail Graphs');
    analyze();
  }

  numericSort = (key, dir) => (a, b) => {
    if (a[key] > b[key]) {
      return -1 * dir;
    }
    if (b[key] > a[key]) {
      return 1 * dir;
    }
    return 0;
  };

  // dataIn = [{name: "dataset name", data: account.payment}, {}, {}]
  formatDataObjects = (accounts) => {
    // dataIn is array of data objects
    const output = [];
    const years = Object.keys(accounts[0].data).sort((a, b) => a - b);

    years.forEach((year) => {
      let row = { x: year };
      accounts.forEach((account) => {
        row = { ...row, [account.name]: account.data[year] };
      });
      output.push(row);
    });

    return output;
  };

  cumulativeSum = (inputObject) => {
    const output = {};
    let total = 0;
    Object.keys(inputObject).sort((a, b) => a - b).forEach((key) => {
      total += inputObject[key];
      output[key] = total;
    });
    return output;
  };

  objectSubtract = (a, b) => {
    const output = {};
    Object.keys(a).forEach((key) => {
      output[key] = a[key] - b[key];
    });
    return output;
  };

  arraySum = (input) => {
    let output = 0;
    input.forEach((value) => {
      output += value;
    });
    return output;
  };


  render() {
    const {
      classes,
      accounts,
      yearTable,
    } = this.props;

    const yearStart = yearTable[0];

    if (yearTable.length > 0) {
      return (
        <div key="charts">

          <Paper className={classes.paper}>
            <Typography variant="title" id="modal-title">Retirement Accounts</Typography>
            {Object.values(accounts).map((account) => {
              if (account.type === 'retirement') {
                return (
                  <div key={`charts-retirement-${account.name}`}>
                    <Typography variant="subheading" id="chart-title" align="center">
                      {account.name}
                    </Typography>
                    <ResponsiveContainer width="100%" height={chartHeight}>
                      <ComposedChart
                        data={this.formatDataObjects([
                          { name: 'Account Value', data: account.table },
                          { name: 'Cumulative Contribution', data: this.cumulativeSum(account.contribution) },
                          { name: 'Cumulative Earnings', data: this.cumulativeSum(account.earnings) },
                          { name: 'Contribution', data: account.contribution },
                          { name: 'Earnings', data: account.earnings },
                        ])}
                        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                      >
                        <XAxis dataKey="x" />
                        <YAxis />
                        <CartesianGrid strokeDasharray="3 3" />
                        <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />

                        <Area key={`charts-retirement-${account.name}-area-1`} type="linear" stackId="1" dataKey="Cumulative Contribution" stroke={colors[1]} fill={colors[1]} />
                        <Area key={`charts-retirement-${account.name}-area-2`} type="linear" stackId="1" dataKey="Cumulative Earnings" stroke={colors[2]} fill={colors[2]} />

                        <Line key={`charts-retirement-${account.name}-line-1`} type="linear" dataKey="Contribution" stroke={colors[1]} strokeWidth="2" dot={false} />
                        <Line key={`charts-retirement-${account.name}-line-2`} type="linear" dataKey="Earnings" stroke={colors[2]} strokeWidth="2" dot={false} />

                        <Line type="linear" dataKey="Account Value" stroke={colors[0]} strokeWidth="2" dot={false} />
                      </ComposedChart>
                    </ResponsiveContainer>
                  </div>
                );
              }
              return null;
            })}
          </Paper>

          <Paper className={classes.paper}>
            <Typography variant="title" id="modal-title">College Savings</Typography>
            {Object.values(accounts).map((account) => {
              if (account.type === 'college') {
                return (
                  <div key={`charts-college-${account.name}`}>
                    <Typography variant="subheading" id="chart-title" align="center">
                      {account.name}
                    </Typography>
                    <ResponsiveContainer width="100%" height={chartHeight}>
                      <ComposedChart
                        data={this.formatDataObjects([
                          { name: 'Account Value', data: account.table },
                          { name: 'Cumulative Contribution', data: this.cumulativeSum(account.contribution) },
                          { name: 'Cumulative Earnings', data: this.cumulativeSum(account.earnings) },
                          { name: 'Contribution', data: account.contribution },
                          { name: 'Earnings', data: account.earnings },
                        ])}
                        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                      >
                        <XAxis dataKey="x" />
                        <YAxis />
                        <CartesianGrid strokeDasharray="3 3" />
                        <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />

                        <Area key={`charts-college-${account.name}-area-1`} type="linear" stackId="1" dataKey="Cumulative Contribution" stroke={colors[1]} fill={colors[1]} />
                        <Area key={`charts-college-${account.name}-area-2`} type="linear" stackId="1" dataKey="Cumulative Earnings" stroke={colors[2]} fill={colors[2]} />

                        <Line key={`charts-college-${account.name}-line-1`} type="linear" dataKey="Contribution" stroke={colors[1]} strokeWidth="2" dot={false} />
                        <Line key={`charts-college-${account.name}-line-2`} type="linear" dataKey="Earnings" stroke={colors[2]} strokeWidth="2" dot={false} />

                        <Line type="linear" dataKey="Account Value" stroke={colors[0]} strokeWidth="2" dot={false} />
                      </ComposedChart>
                    </ResponsiveContainer>
                  </div>
                );
              }
              return null;
            })}
          </Paper>

          <Paper className={classes.paper}>
            <Typography variant="title" id="modal-title">Mortgage</Typography>
            {Object.values(accounts).map((account) => {
              if (account.type === 'mortgage') {
                return (
                  <div key={`charts-mortgage-${account.name}`}>
                    <Typography variant="subheading" id="chart-title" align="center">
                      {account.name}
                    </Typography>
                    <ResponsiveContainer width="100%" height={chartHeight}>
                      <LineChart
                        data={this.formatDataObjects([
                          { name: 'Principal', data: account.table },
                          { name: 'Payment', data: account.payment },
                          { name: `Cumulative Payment since ${yearStart}`, data: this.objectSubtract(this.cumulativeSum(account.payment), this.cumulativeSum(account.escrow)) },
                        ])}
                        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                      >
                        <XAxis dataKey="x" />
                        <YAxis />
                        <CartesianGrid strokeDasharray="3 3" />
                        <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                        <Line type="monotone" dataKey="Principal" stroke={colors[0]} dot={false} />
                        <Line type="monotone" dataKey="Payment" stroke={colors[1]} dot={false} />
                        <Line type="monotone" dataKey={`Cumulative Payment since ${yearStart}`} stroke={colors[2]} dot={false} />
                      </LineChart>
                    </ResponsiveContainer>

                  </div>
                );
              }
              return null;
            })}
          </Paper>

          <Paper className={classes.paper}>
            <Typography variant="title" id="modal-title">Loan</Typography>
            {Object.values(accounts).map((account) => {
              if (account.type === 'loan') {
                return (
                  <div key={`charts-loan-${account.name}`}>
                    <Typography variant="subheading" id="chart-title" align="center">
                      {account.name}
                    </Typography>
                    <ResponsiveContainer width="100%" height={chartHeight}>
                      <LineChart
                        data={this.formatDataObjects([
                          { name: 'Principal', data: account.table },
                          { name: 'Payment', data: account.payment },
                          { name: `Cumulative Payment since ${yearStart}`, data: this.cumulativeSum(account.payment) },
                        ])}
                        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                      >
                        <XAxis dataKey="x" />
                        <YAxis />
                        <CartesianGrid strokeDasharray="3 3" />
                        <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                        <Line type="monotone" dataKey="Principal" stroke={colors[0]} dot={false} />
                        <Line type="monotone" dataKey="Payment" stroke={colors[1]} dot={false} />
                        <Line type="monotone" dataKey={`Cumulative Payment since ${yearStart}`} stroke={colors[2]} dot={false} />
                      </LineChart>
                    </ResponsiveContainer>
                  </div>
                );
              }
              return null;
            })}
          </Paper>


          <Paper className={classes.paper}>
            <Typography variant="title" id="modal-title">HSA</Typography>
            {Object.values(accounts).map((account) => {
              if (account.type === 'hsa') {
                return (
                  <div key={`charts-hsa-${account.name}`}>
                    <Typography variant="subheading" id="chart-title" align="center">
                      {account.name}
                    </Typography>
                    <ResponsiveContainer width="100%" height={chartHeight}>
                      <LineChart
                        data={this.formatDataObjects([
                          { name: 'Contribution', data: account.contribution },
                          { name: 'Earnings', data: account.earnings },
                          { name: 'Value', data: account.table },
                          { name: 'Withdrawal', data: account.withdrawal },
                        ])}
                        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                      >
                        <XAxis dataKey="x" />
                        <YAxis />
                        <CartesianGrid strokeDasharray="3 3" />
                        <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                        <Line type="monotone" dataKey="Value" stroke={colors[2]} dot={false} />
                        <Line type="monotone" dataKey="Contribution" stroke={colors[0]} dot={false} />
                        <Line type="monotone" dataKey="Earnings" stroke={colors[1]} dot={false} />
                        <Line type="monotone" dataKey="Withdrawal" stroke={colors[3]} dot={false} />
                      </LineChart>
                    </ResponsiveContainer>
                  </div>
                );
              }
              return null;
            })}
          </Paper>
        </div>
      );
    }
    return (
      <Paper className={classes.paper}>
        <Typography variant="body1" id="chart-title" align="center">
          Load some data to make some graphs.
        </Typography>
      </Paper>
    );
  }
}

GraphsDetail.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
  accounts: PropTypes.objectOf(PropTypes.object).isRequired,
  analyze: PropTypes.func.isRequired,
  yearTable: PropTypes.arrayOf(PropTypes.number),
};

GraphsDetail.defaultProps = {
  yearTable: [],
};

const mapStateToProps = state => ({
  accounts: state.results.accounts,
  yearTable: state.results.year,
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
)(GraphsDetail);
