import React from 'react';
import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import Button from '@material-ui/core/Button';

import Grid from '@material-ui/core/Grid';

import { ipcRenderer } from 'electron';

import { SET_APP_BAR_TITLE } from '../../actions/app';

const styles = () => ({
  root: {},
});

class Dashboard extends React.Component {
  componentDidMount() {
    const { setAppBarTitle } = this.props;
    setAppBarTitle('');
  }

  render() {
    const { classes, accounts, settings } = this.props;

    return (
      <Grid container className={classes.root} spacing={16}>
        <Grid item xs={12} />
        <Grid container justify="center" spacing={16}>
          <Grid item xs={12} sm={8} md={6} lg={4}>
            <Button
              className={classes.button}
              variant="contained"
              onClick={() => {
                ipcRenderer.send('backgroundCompute', accounts, settings);
              }}
              type="button"
            >
              compute
            </Button>
          </Grid>
        </Grid>
      </Grid>
    );
  }
}


Dashboard.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
  accounts: PropTypes.objectOf(PropTypes.object).isRequired,
  settings: PropTypes.object.isRequired,
};

Dashboard.defaultProps = {};

const mapStateToProps = state => ({
  accounts: state.data.accounts,
  settings: state.data.settings,
});

const mapDispatchToProps = dispatch => ({
  setAppBarTitle: titleInput => dispatch({ type: SET_APP_BAR_TITLE, title: titleInput }),
});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps,
  ),
)(Dashboard);
