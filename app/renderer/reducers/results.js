import { LOAD_RESULTS, LOAD_ERROR, LOAD_ERRORS, CLEAR_ERRORS } from '../actions/results';

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

    case LOAD_ERROR: {
      const prevErrors = state.errors;
      prevErrors.push(action.error);
      const count = prevErrors.length;
      return {
        ...state,
        errors: prevErrors,
        errorCount: count,
      };
    }

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
