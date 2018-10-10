import fs from 'fs';

export const OPEN_DATA_FILE = 'OPEN_DATA_FILE';
export const SAVE_DATA_FILE = 'SAVE_DATA_FILE';
export const UPDATE_SETTING = 'UPDATE_SETTING';
export const UPDATE_ACCOUNT = 'UPDATE_ACCOUNT';
export const DELETE_ACCOUNT = 'DELETE_ACCOUNT';
export const ADD_ACCOUNT = 'ADD_ACCOUNT';

/**
 * @function openFileReducer
 * @description action to apply opened file to the redux store
 * @fires reducer:OPEN_DATA_FILE
 */
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

/**
 * @function saveFileReducer
 * @description action to notify redux store that the data was stored
 * @fires reducer:SAVE_DATA_FILE
 */
function saveFileReducer() {
  return {
    type: SAVE_DATA_FILE,
  };
}

/**
 * @function getAccountsTypeOf
 * @description helper function to extract accounts of a specified type from the accounts object
 */
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

/**
 * @function openFile
 * @description action to open a data file.  opens file, reads contents, then dispatches action to put data in redux store
 * @fires: action:openFileReducer
 */
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

/**
 * @function saveFile
 * @description action to save the file then dispatch action to notify redux store that the file was saved
 * @fires: action:saveFileReducer
 */
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
