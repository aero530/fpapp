// cSpell:ignore Unmount

/**
 * @class Accounts
 * @description Show accounts of the type specified by the url
 */

import React from 'react';
import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import Grid from '@material-ui/core/Grid';

import { SET_APP_BAR_TITLE } from '../../actions/app';
import {
  UPDATE_ACCOUNT,
  DELETE_ACCOUNT,
  ADD_ACCOUNT,
} from '../../actions/data';

import Account from '../../components/account';
import FloatingActionButton from '../../components/floatingActionButton';

const styles = theme => ({
  root: {
    width: '100%',
  },
  addFloatingActionButton: {
    position: 'absolute',
    bottom: theme.spacing(4),
    right: theme.spacing(4),
  },
});

class Accounts extends React.Component {
  componentDidMount() {
    const { match, setAppBarTitle } = this.props;
    setAppBarTitle(match.params.type);
  }

  componentDidUpdate(prevProps) {
    const { match, setAppBarTitle } = this.props;
    if (match.params.type !== prevProps.match.params.type) {
      // if location changed
      setAppBarTitle(match.params.type);
    }
  }

  render() {
    const {
      classes,
      match,
      accounts,
      incomeAccounts,
      hsaAccounts,
      onUpdate,
      onDelete,
      onAdd,
    } = this.props;

    return (
      <Grid container className={classes.root} spacing={10}>
        <Grid item xs={12} />
        <Grid container justify="center" spacing={10}>
          {Object.keys(accounts).map((name, index) => {
            if (accounts[name].type === match.params.type) {
              const key = `account-${name}-${index}`;
              return (
                <Grid key={`grid-${key}`} item xs={12} sm={12} md={12} lg={6}>
                  <Account
                    key={key}
                    account={accounts[name]}
                    incomeAccounts={incomeAccounts}
                    hsaAccounts={hsaAccounts}
                    onUpdate={(account) => {
                      onUpdate(name, account);
                    }}
                    onDelete={() => {
                      onDelete(name);
                      this.forceUpdate();
                    }}
                  />
                </Grid>
              );
            }
            return null;
          })}
        </Grid>
        <FloatingActionButton
          className={classes.addFloatingActionButton}
          onClick={() => {
            onAdd(match.params.type);
          }}
        />
      </Grid>
    );
  }
}


Accounts.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
  match: PropTypes.object.isRequired,
  accounts: PropTypes.objectOf(PropTypes.object).isRequired,
  incomeAccounts: PropTypes.arrayOf(PropTypes.object).isRequired,
  hsaAccounts: PropTypes.arrayOf(PropTypes.object).isRequired,
  onUpdate: PropTypes.func.isRequired,
  onDelete: PropTypes.func.isRequired,
  onAdd: PropTypes.func.isRequired,
};

Accounts.defaultProps = {
};

const mapStateToProps = state => ({
  accounts: state.data.accounts,
  incomeAccounts: state.data.incomeAccounts,
  hsaAccounts: state.data.hsaAccounts,
});

const mapDispatchToProps = dispatch => ({
  setAppBarTitle: titleInput => dispatch({ type: SET_APP_BAR_TITLE, title: titleInput }),
  onUpdate: (accountNameInput, accountInput) => dispatch({ type: UPDATE_ACCOUNT, name: accountNameInput, data: accountInput }),
  onDelete: nameInput => dispatch({ type: DELETE_ACCOUNT, name: nameInput }),
  onAdd: typeInput => dispatch({ type: ADD_ACCOUNT, accountType: typeInput }),
});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps,
  ),
)(Accounts);
