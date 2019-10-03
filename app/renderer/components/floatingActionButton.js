import React from 'react';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import classNames from 'classnames';
import Button from '@material-ui/core/Button';
import Fab from '@material-ui/core/Fab';
import AddIcon from '@material-ui/icons/Add';

const styles = theme => ({
  button: {
    margin: theme.spacing(1),
  },
  fab: {
    margin: theme.spacing(1),
  },
});

/**
 * @function FloatingActionButton
 * @description generate jsx for a floating action add button
 * @param {function} onClick what happens when the button is clicked
 * @returns {React Component} floating action button
 */

function FloatingActionButton(props) {
  const { classes, className, onClick } = props;
  return (
    <Fab
      
      color="primary"
      aria-label="Add"
      className={classNames(classes.button, className)}
      onClick={() => {
        onClick();
      }}
    >
      <AddIcon />
    </Fab>
  );
}

FloatingActionButton.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  onClick: PropTypes.func.isRequired,
  className: PropTypes.string,
};

FloatingActionButton.defaultProps = {
  className: '',
};

export default withStyles(styles)(FloatingActionButton);
