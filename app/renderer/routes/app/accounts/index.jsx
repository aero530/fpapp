// cSpell:ignore Unmount

import React from 'react';
import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import { SET_APP_BAR_TITLE } from '../../../actions/app';
import {
  UPDATE_ACCOUNT,
  DELETE_ACCOUNT,
  ADD_ACCOUNT
} from '../../../actions/data';

import Account from '../../../components/account';
import FloatingActionButton from '../../../components/floatingActionButton';

const styles = theme => ({
  root: {
    width: '100%'
  },
  addFloatingActionButton: {
    position: 'absolute',
    bottom: theme.spacing.unit * 4,
    right: theme.spacing.unit * 4
  }
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
      onUpdate,
      onDelete,
      onAdd
    } = this.props;

    return (
      <div>
        {Object.keys(accounts).map((name, index) => {
          if (accounts[name].type === match.params.type) {
            const key = `account-${name}-${index}`;
            return (
              <Account
                key={key}
                account={accounts[name]}
                incomeAccounts={incomeAccounts}
                onUpdate={account => {
                  onUpdate(name, account);
                }}
                onDelete={() => {
                  onDelete(name);
                  this.forceUpdate();
                }}
              />
            );
          }
          return null;
        })}
        <FloatingActionButton
          className={classes.addFloatingActionButton}
          onClick={() => {
            onAdd(match.params.type);
          }}
        />
      </div>
    );
  }
}

Accounts.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
  onAccountChange: PropTypes.func.isRequired
};

Accounts.defaultProps = {};

const mapStateToProps = state => ({
  accounts: state.data.accounts,
  incomeAccounts: state.data.incomeAccounts
});

const mapDispatchToProps = dispatch => ({
  setAppBarTitle: titleInput =>
    dispatch({ type: SET_APP_BAR_TITLE, title: titleInput }),
  onUpdate: (accountNameInput, accountInput) =>
    dispatch({
      type: UPDATE_ACCOUNT,
      name: accountNameInput,
      data: accountInput
    }),
  onDelete: nameInput => dispatch({ type: DELETE_ACCOUNT, name: nameInput }),
  onAdd: typeInput => dispatch({ type: ADD_ACCOUNT, accountType: typeInput })
});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps
  )
)(Accounts);
