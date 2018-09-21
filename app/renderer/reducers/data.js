import {
  OPEN_DATA_FILE,
  SAVE_DATA_FILE,
  EDIT_DATA_FILE_STATE_CONTENT,
  UPDATE_SETTING,
  UPDATE_ACCOUNT,
  DELETE_ACCOUNT,
  ADD_ACCOUNT
} from '../actions/data';

import show from '../components/accountStructure';

import { v1 as uuidv1 } from 'uuid';

const initialState = {
  settings: {},
  accounts: {},
  incomeAccounts: [],
  filename: '',
  modified: false
};

export default function(state = initialState, action) {
  switch (action.type) {
    case OPEN_DATA_FILE:
      return {
        ...state,
        settings: action.settings,
        accounts: action.accounts,
        incomeAccounts: action.incomeAccounts,
        filename: action.filename,
        modified: false
      };

    case SAVE_DATA_FILE:
      return {
        ...state,
        modified: false
      };

    case EDIT_DATA_FILE_STATE_CONTENT:
      return {
        ...state,
        modified: true,
        settings: action.settings,
        accounts: action.accounts
      };

    case UPDATE_SETTING: {
      const prevSettings = state.settings;
      prevSettings[action.name] = action.value;
      return {
        ...state,
        settings: prevSettings
      };
    }

    case UPDATE_ACCOUNT: {
      return {
        ...state,
        accounts: { ...state.accounts, [action.name]: action.data }
      };
    }

    case DELETE_ACCOUNT: {
      const prevAccounts = state.accounts;
      delete prevAccounts[action.name];
      console.log(action.name);
      return {
        ...state,
        accounts: prevAccounts
      };
    }

    case ADD_ACCOUNT: {
      const prevAccounts = state.accounts;
      const id = uuidv1();

      // Create new account object and populate with keys from imported show object
      let newAccount = { ...show[action.accountType] };
      Object.keys(newAccount).forEach(key => {
        newAccount[key] = '';
      });

      newAccount.type = action.accountType; // set account type

      const newAccounts = { ...prevAccounts, [id]: newAccount };
      return {
        ...state,
        accounts: newAccounts
      };
    }

    default:
      return state;
  }
}
