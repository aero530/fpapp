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
import { ConnectedRouter } from 'react-router-redux';
import Routes from './routes';

// Default theme settings
// https://material-ui.com/customization/default-theme/

const theme = createMuiTheme({
  palette: {
    // Used by `getContrastText()` to maximize the contrast between the background and
    // the text.
    contrastThreshold: 3,
    // Used to shift a color's luminance by approximately
    // two indexes within its tonal palette.
    // E.g., shift from Red 500 to Red 300 or Red 700.
    tonalOffset: 0.2,
    background: {
      paper: '#fafafa',
      default: '#eee',
    },
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
