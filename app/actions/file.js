import fs from 'fs';

export const OPEN_FILE = 'OPEN_FILE';

export function readFile(contentInput) {
  return {
    type: OPEN_FILE,
    content: contentInput
  };
}

export function openFile(filePaths) {
  return dispatch => {
    fs.readFile(filePaths[0], 'utf8', (err, data) => {
      if (err) throw err;
      console.log(data);
      dispatch(readFile(data));
    });
  };
}
