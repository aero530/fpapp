import { createStore, applyMiddleware, combineReducers, compose } from 'redux';
import { connectRouter, routerMiddleware, push, routerActions } from 'connected-react-router';
import persistState from 'redux-localstorage';
import thunk from 'redux-thunk';


import user from './reducers/user';
import app from './reducers/app';
import data from './reducers/data';
import results from './reducers/results';


import userActions from './actions/user';
//import appActions from './actions/app';
//import dataActions from './actions/data';
//import resultsActions from './actions/results';

export default function configureStore(initialState, routerHistory) {
  const router = routerMiddleware(routerHistory);

  const actionCreators = {
    ...routerActions,
    //...appActions,
    //...dataActions,
    //...resultsActions,
    ...userActions,
    push,
  };

  
  
  const reducers = {
    router: connectRouter(routerHistory),
    app,
    data,
    results,
    user,
  };
  
  /*
  const createRootReducer = (history) => combineReducers({
    router: connectRouter(history),
    app,
    data,
    results,
  })
  */

  const middlewares = [thunk, router];

  const composeEnhancers = (() => {
    const compose_ = window && window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__;
    if (process.env.NODE_ENV === 'development' && compose_) {
      return compose_({ actionCreators });
    }
    return compose;
  })();

  const enhancer = composeEnhancers(applyMiddleware(...middlewares), persistState());
  const rootReducer = combineReducers(reducers);

  return createStore(rootReducer, initialState, enhancer);
}
