import { combineReducers } from 'redux';
import { routerReducer as router } from 'react-router-redux';
import counter from './counter';
import file from './file';
import data from './data';
import results from './results';
import app from './app';

const rootReducer = combineReducers({
  app,
  data,
  results,
  file,
  counter,
  router
});

export default rootReducer;
