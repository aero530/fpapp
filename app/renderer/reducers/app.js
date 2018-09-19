import { SET_APP_BAR_TITLE } from '../actions/app';

const initialState = {
  appBarTitle: ''
};

export default function(state = initialState, action) {
  switch (action.type) {
    case SET_APP_BAR_TITLE:
      return {
        ...state,
        appBarTitle: action.title
      };

    default:
      return state;
  }
}
