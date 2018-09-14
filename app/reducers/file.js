import { OPEN_FILE } from '../actions/file';

const initialState = {
  content: ''
};

export default function(state = initialState, action) {
  switch (action.type) {
    case OPEN_FILE:
      return {
        ...state,
        content: action.content
      };
    default:
      return state;
  }
}
