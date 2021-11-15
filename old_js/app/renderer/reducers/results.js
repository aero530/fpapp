import { LOAD_RESULTS, LOAD_ERRORS, CLEAR_ERRORS, INITIALIZE_RESULTS_STATE } from '../actions/results';

const initialState = {
  accounts: {},
  savings: {},
  expenses: {},
  incomeTaxable: {},
  incomeTotal: {},
  incomeAfterTax: {},
  net: {},
  year: [],
  errors: [],
};

export default function (state = initialState, action) {
  switch (action.type) {
    /**
     * @function LOAD_RESULTS
     * @description initialize results data in store
     * @listens: reducer:LOAD_RESULTS
     */
    case INITIALIZE_RESULTS_STATE: {
      return {
        ...state,
        ...initialState,
      };
    }

    /**
     * @function LOAD_RESULTS
     * @description update redux store results object with output from background analysis
     * @listens: reducer:LOAD_RESULTS
     */
    case LOAD_RESULTS: {
      return {
        ...state,
        accounts: { ...action.accounts },
        savings: action.savings,
        expenses: action.expenses,
        incomeTaxable: action.incomeTaxable,
        incomeTotal: action.incomeTotal,
        incomeAfterTax: action.incomeAfterTax,
        net: action.net,
        year: action.year,
      };
    }

    /**
     * @function LOAD_ERRORS
     * @description update redux store results object by adding new errors from background analysis to existing array
     * @listens: reducer:LOAD_ERRORS
     */
    case LOAD_ERRORS: {
      let prevErrors = state.errors;
      prevErrors = prevErrors.concat(action.errors);
      const count = prevErrors.length;
      return {
        ...state,
        errors: prevErrors,
        errorCount: count,
      };
    }

    /**
     * @function CLEAR_ERRORS
     * @description update redux store results object by clearing all errors
     * @listens: reducer:CLEAR_ERRORS
     */
    case CLEAR_ERRORS: {
      return {
        ...state,
        errors: [],
        errorCount: 0,
      };
    }

    default:
      return state;
  }
}
