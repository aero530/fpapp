import fs from 'fs';

export const OPEN_DATA_FILE = 'OPEN_DATA_FILE';
export const SAVE_DATA_FILE = 'SAVE_DATA_FILE';
export const EDIT_DATA_FILE_STATE_CONTENT = 'EDIT_DATA_FILE_STATE_CONTENT';
export const UPDATE_SETTING = 'UPDATE_SETTING';
export const UPDATE_ACCOUNT = 'UPDATE_ACCOUNT';
export const DELETE_ACCOUNT = 'DELETE_ACCOUNT';
export const ADD_ACCOUNT = 'ADD_ACCOUNT';

function openFileReducer(
  settingsInput,
  accountsInput,
  incomeAccountsInput,
  filenameInput
) {
  return {
    type: OPEN_DATA_FILE,
    settings: settingsInput,
    accounts: accountsInput,
    incomeAccounts: incomeAccountsInput,
    filename: filenameInput
  };
}

function saveFileReducer() {
  return {
    type: SAVE_DATA_FILE
  };
}

function getIncomeAccounts(accounts) {
  const incomeAccounts = [];
  incomeAccounts.push({ value: 'none', label: 'not linked' });

  Object.keys(accounts).forEach(name => {
    if (accounts[name].type === 'income') {
      const account = {
        value: name,
        label: name
      };

      incomeAccounts.push(account);
    }
  });
  return incomeAccounts;
}

export function openFile(filePath) {
  return dispatch => {
    fs.readFile(filePath, 'utf8', (err, data) => {
      if (err) throw err;
      const result = JSON.parse(data);

      const settingsFromFile = result.settings ? result.settings : {};
      const accountsFromFile = result.accounts ? result.accounts : {};
      const incomeAccounts = getIncomeAccounts(accountsFromFile);

      dispatch(
        openFileReducer(
          settingsFromFile,
          accountsFromFile,
          incomeAccounts,
          filePath
        )
      );
    });
  };
}

export function saveFile(filePathInput = null) {
  return (dispatch, getState) => {
    const { settings, accounts } = getState().data;
    let { filename } = getState().data;

    if (filePathInput) {
      filename = filePathInput;
    }
    const data = { accounts, settings };
    const result = JSON.stringify(data, null, '  ');

    fs.writeFile(filename, result, err => {
      if (err) throw err;
      dispatch(saveFileReducer());
    });
  };
}

export function editFileStateContent(contentInput) {
  return {
    type: EDIT_DATA_FILE_STATE_CONTENT,
    content: contentInput
  };
}
