export const LOAD_RESULTS = 'LOAD_RESULTS';

export function loadResults(arg) {
  return {
    type: LOAD_RESULTS,
    ...arg
  };
}
