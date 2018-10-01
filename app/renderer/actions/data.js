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
  hsaAccountsInput,
  filenameInput,
) {
  return {
    type: OPEN_DATA_FILE,
    settings: settingsInput,
    accounts: accountsInput,
    incomeAccounts: incomeAccountsInput,
    hsaAccounts: hsaAccountsInput,
    filename: filenameInput,
  };
}

function saveFileReducer() {
  return {
    type: SAVE_DATA_FILE,
  };
}

function getAccountsTypeOf(accounts, type) {
  const typeAccounts = [];
  typeAccounts.push({ value: 'none', label: 'not linked' });

  Object.keys(accounts).forEach((name) => {
    if (accounts[name].type === type) {
      const account = {
        value: name,
        label: accounts[name].name,
      };
      typeAccounts.push(account);
    }
  });
  return typeAccounts;
}

export function openFile(filePath) {
  return (dispatch) => {
    fs.readFile(filePath, 'utf8', (err, data) => {
      if (err) throw err;
      const result = JSON.parse(data);

      const settingsFromFile = result.settings ? result.settings : {};
      const accountsFromFile = result.accounts ? result.accounts : {};
      const incomeAccounts = getAccountsTypeOf(accountsFromFile, 'income');
      const hsaAccounts = getAccountsTypeOf(accountsFromFile, 'hsa');

      dispatch(
        openFileReducer(
          settingsFromFile,
          accountsFromFile,
          incomeAccounts,
          hsaAccounts,
          filePath,
        ),
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

    fs.writeFile(filename, result, (err) => {
      if (err) throw err;
      dispatch(saveFileReducer());
    });
  };
}

export function editFileStateContent(contentInput) {
  return {
    type: EDIT_DATA_FILE_STATE_CONTENT,
    content: contentInput,
  };
}
