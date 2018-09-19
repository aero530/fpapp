import {
  OPEN_DATA_FILE,
  SAVE_DATA_FILE,
  EDIT_DATA_FILE_STATE_CONTENT,
  UPDATE_SETTING,
  UPDATE_ACCOUNT
} from '../actions/data';

const initialState = {
  settings: {},
  accounts: {},
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
      const prevAccount = { ...state.accounts[action.accountName] };
      prevAccount[action.fieldName] = action.fieldValue;
      return {
        ...state,
        accounts: { ...state.accounts, [action.accountName]: prevAccount }
      };
    }

    default:
      return state;
  }
}
