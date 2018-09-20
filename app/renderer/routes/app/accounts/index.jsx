// cSpell:ignore Unmount

import React from 'react';
import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import { SET_APP_BAR_TITLE } from '../../../actions/app';
import { UPDATE_ACCOUNT } from '../../../actions/data';

import Account from '../../../components/account';

const styles = () => ({
  root: {
    width: '100%'
  }
});

class Accounts extends React.Component {
  constructor(props) {
    super(props);

    this.state = {};
  }

  render() {
    const {
      classes,
      match,
      accounts,
      incomeAccounts,
      onAccountChange,
      setAppBarTitle
    } = this.props;

    setAppBarTitle(match.params.type);

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
                onAccountChange={onAccountChange}
              />
            );
          }
          return null;
        })}
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
  onAccountChange: (accountNameInput, fieldNameInput, fieldValueInput) =>
    dispatch({
      type: UPDATE_ACCOUNT,
      accountName: accountNameInput,
      fieldName: fieldNameInput,
      fieldValue: fieldValueInput
    })
});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps
  )
)(Accounts);
