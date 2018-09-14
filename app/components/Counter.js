import React, { Component } from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import { Link } from 'react-router-dom';

import routes from '../constants/routes.json';

const muiStyles = () => ({
  backButton: {
    position: 'absolute'
  },

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
    width: '480px',
    margin: '0 auto'
  },

  btn: {
    fontSize: '1.6rem',
    fontWeight: 'bold',
    backgroundColor: '#fff',
    borderRadius: '50%',
    margin: '10px',
    width: '100px',
    height: '100px',
    opacity: '0.7',
    cursor: 'pointer',
    fontFamily: 'Arial, Helvetica, Helvetica Neue, sans-serif',
    '&:hover': {
      color: 'white',
      backgroundColor: 'rgba(0, 0, 0, 0.5)'
    }
  }
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
        <div className={classes.backButton}>
          <Link to={routes.HOME}>
            <i className="fa fa-arrow-left fa-3x" />
          </Link>
        </div>
        <div className={`counter ${classes.counter}`}>{counter}</div>
        <div className={classes.btnGroup}>
          <button className={classes.btn} onClick={increment} type="button">
            <i className="fa fa-plus" />
          </button>
          <button className={classes.btn} onClick={decrement} type="button">
            <i className="fa fa-minus" />
          </button>
          <button
            className={classes.btn}
            onClick={incrementIfOdd}
            type="button"
          >
            odd
          </button>
          <button
            className={classes.btn}
            onClick={() => incrementAsync()}
            type="button"
          >
            async
          </button>
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
