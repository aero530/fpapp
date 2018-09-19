import fs from 'fs';

export const OPEN_DATA_FILE = 'OPEN_DATA_FILE';
export const SAVE_DATA_FILE = 'SAVE_DATA_FILE';
export const EDIT_DATA_FILE_STATE_CONTENT = 'EDIT_DATA_FILE_STATE_CONTENT';
export const UPDATE_SETTING = 'UPDATE_SETTING';
export const UPDATE_ACCOUNT = 'UPDATE_ACCOUNT';

function openFileReducer(settingsInput, accountsInput, filenameInput) {
  return {
    type: OPEN_DATA_FILE,
    settings: settingsInput,
    accounts: accountsInput,
    filename: filenameInput
  };
}

function saveFileReducer() {
  return {
    type: SAVE_DATA_FILE
  };
}

export function openFile(filePath) {
  return dispatch => {
    fs.readFile(filePath, 'utf8', (err, data) => {
      if (err) throw err;
      const result = JSON.parse(data);

      const settingsFromFile = result.settings ? result.settings : {};
      const accountsFromFile = result.accounts ? result.accounts : {};

      dispatch(openFileReducer(settingsFromFile, accountsFromFile, filePath));
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
