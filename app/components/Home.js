import React, { Component } from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import { Link } from 'react-router-dom';
import routes from '../constants/routes.json';

const muiStyles = theme => ({
  container: {
    position: 'absolute',
    top: '30%',
    left: '10px',
    textAlign: 'center'
  },
  h2: {
    fontSize: '5rem',
    color: theme.palette.secondary.main
  },
  link: {
    fontSize: '1.4rem'
  }
});

class Home extends Component {
  render() {
    const { classes } = this.props;

    return (
      <div className={classes.container} data-tid="container">
        <h2 className={classes.h2}>Home</h2>
        <Link className={classes.link} to={routes.COUNTER}>
          to Counter
        </Link>
        <br />
        <Link className={classes.link} to={routes.FILETEST}>
          to file test
        </Link>
      </div>
    );
  }
}

Home.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired
};

export default compose(withStyles(muiStyles))(Home);
