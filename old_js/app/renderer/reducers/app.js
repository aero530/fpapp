
import { SET_APP_BAR_TITLE } from '../actions/app';

const initialState = {
  appBarTitle: '',
};

export default function (state = initialState, action) {
  switch (action.type) {
    /**
     * @function SET_APP_BAR_TITLE
     * @description update redux store app object with new app bar title
     * @listens: reducer:SET_APP_BAR_TITLE
     */
    case SET_APP_BAR_TITLE:
      return {
        ...state,
        appBarTitle: action.title,
      };

    default:
      return state;
  }
}
