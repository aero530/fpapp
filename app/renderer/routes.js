import React from 'react';
import { Switch, Route, Redirect } from 'react-router-dom';

import AppRoute from './containers/AppRoute'

export default (
  <Switch>
    <Route path="/app" component={AppRoute} />
    <Redirect to={{ pathname: '/app' }} />
  </Switch>
);