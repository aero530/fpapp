import { LOAD_RESULTS } from '../actions/results';

const initialState = {
  accounts: {},
  savings: [],
  expenses: [],
  incomeTaxable: [],
  incomeTotal: [],
  incomeAfterTax: [],
  net: [],
  year: []
};

export default function(state = initialState, action) {
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
        year: action.year
      };
    }

    default:
      return state;
  }
}
