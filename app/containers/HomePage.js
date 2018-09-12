import React, { Component } from 'react';
import compose from 'recompose/compose';
import { withStyles } from '@material-ui/core/styles';
import Home from '../components/Home';

const muiStyles = () => ({
  root: {}
});

class HomePage extends Component {
  render() {
    return <Home />;
  }
}

HomePage.propTypes = {};

export default compose(withStyles(muiStyles))(HomePage);
