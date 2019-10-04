// cSpell: ignore uuidv1

import { v1 as uuidv1 } from 'uuid';

import {
  INITIALIZE_DATA_STATE,
  OPEN_DATA_FILE,
  SAVE_DATA_FILE,
  UPDATE_SETTING,
  UPDATE_ACCOUNT,
  DELETE_ACCOUNT,
  ADD_ACCOUNT,
} from '../actions/data';

import { show } from '../constants/accountStructure';

const initialState = {
  settings: {
    ageRetire: null,
    ageDie: null,
    yearBorn: null,
    yearStart: null,
    inflationBase: null,
    taxIncome: null,
    taxCapitalGains: null,
    retirementCostOfLiving: null,
    ssaBreakpointsLow: null,
    ssaBreakpointsHigh: null,
    ssaTaxableIncomePercentageLow: null,
    ssaTaxableIncomePercentageHigh: null,
  },
  accounts: {},
  incomeAccounts: [],
  hsaAccounts: [],
  filename: '',
  modified: false,
};

export default function (state = initialState, action) {
  switch (action.type) {
    /**
     * @function INITIALIZE_DATA_STATE
     * @description update redux store data object to default (initialized) values
     * @listens: reducer:INITIALIZE_DATA_STATE
     */
    case INITIALIZE_DATA_STATE:
      return {
        ...store,
        ...initialState,
      };

    /**
     * @function OPEN_DATA_FILE
     * @description update redux store data object with data from opened file
     * @listens: reducer:OPEN_DATA_FILE
     */
    case OPEN_DATA_FILE:
      return {
        ...state,
        settings: action.settings,
        accounts: action.accounts,
        incomeAccounts: action.incomeAccounts,
        hsaAccounts: action.hsaAccounts,
        filename: action.filename,
        modified: false,
      };

    /**
     * @function SAVE_DATA_FILE
     * @description update redux store data object to note that the file has been saved (ie modified = false)
     * @listens: reducer:SAVE_DATA_FILE
     */
    case SAVE_DATA_FILE:
      return {
        ...state,
        modified: false,
      };

    /**
     * @function UPDATE_SETTING
     * @description update redux store data object with new setting value
     * @listens: reducer:UPDATE_SETTING
     */
    case UPDATE_SETTING: {
      const prevSettings = state.settings;
      prevSettings[action.name] = action.value;
      return {
        ...state,
        settings: prevSettings,
        modified: true,
      };
    }

    /**
     * @function UPDATE_ACCOUNT
     * @description update redux store data object by replacing existing account entry with new account entry
     * @listens: reducer:UPDATE_ACCOUNT
     */
    case UPDATE_ACCOUNT: {
      return {
        ...state,
        accounts: { ...state.accounts, [action.name]: action.data },
        modified: true,
      };
    }

    /**
     * @function DELETE_ACCOUNT
     * @description update redux store data object by removing an account
     * @listens: reducer:DELETE_ACCOUNT
     */
    case DELETE_ACCOUNT: {
      const prevAccounts = state.accounts;
      delete prevAccounts[action.name];
      console.log(action.name);
      return {
        ...state,
        accounts: prevAccounts,
        modified: true,
      };
    }

    /**
     * @function ADD_ACCOUNT
     * @description update redux store data object by adding a new empty account populated only with the account type
     * @listens: reducer:ADD_ACCOUNT
     */
    case ADD_ACCOUNT: {
      const prevAccounts = state.accounts;
      const id = uuidv1();

      // Create new account object and populate with keys from imported show object
      const newAccount = { ...show[action.accountType] };
      Object.keys(newAccount).forEach((key) => {
        newAccount[key] = '';
      });

      newAccount.type = action.accountType; // set account type

      const newAccounts = { ...prevAccounts, [id]: newAccount };
      return {
        ...state,
        accounts: newAccounts,
        modified: true,
      };
    }

    default:
      return state;
  }
}
