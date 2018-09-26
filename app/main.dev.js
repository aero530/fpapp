/* eslint global-require: 0 */

/**
 * This module executes inside of electron's main process. You can start
 * electron renderer process from here and communicate with the other processes
 * through IPC.
 *
 * When running `yarn build` or `yarn build-main`, this file is compiled to
 * `./app/main.prod.js` using webpack. This gives us some performance wins.
 *
 */
import { app, BrowserWindow, ipcMain } from 'electron';
import MenuBuilder from './menu';

let mainWindow = null;
let backgroundWindow = null;
let showExitPrompt = true;

if (process.env.NODE_ENV === 'production') {
  const sourceMapSupport = require('source-map-support');
  sourceMapSupport.install();
}

if (
  process.env.NODE_ENV === 'development' ||
  process.env.DEBUG_PROD === 'true'
) {
  require('electron-debug')();
  const path = require('path');
  const p = path.join(__dirname, '..', 'app', 'node_modules');
  require('module').globalPaths.push(p);
}

const installExtensions = async () => {
  const installer = require('electron-devtools-installer');
  const forceDownload = !!process.env.UPGRADE_EXTENSIONS;
  const extensions = ['REACT_DEVELOPER_TOOLS', 'REDUX_DEVTOOLS'];

  return Promise.all(
    extensions.map(name => installer.default(installer[name], forceDownload))
  ).catch(console.log);
};

/**
 * Add event listeners...
 */

app.on('window-all-closed', () => {
  // Respect the OSX convention of having the application in memory even
  // after all windows have been closed
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('ready', async () => {
  if (
    process.env.NODE_ENV === 'development' ||
    process.env.DEBUG_PROD === 'true'
  ) {
    await installExtensions();
  }

  // ------------------------------------------------
  // Main Window
  // ------------------------------------------------
  mainWindow = new BrowserWindow({
    show: false,
    width: 1024,
    height: 900
  });

  mainWindow.loadURL(`file://${__dirname}/app.html`);

  // @TODO: Use 'ready-to-show' event
  //        https://github.com/electron/electron/blob/master/docs/api/browser-window.md#using-ready-to-show-event
  mainWindow.webContents.on('did-finish-load', () => {
    if (!mainWindow) {
      throw new Error('"mainWindow" is not defined');
    }
    if (process.env.START_MINIMIZED) {
      mainWindow.minimize();
    } else {
      mainWindow.show();
      mainWindow.focus();
    }
  });

  mainWindow.on('close', e => {
    if (showExitPrompt) {
      e.preventDefault(); // Prevents the window from closing
      mainWindow.webContents.send('closeOnClick'); // Tell the rendered that a window close was attempted
    }
  });

  mainWindow.on('closed', () => {
    mainWindow = null;
  });

  // ------------------------------------------------
  // Background Window
  // ------------------------------------------------
  backgroundWindow = new BrowserWindow({
    show: false
  });

  // backgroundWindow.loadURL(`file://${__dirname}/background/index.html`);
  backgroundWindow.loadURL(`file://${__dirname}/background.html`);
  backgroundWindow.on('closed', () => {
    backgroundWindow = null;
  });

  // ------------------------------------------------
  // Make menu
  // ------------------------------------------------

  const menuBuilder = new MenuBuilder(mainWindow, backgroundWindow);
  menuBuilder.buildMenu();

  // ------------------------------------------------
  // Setup IPC Communication
  // ------------------------------------------------

  // renderer allows the app to close
  ipcMain.on('quitApp', () => {
    showExitPrompt = false;
    mainWindow.close();
    backgroundWindow.close();
  });

  // Windows can talk to each other via main
  ipcMain.on('for-renderer', (event, arg) => {
    mainWindow.webContents.send('to-renderer', arg);
  });

  ipcMain.on('for-background', (event, arg) => {
    backgroundWindow.webContents.send('to-background', arg);
  });

  ipcMain.on('backgroundCompute', (event, arg1, arg2) => {
    backgroundWindow.webContents.send('backgroundCompute', arg1, arg2);
  });
});
