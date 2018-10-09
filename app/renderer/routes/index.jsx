import React from 'react';
import { Route, Switch, Redirect } from 'react-router-dom';

import AppRoute from './app';

const Routes = () => (
  <Switch>
    <Route path="/app" component={AppRoute} />
    <Redirect to={{ pathname: '/app' }} />
  </Switch>
);

export default Routes;
