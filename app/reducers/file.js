import { OPEN_FILE, SAVE_FILE, EDIT_FILE_STATE_CONTENT } from '../actions/file';

const initialState = {
  content: '',
  filename: '',
  modified: false
};

export default function(state = initialState, action) {
  switch (action.type) {
    case OPEN_FILE:
      return {
        ...state,
        content: action.content,
        filename: action.filename,
        modified: false
      };
    case SAVE_FILE:
      return {
        ...state,
        modified: false
      };
    case EDIT_FILE_STATE_CONTENT:
      return {
        ...state,
        modified: true,
        content: action.content
      };
    default:
      return state;
  }
}
