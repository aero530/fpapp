/**
 * @class Root
 * @description Root react component with material ui applied
 * @property {Object} store redux store
 * @property {string} history react router history
 */

import React, { PureComponent } from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import CssBaseline from '@material-ui/core/CssBaseline';
import {
  createMuiTheme,
  withStyles,
  MuiThemeProvider,
} from '@material-ui/core/styles';

import { Provider } from 'react-redux';
// import { ConnectedRouter } from 'react-router-redux';
import { ConnectedRouter } from 'connected-react-router';
import Routes from './routes';


/** @constant
 * Default theme settings
 * https://material-ui.com/customization/default-theme/
*/
const theme = createMuiTheme({
  palette: {
    contrastThreshold: 3,
    tonalOffset: 0.2,
    background: {
      paper: '#fafafa',
      default: '#eee',
    },
    primary: {
      light: '#ea96b9',
      main: '#9f1d54',
      dark: '#72173d',
      contrastText: '#ffffff',
    },
  },
  overrides: {
  },
});

const muiStyles = () => ({
  root: {},
});

class Root extends PureComponent {
  render() {
    const { store, history } = this.props;
    return (
      <MuiThemeProvider theme={theme}>
        <CssBaseline />
        <Provider store={store}>
          <ConnectedRouter history={history}>
            <Routes />
          </ConnectedRouter>
        </Provider>
      </MuiThemeProvider>
    );
  }
}

Root.propTypes = {
  store: PropTypes.object.isRequired, // eslint-disable-line react/forbid-prop-types
  history: PropTypes.object.isRequired, // eslint-disable-line react/forbid-prop-types
};

export default compose(withStyles(muiStyles))(Root);
