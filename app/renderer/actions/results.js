import { ipcRenderer } from 'electron';

export const LOAD_RESULTS = 'LOAD_RESULTS';
export const RUN_ANALYSIS = 'RUN_ANALYSIS';

export function loadResults(arg) {
  return {
    type: LOAD_RESULTS,
    ...arg
  };
}

export function analyze() {
  return (dispatch, getState) => {
    const { settings, accounts } = getState().data;
    dispatch({ type: RUN_ANALYSIS });
    ipcRenderer.send('backgroundCompute', accounts, settings);
  };
}
