import React from 'react';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import classNames from 'classnames';
import Button from '@material-ui/core/Button';
import AddIcon from '@material-ui/icons/Add';

const styles = theme => ({
  button: {
    margin: theme.spacing.unit
  }
});

function FloatingActionButton(props) {
  const { classes, className, onClick } = props;
  return (
    <Button
      variant="fab"
      color="primary"
      aria-label="Add"
      className={classNames(classes.button, className)}
      onClick={() => {
        onClick();
      }}
    >
      <AddIcon />
    </Button>
  );
}

FloatingActionButton.propTypes = {
  classes: PropTypes.object.isRequired,
  onClick: PropTypes.func.isRequired
};

export default withStyles(styles)(FloatingActionButton);
