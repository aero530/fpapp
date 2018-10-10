import { ipcRenderer } from 'electron';

export const LOAD_RESULTS = 'LOAD_RESULTS';
export const RUN_ANALYSIS = 'RUN_ANALYSIS';
export const LOAD_ERRORS = 'LOAD_ERRORS';
export const CLEAR_ERRORS = 'CLEAR_ERRORS';

/**
 * @function loadResults
 * @description action to load results of background computation
 * @fires: reducer:LOAD_RESULTS
 */
export function loadResults(arg) {
  return {
    type: LOAD_RESULTS,
    ...arg,
  };
}

/**
 * @function loadErrors
 * @description action to load errors of background computation
 * @fires: reducer:LOAD_ERRORS
 */
export function loadErrors(arg) {
  return {
    type: LOAD_ERRORS,
    errors: arg,
  };
}

/**
 * @function analyze
 * @description action to kick off background computation
 * @fires: reducer:RUN_ANALYSIS
 * @fires: reducer:CLEAR_ERRORS
 * @fires: ipcRenderer:backgroundCompute
 */
export function analyze() {
  return (dispatch, getState) => {
    const { settings, accounts } = getState().data;
    dispatch({ type: RUN_ANALYSIS }); // notify redux store that analysis will start
    dispatch({ type: CLEAR_ERRORS }); // clear existing errors from the redux store
    ipcRenderer.send('backgroundCompute', accounts, settings); // send ipc message to start computation / simulation
  };
}
