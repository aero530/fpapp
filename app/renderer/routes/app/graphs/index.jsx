import React from 'react';
import compose from 'recompose/compose';
import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import {
  AreaChart,
  Area,
  ResponsiveContainer,
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
} from 'recharts';

import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

import * as ResultsActions from '../../../actions/results';

import { SET_APP_BAR_TITLE } from '../../../actions/app';

const chartHeight = 300;

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

class Graphs extends React.Component {
  componentDidMount() {
    const { setAppBarTitle, analyze } = this.props;
    setAppBarTitle('Graphs');
    analyze();
  }

  formatDataObject = (dataIn) => {
    const output = [];
    const years = Object.keys(dataIn).sort((a, b) => a - b);
    years.forEach((year) => {
      output.push({ x: year, y: dataIn[year] });
    });
    return output;
  };

  // account.payment
  // dataIn = [{name: "asdf", data: account.payment}, {}, {}]
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
      year,
    } = this.props;

    return (
      <div>
        <Paper className={classes.paper}>
          <Typography variant="title" id="modal-title">
            Net
          </Typography>
          <ResponsiveContainer width="100%" height={chartHeight}>
            <LineChart data={this.formatDataObject(net)} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
              <XAxis dataKey="x" />
              <YAxis />
              <CartesianGrid strokeDasharray="3 3" />
              <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
              <Line type="monotone" dataKey="y" stroke="#8884d8" dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </Paper>

        <Paper className={classes.paper}>
          <Typography variant="title" id="modal-title">
            College
          </Typography>
          {Object.values(accounts).map((account) => {
            if (account.type === 'college') {
              return (
                <div>
                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name}
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart data={this.formatDataObject(account.table)} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                      <Line type="monotone" dataKey="y" stroke="#8884d8" dot={false} />
                    </LineChart>
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
                <div>
                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name} Mortgage
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart
                      data={this.formatDataObjects([
                        { name: 'Value', data: account.table },
                        { name: 'Payment', data: account.payment },
                        { name: 'Cumulative Payment', data: this.objectSubtract(this.cumulativeSum(account.payment), this.cumulativeSum(account.escrow)) },
                      ])}
                      margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                    >
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                      <Line type="monotone" dataKey="Value" stroke="#e91e63" dot={false} />
                      <Line type="monotone" dataKey="Payment" stroke="#2196f3" dot={false} />
                      <Line type="monotone" dataKey="Cumulative Payment" stroke="#4caf50" dot={false} />
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
                <div>
                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name} Loan Value
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart
                      data={this.formatDataObjects([
                        { name: 'Value', data: account.table },
                        { name: 'Payment', data: account.payment },
                        { name: 'Cumulative Payment', data: this.cumulativeSum(account.payment) },
                      ])}
                      margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                    >
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                      <Line type="monotone" dataKey="Value" stroke="#e91e63" dot={false} />
                      <Line type="monotone" dataKey="Payment" stroke="#2196f3" dot={false} />
                      <Line type="monotone" dataKey="Cumulative Payment" stroke="#4caf50" dot={false} />
                    </LineChart>
                  </ResponsiveContainer>
                </div>
              );
            }
            return null;
          })}
        </Paper>

        <Paper className={classes.paper}>
          <Typography variant="title" id="modal-title">Income</Typography>
          {Object.values(accounts).map((account) => {
            if (account.type === 'income') {
              return (
                <div>
                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name}
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart
                      data={this.formatDataObject(account.table)}
                      margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                    >
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                      <Line type="monotone" dataKey="y" stroke="#e91e63" dot={false} />
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
}

Graphs.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
};

Graphs.defaultProps = {};

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
)(Graphs);
