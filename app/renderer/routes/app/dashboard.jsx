import React from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import Grid from '@material-ui/core/Grid';

const styles = () => ({
  root: {
  },
});

class Dashboard extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
    };
  }

  render() {
    const {
      classes,
    } = this.props;

    return (
      <Grid container className={classes.root} spacing={16}>
        <Grid item xs={12} />
      </Grid>
    );
  }
}

Dashboard.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
};

export default compose(withStyles(styles))(Dashboard);
