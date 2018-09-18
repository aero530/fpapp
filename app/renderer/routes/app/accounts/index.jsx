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

class Accounts extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      isRemoveModalOpen: false
    };
  }

  render() {
    const { classes, match } = this.props;

    return (
      <Paper className={classes.root}>
        <Typography variant="title" id="modal-title">
          {match.params.type}
        </Typography>
      </Paper>
    );
  }
}

Accounts.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired
};

Accounts.defaultProps = {};

const mapStateToProps = state => ({});

const mapDispatchToProps = dispatch => ({});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps
  )
)(Accounts);
