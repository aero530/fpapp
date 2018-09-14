import fs from 'fs';

export const OPEN_FILE = 'OPEN_FILE';
export const SAVE_FILE = 'SAVE_FILE';
export const EDIT_FILE_STATE_CONTENT = 'EDIT_FILE_STATE_CONTENT';

function openFileReducer(contentInput, filenameInput) {
  return {
    type: OPEN_FILE,
    content: contentInput,
    filename: filenameInput
  };
}

function saveFileReducer() {
  return {
    type: SAVE_FILE
  };
}

export function openFile(filePaths) {
  return dispatch => {
    fs.readFile(filePaths[0], 'utf8', (err, data) => {
      if (err) throw err;
      dispatch(openFileReducer(data, filePaths[0]));
    });
  };
}

export function saveFile(filePathInput) {
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
    type: EDIT_FILE_STATE_CONTENT,
    content: contentInput
  };
}
