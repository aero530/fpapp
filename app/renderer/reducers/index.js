import { combineReducers } from 'redux';
// import { routerReducer as router } from 'react-router-redux';
import { connectRouter } from 'connected-react-router';
import data from './data';
import results from './results';
import app from './app';

// /**
//  * @description combine all redux reducers to a single root reducer for application to the redux store
//  */
// const rootReducer = combineReducers({
//   app,
//   data,
//   results,
//   router,
// });

// export default rootReducer;

export default function createRootReducer(history) {
  return combineReducers({
    router: connectRouter(history),
    app,
    data,
    results,
  });
}
