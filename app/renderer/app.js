import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import { ConnectedRouter } from 'connected-react-router';
import { createBrowserHistory } from 'history';
import { BrowserRouter as Router } from "react-router-dom"
import { ipcRenderer } from 'electron';
import CssBaseline from '@material-ui/core/CssBaseline';
import {
  createMuiTheme,
  MuiThemeProvider,
} from '@material-ui/core/styles';

import routes from './routes';
import configureStore from './store';

import { openFile, saveFile } from './actions/data';
import { loadResults, loadErrors } from './actions/results';

const { dialog } = require('electron').remote;

/**
 * @description listen for analysisErrors from main and dispatch loadErrors action
 * @listens ipcRenderer:analysisErrors
 * @param event
 * @param arg
 * @fires store:loadErrors
 */
ipcRenderer.on('analysisErrors', (event, arg) => {
  store.dispatch(loadErrors(arg));
});

/**
 * @description listen for analysisResults from main and dispatch loadResults action
 * @listens ipcRenderer:analysisResults
 * @param event
 * @param arg
 * @fires store:loadResults
 */
ipcRenderer.on('analysisResults', (event, arg) => {
  store.dispatch(loadResults(arg));
});

/**
 * @description listen for fileOpen from main and dispatch openFile action
 * @listens ipcRenderer:fileOpen
 * @param event
 * @param arg
 * @fires store:openFile
 */
ipcRenderer.on('fileOpen', (event, arg) => {
  store.dispatch(openFile(arg.filename[0]));
});

/**
 * @description listen for fileSave from main and dispatch saveFile action
 * @listens ipcRenderer:fileSave
 * @param event
 * @param arg
 * @fires store:saveFile
 */
ipcRenderer.on('fileSave', () => {
  store.dispatch(saveFile());
});

/**
 * @description listen for fileSaveAs from main and dispatch saveFile action
 * @listens ipcRenderer:fileSaveAs
 * @param event
 * @param arg
 * @fires store:saveFile
 */
ipcRenderer.on('fileSaveAs', (event, arg) => {
  store.dispatch(saveFile(arg.filename));
});

/**
 * @function showOpen
 * @description show OS native dialog to open a file
 * @fires store:openFile
 */
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

/**
 * @description listen for fileOpenOnClick from main. check if current file has been modified
 * (in which case show dialog to save current or cancel).  then show open file dialog.
 * @listens ipcRenderer:fileOpenOnClick
 * @fires store:showOpen
 */
ipcRenderer.on('fileOpenOnClick', () => {
  const { modified } = store.getState().data;

  if (modified) {
    dialog.showMessageBox(
      {
        type: 'warning', // Can be "none", "info", "error", "question" or "warning".
        buttons: ['Save', "Don't Save", 'Cancel'], // Array of texts for buttons. On Windows, an empty array will result in one button labeled "OK".
        defaultId: 0, // Index of the button in the buttons array which will be selected by default when the message box opens.
        title: 'File has not been saved',
        message: 'Unsaved file',
        detail: 'Do you want to save or discard your current edits?',
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

/**
 * @description listen for closeOnClick from main. check if current file has been modified
 * (in which case show dialog to save or not).  then close the application.
 * @listens ipcRenderer:closeOnClick
 * @fires ipcRenderer:quitApp
 */
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
        title: 'File has not been saved',
        message: 'Unsaved file',
        detail: 'Do you want to save or discard your current edits?',
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



/** @constant
 * Default theme settings
 * https://material-ui.com/customization/default-theme/
*/
const theme = createMuiTheme({
  palette: {
    contrastThreshold: 3,
    tonalOffset: 0.2,
    background: {
      paper: '#fafafa',
      default: '#eee',
    },
    primary: {
      light: '#ea96b9',
      main: '#9f1d54',
      dark: '#72173d',
      contrastText: '#ffffff',
    },
  },
  overrides: {
  },
});

const syncHistoryWithStore = (store, history) => {
  const { router } = store.getState();
  if (router && router.location) {
    history.replace(router.location);
  }
};

const initialState = {};
const routerHistory = createBrowserHistory();
const store = configureStore(initialState, routerHistory);
syncHistoryWithStore(store, routerHistory);

const rootElement = document.querySelector(document.currentScript.getAttribute('data-container'));

ReactDOM.render(
  <MuiThemeProvider theme={theme}>
    <CssBaseline />
    <Provider store={store}>
      <Router>
        <ConnectedRouter history={routerHistory}>{routes}</ConnectedRouter>
      </Router>  
    </Provider>
  </MuiThemeProvider>,
  rootElement,
);