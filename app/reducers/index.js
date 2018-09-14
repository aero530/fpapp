import { combineReducers } from 'redux';
import { routerReducer as router } from 'react-router-redux';
import counter from './counter';
import file from './file';

const rootReducer = combineReducers({
  file,
  counter,
  router
});

export default rootReducer;
