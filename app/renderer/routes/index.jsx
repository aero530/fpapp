/*
This work contains valuable confidential and proprietary information.
Disclosure, use or reproduction without the written authorization of
Dedicated Computing, LLC is prohibited. This unpublished work by the company
is protected by the laws of the United States and other countries.
If publication of the work should occur the following notice shall apply:

"c" Copyright Dedicated Computing LLC 2018 All Rights Reserved
*/

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
