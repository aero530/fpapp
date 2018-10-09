import { ipcRenderer } from 'electron';

export const LOAD_RESULTS = 'LOAD_RESULTS';
export const RUN_ANALYSIS = 'RUN_ANALYSIS';
export const LOAD_ERROR = 'LOAD_ERROR';
export const LOAD_ERRORS = 'LOAD_ERRORS';
export const CLEAR_ERRORS = 'CLEAR_ERRORS';

export function loadResults(arg) {
  return {
    type: LOAD_RESULTS,
    ...arg,
  };
}

export function loadError(arg) {
  return {
    type: LOAD_ERROR,
    error: arg,
  };
}

export function loadErrors(arg) {
  return {
    type: LOAD_ERRORS,
    errors: arg,
  };
}

export function analyze() {
  return (dispatch, getState) => {
    const { settings, accounts } = getState().data;
    dispatch({ type: RUN_ANALYSIS }); // notify redux store that analysis will start
    dispatch({ type: CLEAR_ERRORS }); // clear existing errors from the redux store
    ipcRenderer.send('backgroundCompute', accounts, settings); // send ipc message to start computation / simulation
  };
}
