import { ipcRenderer } from 'electron';
import accountComputation from './accountComputation';

/**
 * @function ready
 * @description let the main thread know this thread is ready to process something
 * @returns {Object} object version of the array data
 */
function ready() {
  ipcRenderer.send('ready');
}

/**
 * @description run computation and send results back
 * @listens ipcRenderer:backgroundCompute
 * @param event
 * @param accounts accounts object
 * @param settings settings object
 * @fires ipcRenderer:analysisResults
 */
ipcRenderer.on('backgroundCompute', (event, accounts, settings) => {
  console.log('background computation');
  const result = accountComputation(accounts, settings);
  ipcRenderer.send('analysisResults', result);
});

ready();
