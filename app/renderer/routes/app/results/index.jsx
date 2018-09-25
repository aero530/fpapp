import React from 'react';
import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import classNames from 'classnames';

import Paper from '@material-ui/core/Paper';

import { SET_APP_BAR_TITLE } from '../../../actions/app';

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

class Results extends React.Component {
  constructor(props) {
    super(props);

    this.state = {};

    props.setAppBarTitle('Results');
  }

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
          <span>{JSON.stringify(accounts, undefined, 2)}</span>
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
