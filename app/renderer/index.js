import React from 'react';
import { render } from 'react-dom';
import { AppContainer } from 'react-hot-loader';
import { ipcRenderer } from 'electron';
import Root from './root';
import { configureStore, history } from './store/configureStore';

import { openFile, saveFile } from './actions/data';
import { loadResults, loadError, loadErrors } from './actions/results';

import '../app.global.css';

const { dialog } = require('electron').remote;

const store = configureStore();

ipcRenderer.on('analysisError', (event, arg) => {
  store.dispatch(loadError(arg));
});

ipcRenderer.on('analysisErrors', (event, arg) => {
  store.dispatch(loadErrors(arg));
});

ipcRenderer.on('analysisResults', (event, arg) => {
  store.dispatch(loadResults(arg));
});

ipcRenderer.on('fileOpen', (event, arg) => {
  store.dispatch(openFile(arg.filename[0]));
});

ipcRenderer.on('fileSave', () => {
  store.dispatch(saveFile());
});

ipcRenderer.on('fileSaveAs', (event, arg) => {
  store.dispatch(saveFile(arg.filename));
});

function showOpen() {
  dialog.showOpenDialog(
    {
      title: 'Open file',
      buttonLabel: 'open the file',
      filters: [
        { name: 'Text', extensions: ['json'] },
        { name: 'All Files', extensions: [] },
      ],
    },
    (filePaths) => {
      if (filePaths) {
        store.dispatch(openFile(filePaths[0]));
      }
    },
  );
}

ipcRenderer.on('fileOpenOnClick', () => {
  const { modified } = store.getState().data;

  if (modified) {
    dialog.showMessageBox(
      {
        type: 'warning', // Can be "none", "info", "error", "question" or "warning".
        buttons: ['Save', "Don't Save", 'Cancel'], // Array of texts for buttons. On Windows, an empty array will result in one button labeled "OK".
        defaultId: 0, // Index of the button in the buttons array which will be selected by default when the message box opens.
        title: 'File has not been saved', // Title of the message box, some platforms will not show it.
        message: 'Unsaved file', // Content of the message box.
        detail: 'Do you want to save or discard your current edits?', // extra information of the message.
      },
      (selection) => {
        if (selection === 0) {
          store.dispatch(saveFile());
          showOpen();
        } else if (selection === 1) {
          showOpen();
        } else if (selection === 2) {
          return null;
        }
      },
    );
  } else {
    showOpen();
  }
});

ipcRenderer.on('closeOnClick', () => {
  const { modified } = store.getState().data;
  if (modified) {
    // if file has been modified
    dialog.showMessageBox(
      // check to see if user wants to save
      {
        type: 'warning', // Can be "none", "info", "error", "question" or "warning".
        buttons: ['Save', "Don't Save"], // Array of texts for buttons. On Windows, an empty array will result in one button labeled "OK".
        defaultId: 0, // Index of the button in the buttons array which will be selected by default when the message box opens.
        title: 'File has not been saved', // Title of the message box, some platforms will not show it.
        message: 'Unsaved file', // Content of the message box.
        detail: 'Do you want to save or discard your current edits?', // extra information of the message.
      },
      (selection) => {
        if (selection === 0) {
          // user chose to save
          store.dispatch(saveFile()); // save file
          ipcRenderer.send('quitApp'); // tell main process to quit
        } else {
          // user chose not to save
          ipcRenderer.send('quitApp'); // tell main process to quit
        }
      },
    );
  } else {
    // file was not modified
    ipcRenderer.send('quitApp'); // tell main process to quit
  }
});

render(
  <AppContainer>
    <Root store={store} history={history} />
  </AppContainer>,
  document.getElementById('root'),
);

if (module.hot) {
  module.hot.accept('./root', () => {
    const NextRoot = require('./root'); // eslint-disable-line global-require
    render(
      <AppContainer>
        <NextRoot store={store} history={history} />
      </AppContainer>,
      document.getElementById('root'),
    );
  });
}
