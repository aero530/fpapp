// cSpell:ignore Unmount

import React from 'react';
import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

const styles = theme => ({
  root: {
    width: '100%',
    marginTop: theme.spacing.unit * 3
  },
  paper: {
    position: 'absolute',
    width: theme.spacing.unit * 50,
    backgroundColor: theme.palette.background.paper,
    boxShadow: theme.shadows[5],
    padding: theme.spacing.unit * 4
  },
  button: {
    margin: theme.spacing.unit
  }
});

class Settings extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      isRemoveModalOpen: false
    };
  }

  render() {
    const { classes } = this.props;

    return (
      <Paper className={classes.root}>
        <Typography variant="title" id="modal-title">
          Settings
        </Typography>
        age_retire = 67 age_die = 100 year_born = 1982 year_start = 2014 #
        10/27/2013 inflation_base = 3.0 tax_income = 20.0 tax_capitalgains =
        15.0 retirement_costofliving = 85.0 # setting this to 100% will cover
        all of the expense category items which currently does
      </Paper>
    );
  }
}

Settings.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired
};

Settings.defaultProps = {};

const mapStateToProps = state => ({});

const mapDispatchToProps = dispatch => ({});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps
  )
)(Settings);
