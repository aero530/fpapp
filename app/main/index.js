import path from 'path';
import { app, crashReporter, BrowserWindow, Menu, ipcMain } from 'electron';

import MenuBuilder from './menu';

const isDevelopment = process.env.NODE_ENV === 'development';

let mainWindow = null;
let backgroundWindow = null;
let forceQuit = false;
let showExitPrompt = true;

const installExtensions = async () => {
  const installer = require('electron-devtools-installer');
  const extensions = ['REACT_DEVELOPER_TOOLS', 'REDUX_DEVTOOLS'];
  const forceDownload = !!process.env.UPGRADE_EXTENSIONS;
  for (const name of extensions) {
    try {
      await installer.default(installer[name], forceDownload);
    } catch (e) {
      console.log(`Error installing ${name} extension: ${e.message}`);
    }
  }
};

crashReporter.start({
  productName: 'P. Spindler',
  companyName: 'n/a',
  submitURL: 'https://github.com/aero530/fpapp.git',
  uploadToServer: false,
});

app.on('window-all-closed', () => {
  // On OS X it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('ready', async () => {
  if (isDevelopment) {
    await installExtensions();
  }

  // ------------------------------------------------
  // Main Window
  // ------------------------------------------------
  mainWindow = new BrowserWindow({
    show: false,
    width: 1024,
    height: 900,
    webPreferences: {
      nodeIntegration: true,
    },
    icon: `${__dirname}/../../resources/icon.ico`,
  });

  mainWindow.loadFile(path.resolve(path.join(__dirname, '../renderer/index.html')));
  
  // show window once on first load
  mainWindow.webContents.once('did-finish-load', () => {
    mainWindow.show();
  });

  mainWindow.webContents.on('did-finish-load', () => {
    // Handle window logic properly on macOS:
    // 1. App should not terminate if window has been closed
    // 2. Click on icon in dock should re-open the window
    // 3. ⌘+Q should close the window and quit the app
    if (process.platform === 'darwin') {
      mainWindow.on('close', function(e) {
        if (!forceQuit) {
          e.preventDefault();
          mainWindow.hide();
        }
      });

      app.on('activate', () => {
        mainWindow.show();
      });

      app.on('before-quit', () => {
        forceQuit = true;
      });
    } else {
      mainWindow.on('close', e => {
        if (showExitPrompt) {
          e.preventDefault(); // Prevents the window from closing
          mainWindow.webContents.send('closeOnClick'); // Tell the rendered that a window close was attempted
        }
      });
      mainWindow.on('closed', () => {
        mainWindow = null;
      });
    }
  });
  

  if (isDevelopment) {
    // auto-open dev tools
    mainWindow.webContents.openDevTools();

    // add inspect element on right click menu
    mainWindow.webContents.on('context-menu', (e, props) => {
      Menu.buildFromTemplate([
        {
          label: 'Inspect element',
          click() {
            mainWindow.inspectElement(props.x, props.y);
          },
        },
      ]).popup(mainWindow);
    });
  }
  

  // ------------------------------------------------
  // Background Window
  // ------------------------------------------------
  if (isDevelopment) {
    backgroundWindow = new BrowserWindow({
      show: true,
      webPreferences: {
        nodeIntegration: true, // this must be included to allow the js code in this window to import node modules
      },
      parent: mainWindow,
    });
  } else {
    backgroundWindow = new BrowserWindow({
      show: false,
      webPreferences: {
        nodeIntegration: true, // this must be included to allow the js code in this window to import node modules
      },
      parent: mainWindow,
    });
  }
  

  backgroundWindow.loadFile(path.resolve(path.join(__dirname, '../background/index.html')));

  backgroundWindow.on('closed', () => {
    backgroundWindow = null;
  });

  backgroundWindow.webContents.on('did-finish-load', () => {
    // Handle window logic properly on macOS:
    // 1. App should not terminate if window has been closed
    // 2. Click on icon in dock should re-open the window
    // 3. ⌘+Q should close the window and quit the app
    if (process.platform === 'darwin') {
      backgroundWindow.on('close', function(e) {
        if (!forceQuit) {
          e.preventDefault();
          backgroundWindow.hide();
        }
      });

      app.on('activate', () => {
        backgroundWindow.show();
      });

      app.on('before-quit', () => {
        forceQuit = true;
      });
    } else {
      backgroundWindow.on('closed', () => {
        backgroundWindow = null;
      });
    }
  });

  
  if (isDevelopment) {
    // auto-open dev tools
    backgroundWindow.webContents.openDevTools();

    // add inspect element on right click menu
    backgroundWindow.webContents.on('context-menu', (e, props) => {
      Menu.buildFromTemplate([
        {
          label: 'Inspect element',
          click() {
            backgroundWindow.inspectElement(props.x, props.y);
          },
        },
      ]).popup(backgroundWindow);
    });
  }

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
    backgroundWindow.close();
    mainWindow.close();
  });

  // Windows can talk to each other via main
  ipcMain.on('analysisResults', (event, arg) => {
    mainWindow.webContents.send('analysisResults', arg);
  });

  ipcMain.on('analysisErrors', (event, arg) => {
    mainWindow.webContents.send('analysisErrors', arg);
  });

  ipcMain.on('backgroundCompute', (event, arg1, arg2) => {
    backgroundWindow.webContents.send('backgroundCompute', arg1, arg2);
  });


});
