import React from 'react';
import compose from 'recompose/compose';
import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import classNames from 'classnames';
import * as ResultsActions from '../../../actions/results';

import { AreaChart, Area, ResponsiveContainer, LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from 'recharts';

import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

import { SET_APP_BAR_TITLE } from '../../../actions/app';

const chartHeight = 300;

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
  button: {
    margin: theme.spacing.unit
  }
});

class Graphs extends React.Component {
  componentDidMount() {
    const { setAppBarTitle, analyze } = this.props;
    setAppBarTitle('Graphs');
    analyze();
  }

  formatData = (years, dataIn) => {
    const output = [];
    years.forEach((year, index) => {
      output.push({ x: year, y: dataIn[index] });
    });
    return output;
  };

  formatDataObject = dataIn => {
    const output = [];
    const years = Object.keys(dataIn).sort((a, b) => {
      return a - b;
    });
    years.forEach(year => {
      output.push({ x: year, y: dataIn[year] });
    });
    return output;
  };

  cumulativeSum = a => {
    const result = [a[0]];
    for (let i = 1; i < a.length; i += 1) {
      result[i] = result[i - 1] + a[i];
    }
    return result;
  };

  arraySubtract = (a, b) => {
    const x = a.map((item, index) => {
      return item - b[index];
    });
    return x;
  };

  render() {
    const { classes, accounts, savings, expenses, incomeTaxable, incomeTotal, incomeAfterTax, net, year } = this.props;

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
              <Tooltip />
              <Line type="monotone" dataKey="y" stroke="#8884d8" dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </Paper>

        <Paper className={classes.paper}>
          <Typography variant="title" id="modal-title">
            College
          </Typography>
          {Object.values(accounts).map(account => {
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
                      <Tooltip />
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
          <Typography variant="title" id="modal-title">
            Mortgage
          </Typography>
          {Object.values(accounts).map(account => {
            if (account.type === 'mortgage') {
              return (
                <div>
                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name} Mortgage
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart data={this.formatDataObject(account.table)} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip />
                      <Line type="monotone" dataKey="y" stroke="#8884d8" dot={false} />
                    </LineChart>
                  </ResponsiveContainer>

                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name} Payment
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart data={this.formatData(year, account.payment)} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip />
                      <Line type="monotone" dataKey="y" stroke="#8884d8" dot={false} />
                    </LineChart>
                  </ResponsiveContainer>

                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name} Cumulative Payment
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart
                      data={this.formatData(year, this.arraySubtract(this.cumulativeSum(account.payment), this.cumulativeSum(account.escrow)))}
                      margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                    >
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip />
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
          <Typography variant="title" id="modal-title">
            Loan
          </Typography>
          {Object.values(accounts).map(account => {
            if (account.type === 'loan') {
              return (
                <div>
                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name} Loan Value
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart data={this.formatDataObject(account.table)} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip />
                      <Line type="monotone" dataKey="y" stroke="#8884d8" dot={false} />
                    </LineChart>
                  </ResponsiveContainer>

                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name} Yearly Payment
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart data={this.formatData(year, account.payment)} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip />
                      <Line type="monotone" dataKey="y" stroke="#8884d8" dot={false} />
                    </LineChart>
                  </ResponsiveContainer>

                  <Typography variant="subheading" id="chart-title" align="center">
                    {account.name} Cumulative Payment
                  </Typography>
                  <ResponsiveContainer width="100%" height={chartHeight}>
                    <LineChart data={this.formatData(year, this.cumulativeSum(account.payment))} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                      <XAxis dataKey="x" />
                      <YAxis />
                      <CartesianGrid strokeDasharray="3 3" />
                      <Tooltip />
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
          <span>{JSON.stringify(savings, undefined, 2)}</span>
        </Paper>
        <Paper className={classes.paper}>
          <span>{JSON.stringify(expenses, undefined, 2)}</span>
        </Paper>
        <Paper className={classes.paper}>
          <span>{JSON.stringify(incomeTaxable, undefined, 2)}</span>
        </Paper>
        <Paper className={classes.paper}>
          <span>{JSON.stringify(incomeTotal, undefined, 2)}</span>
        </Paper>
        <Paper className={classes.paper}>
          <span>{JSON.stringify(incomeAfterTax, undefined, 2)}</span>
        </Paper>
        <Paper className={classes.paper}>
          <span>{JSON.stringify(net, undefined, 2)}</span>
        </Paper>
      </div>
    );
  }
}

Graphs.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired
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
  year: state.results.year
});

const mapDispatchToProps = dispatch => ({
  setAppBarTitle: titleInput => dispatch({ type: SET_APP_BAR_TITLE, title: titleInput }),
  ...bindActionCreators(ResultsActions, dispatch)
});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps
  )
)(Graphs);
