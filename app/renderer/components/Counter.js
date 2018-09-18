import React, { Component } from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import Button from '@material-ui/core/Button';

import DecrementIcon from '@material-ui/icons/Remove';
import IncrementIcon from '@material-ui/icons/Add';

const muiStyles = (theme) => ({

  counter: {
    position: 'absolute',
    top: '30%',
    left: '45%',
    fontSize: '10rem',
    fontWeight: 'bold',
    letterSpacing: '-0.025em'
  },

  btnGroup: {
    position: 'relative',
    top: '500px',
    width: '500px',
    margin: '0 auto'
  },

  button: {
    margin: theme.spacing.unit,
  },
  extendedIcon: {
    marginRight: theme.spacing.unit,
  },

});

class Counter extends Component {
  render() {
    const {
      increment,
      incrementIfOdd,
      incrementAsync,
      decrement,
      counter,
      classes
    } = this.props;
    return (
      <div>
        <div className={`counter ${classes.counter}`}>{counter}</div>
        <div className={classes.btnGroup}>
          <Button className={classes.button} onClick={increment}>
            <IncrementIcon />
          </Button>
          <Button className={classes.button} onClick={decrement}>
            <DecrementIcon />
          </Button>
          <Button
          className={classes.button}
            onClick={incrementIfOdd}
          >
            odd
          </Button>
          <Button
          className={classes.button}
            onClick={() => incrementAsync()}
          >
            async
          </Button>
        </div>
      </div>
    );
  }
}

Counter.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  increment: PropTypes.func.isRequired,
  incrementIfOdd: PropTypes.func.isRequired,
  incrementAsync: PropTypes.func.isRequired,
  decrement: PropTypes.func.isRequired,
  counter: PropTypes.number.isRequired
};

export default compose(withStyles(muiStyles))(Counter);
