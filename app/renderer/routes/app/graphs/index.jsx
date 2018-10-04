import React from 'react';
import compose from 'recompose/compose';
import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import { ResponsiveTreeMapCanvas } from '@nivo/treemap';
import Slider from '@material-ui/lab/Slider';

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


class Graphs extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      sliderValue: 2020,
    };
  }

  componentDidMount() {
    const { setAppBarTitle, analyze } = this.props;
    setAppBarTitle('Graphs');
    analyze();
  }

  numericSort = (key, dir) => {
    return (a, b) => {
      if (a[key] > b[key]) {
        return -1 * dir;
      }
      if (b[key] > a[key]) {
        return 1 * dir;
      }
      return 0;
    };
  };

  formatDataObject = (dataIn) => {
    const output = [];
    const years = Object.keys(dataIn).sort((a, b) => a - b);
    years.forEach((year) => {
      output.push({ x: year, y: dataIn[year] });
    });
    return output;
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

  expenseTreeData = (year) => {
    const { accounts, expenses } = this.props;
    const yearsData = expenses[year];

    const outputTable = [];
    if (yearsData) {
      Object.keys(yearsData).forEach((accountID) => {
        outputTable.push({ name: accounts[accountID].name, amount: Math.round(yearsData[accountID]) });
      });
    } else {
      outputTable.push({ name: '', amount: 0 });
    }
    return { name: 'root', children: outputTable };
  };

  handleSliderChange = (event, sliderValue) => {
    this.setState({ sliderValue });
  };

  render() {
    const {
      classes,
      accounts,
      expenses,
      incomeTotal,
      incomeAfterTax,
      net,
      yearTable,
    } = this.props;

    const { sliderValue } = this.state;
    const yearStart = yearTable[0];
    const yearEnd = yearTable[yearTable.length - 1];

    // Create data objects for graphs

    // generate income and retirement data objects
    let incomeData = [];
    const incomeAccounts = [];
    let retirementData = [];
    const retirementAccounts = [];
    if (Object.keys(accounts).length > 0) {
      Object.values(accounts).forEach((account) => {
        if (account.type === 'income') {
          incomeAccounts.push({ name: account.name, data: account.table });
        }
      });
      incomeData = this.formatDataObjects(incomeAccounts);
      Object.values(accounts).forEach((account) => {
        if (account.type === 'retirement') {
          retirementAccounts.push({ name: account.name, data: account.table });
        }
      });
      retirementData = this.formatDataObjects(retirementAccounts);
    }

    // generate expense data objects
    const expensesTotal = {};
    const expenseData = [];
    const expenseAccounts = [];

    if (Object.keys(expenses).length > 0) {
      Object.keys(expenses).forEach((rowYear) => {
        expensesTotal[rowYear] = this.arraySum(Object.values(expenses[rowYear]));
      });

      Object.keys(expenses).forEach((rowYear) => {
        const expenseRow = {};
        expenseRow.x = rowYear;
        Object.keys(expenses[rowYear]).forEach((accountID) => {
          expenseRow[accounts[accountID].name] = expenses[rowYear][accountID];
        });
        expenseData.push(expenseRow);
      });

      Object.keys(Object.values(expenses)[0]).forEach((accountID) => {
        expenseAccounts.push({ name: accounts[accountID].name, id: accountID });
      });
    }


    // Sort expense accounts by the amount of the expense on yearStart
    expenseAccounts.sort((a, b) => {
      if (expenses[yearStart][a.id] > expenses[yearStart][b.id]) {
        return -1;
      }
      if (expenses[yearStart][b.id] > expenses[yearStart][a.id]) {
        return 1;
      }
      return 0;
    });

    return (
      <div key="charts">

        <Paper className={classes.paper}>
          <Typography variant="subheading" id="chart-title" align="center">
            Overall
          </Typography>
          <ResponsiveContainer width="100%" height={chartHeight}>
            <LineChart
              data={this.formatDataObjects([
                { name: 'Net', data: net },
                { name: 'Total Income', data: incomeTotal },
                { name: 'After Tax Income', data: incomeAfterTax },
                { name: 'Expenses', data: expensesTotal },
              ])}
              margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
            >
              <XAxis dataKey="x" />
              <YAxis />
              <CartesianGrid strokeDasharray="3 3" />
              <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
              <Line type="monotone" dataKey="Net" stroke={colors[0]} strokeWidth="2" dot={false} />
              <Line type="monotone" dataKey="Total Income" stroke={colors[1]} strokeWidth="2" dot={false} />
              <Line type="monotone" dataKey="After Tax Income" stroke={colors[2]} strokeWidth="2" dot={false} />
              <Line type="monotone" dataKey="Expenses" stroke={colors[3]} strokeWidth="2" dot={false} />
            </LineChart>
          </ResponsiveContainer>
        </Paper>

        <Paper className={classes.paper}>
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
                      <Line type="monotone" dataKey="Principal" stroke="#e91e63" dot={false} />
                      <Line type="monotone" dataKey="Payment" stroke="#2196f3" dot={false} />
                      <Line type="monotone" dataKey={`Cumulative Payment since ${yearStart}`} stroke="#4caf50" dot={false} />
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
                      <Line type="monotone" dataKey="Principal" stroke="#e91e63" dot={false} />
                      <Line type="monotone" dataKey="Payment" stroke="#2196f3" dot={false} />
                      <Line type="monotone" dataKey={`Cumulative Payment since ${yearStart}`} stroke="#4caf50" dot={false} />
                    </LineChart>
                  </ResponsiveContainer>
                </div>
              );
            }
            return null;
          })}
        </Paper>

        <Paper className={classes.paper}>
          <div key="charts-income">
            <Typography variant="subheading" id="chart-title" align="center">
              Income
            </Typography>
            <ResponsiveContainer width="100%" height={chartHeight}>
              <AreaChart
                data={incomeData}
                margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
              >
                <XAxis dataKey="x" />
                <YAxis />
                <CartesianGrid strokeDasharray="3 3" />
                <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                {incomeAccounts.map((row, index) => <Area key={`charts-income-area-${row.name}`} type="monotone" stackId="1" dataKey={row.name} stroke={colors[index % colors.length]} fill={colors[index % colors.length]} />)}
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </Paper>

        <Paper className={classes.paper}>
          <div key="charts-retirement">
            <Typography variant="subheading" id="chart-title" align="center">
              Retirement
            </Typography>
            <ResponsiveContainer width="100%" height={chartHeight}>
              <AreaChart
                data={retirementData}
                margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
              >
                <XAxis dataKey="x" />
                <YAxis />
                <CartesianGrid strokeDasharray="3 3" />
                <Tooltip formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                {retirementAccounts.map((row, index) => <Area key={`charts-retirement-area-${row.name}`} type="monotone" stackId="1" dataKey={row.name} stroke={colors[index % colors.length]} fill={colors[index % colors.length]} />)}
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </Paper>

        <Paper className={classes.paper}>
          <div key="charts-expenses">
            <Typography variant="subheading" id="chart-title" align="center">
            Expenses
            </Typography>
            <ResponsiveContainer width="100%" height={chartHeight}>
              <AreaChart
                data={expenseData}
                margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
              >
                <XAxis dataKey="x" />
                <YAxis />
                <CartesianGrid strokeDasharray="3 3" />
                <Tooltip
                  formatter={value => value.toLocaleString('en-US', { maximumFractionDigits: 0 })}
                  itemSorter={this.numericSort('value', 1)}
                />
                {expenseAccounts.map((row, index) => <Area key={`charts-expenses-area-${row.name}`} type="monotone" stackId="1" dataKey={row.name} stroke={colors[index % colors.length]} fill={colors[index % colors.length]} />)}
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </Paper>

        <Paper className={classes.paper} >
          <Typography variant="subheading" id="chart-title" align="center">
            {`${sliderValue} Expenses`}
          </Typography>
          <div style={{ height: `${chartHeight}px` }}>
            <ResponsiveTreeMapCanvas
              root={this.expenseTreeData(sliderValue)}
              leavesOnly
              innerPadding={0}
              margin={{
                top: 10,
                right: 10,
                bottom: 10,
                left: 10,
              }}
              labelSkipSize={18}
              labelTextColor="inherit:darker(1.6)"
              borderWidth={1}
              borderColor="inherit:darker(0.8)"
              colors="accent"
              colorBy="name"
              label="name"
              identity="name"
              value="amount"
            />
          </div>
          <Slider value={sliderValue} min={yearStart} max={yearEnd} step={1} onChange={this.handleSliderChange} />
        </Paper>

      </div>
    );
  }
}

Graphs.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
  accounts: PropTypes.objectOf(PropTypes.object).isRequired,
  expenses: PropTypes.objectOf(PropTypes.object).isRequired,
  incomeTotal: PropTypes.objectOf(PropTypes.number).isRequired,
  incomeAfterTax: PropTypes.objectOf(PropTypes.number).isRequired,
  net: PropTypes.objectOf(PropTypes.number).isRequired,
  analyze: PropTypes.func.isRequired,
  yearTable: PropTypes.arrayOf(PropTypes.number),
};

Graphs.defaultProps = {
  yearTable: [],
};

const mapStateToProps = state => ({
  accounts: state.results.accounts,
  expenses: state.results.expenses,
  incomeTotal: state.results.incomeTotal,
  incomeAfterTax: state.results.incomeAfterTax,
  net: state.results.net,
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
)(Graphs);
