import React from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import Button from '@material-ui/core/Button';

import Grid from '@material-ui/core/Grid';

import { ipcRenderer } from 'electron';

const styles = () => ({
  root: {}
});

class Dashboard extends React.Component {
  constructor(props) {
    super(props);
    this.state = {};
  }

  render() {
    const { classes } = this.props;

    return (
      <Grid container className={classes.root} spacing={16}>
        <Grid item xs={12} />
        <Button
          className={classes.button}
          variant="contained"
          onClick={() => {
            ipcRenderer.send(
              'for-background',
              'do something for a few seconds'
            );
          }}
          type="button"
        >
          test
        </Button>
      </Grid>
    );
  }
}

Dashboard.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired
};

export default compose(withStyles(styles))(Dashboard);
