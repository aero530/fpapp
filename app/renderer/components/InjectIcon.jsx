/**
 * @class InjectIcon
 * @description load and display an icon based on string name
 * @property {Object} classes material ui class
 * @property {string} icon name of icon
 */

import React from 'react';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import * as Icons from '@material-ui/icons';

const styles = theme => ({
  root: {
    backgroundColor: theme.palette.background.paper,
    color: theme.palette.primary.main,
    'font-size': 24,
    [theme.breakpoints.down('xs')]: {
      'font-size': '4.5vw',
      'margin-right': 4,
    },
  },
});

/**
 * @function getIcon
 * @description generate jsx for and svg of the requested icon
 * @param {string} iconName name of icon
 * @param {string} className material ui class
 * @returns {Object} svg icon
 */
function getIcon(iconName, className = '') {
  const SvgIcon = Icons[iconName];
  return (<SvgIcon classes={className}>iconName</SvgIcon>);
}


class InjectIcon extends React.Component {
  render() {
    const {
      classes,
      icon,
    } = this.props;
    return getIcon(icon, classes);
  }
}

InjectIcon.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  icon: PropTypes.string,
};

InjectIcon.defaultProps = {
  icon: 'Brightness1',
};

export default withStyles(styles)(InjectIcon);
