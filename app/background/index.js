import { ipcRenderer } from 'electron';
import accountComputation from './accountComputation';

// let the main thread know this thread is ready to process something
function ready() {
  ipcRenderer.send('ready');
}

ipcRenderer.on('backgroundCompute', (event, accounts, settings) => {
  console.log('background computation');
  // const result = work();
  const result = accountComputation(accounts, settings);
  ipcRenderer.send('analysisResults', result);
});

ready();
