import React from 'react';
import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';

import Grid from '@material-ui/core/Grid';

import { SET_APP_BAR_TITLE } from '../actions/app';


const styles = () => ({
  root: {},
});

class Dashboard extends React.Component {
  componentDidMount() {
    const { setAppBarTitle } = this.props;
    setAppBarTitle('');
  }

  render() {
    const { classes } = this.props;

    return (
      <Grid container className={classes.root} spacing={10}>
        <Grid item xs={12} />
        <Grid container justify="center" spacing={10}>
          <Grid item xs={12} sm={8} md={6} lg={4}>
            <Typography variant="body1" gutterBottom align="center">
              1) Load account data
            </Typography>
            <Typography variant="body1" gutterBottom align="center">
              2) Enjoy graphs
            </Typography>
          </Grid>
        </Grid>
      </Grid>
    );
  }
}

Dashboard.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
};

Dashboard.defaultProps = {};

const mapStateToProps = () => ({});

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
