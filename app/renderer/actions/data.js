import fs from 'fs';

export const OPEN_DATA_FILE = 'OPEN_DATA_FILE';
export const SAVE_DATA_FILE = 'SAVE_DATA_FILE';
export const EDIT_DATA_FILE_STATE_CONTENT = 'EDIT_DATA_FILE_STATE_CONTENT';

function openFileReducer(contentInput, accountsInput, filenameInput) {
  return {
    type: OPEN_DATA_FILE,
    settings: contentInput,
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

      dispatch(openFileReducer(data, data, filePath));
    });
  };
}

export function saveFile() {
  return (dispatch, getState) => {
    const { content, filename } = getState().file;
    fs.writeFile(filename, content, err => {
      if (err) throw err;
      dispatch(saveFileReducer());
    });
  };
}

export function saveAsFile(filePathInput) {
  return (dispatch, getState) => {
    const { content } = getState().file;
    fs.writeFile(filePathInput, content, err => {
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
