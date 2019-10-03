// cSpell:ignore Unmount

import React from 'react';
import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';

import classNames from 'classnames';

import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

import Tooltip from '@material-ui/core/Tooltip';
import TextField from '@material-ui/core/TextField';

import {
  NumberFormatPercentage,
  NumberFormatYear,
} from '../../components/numberFormaters';

import { SET_APP_BAR_TITLE } from '../../actions/app';
import { UPDATE_SETTING } from '../../actions/data';

const styles = theme => ({
  root: {
    width: '100%',
    marginTop: theme.spacing(3),
  },
  paper: {
    width: '100%',
    backgroundColor: theme.palette.background.paper,
    boxShadow: theme.shadows[5],
    padding: theme.spacing(4),
    marginBottom: theme.spacing(4),
  },
  button: {
    margin: theme.spacing(1),
  },
});

class Settings extends React.Component {
  componentDidMount() {
    const { setAppBarTitle } = this.props;
    setAppBarTitle('Settings');
  }

  render() {
    const { classes, settings, changeSetting } = this.props;

    return (
      <div>
        <Paper className={classes.paper}>
          <Typography variant="h6" id="modal-title">
            Time
          </Typography>

          <Tooltip title="Calendar year when you will start pulling from retirement accounts">
            <TextField
              id="ageRetire"
              label="Retirement Age"
              className={classNames(classes.margin, classes.textField)}
              value={settings.ageRetire}
              InputProps={{ inputComponent: NumberFormatYear }}
              onChange={(event) => {
                changeSetting(event.target.id, parseFloat(event.target.value));
                this.forceUpdate();
              }}
            />
          </Tooltip>

          <Tooltip title="Age at which you will no longer need to pull money from your retirement accounts">
            <TextField
              id="ageDie"
              label="Termination Age"
              className={classNames(classes.margin, classes.textField)}
              value={settings.ageDie}
              InputProps={{ inputComponent: NumberFormatYear }}
              onChange={(event) => {
                changeSetting(event.target.id, parseFloat(event.target.value));
                this.forceUpdate();
              }}
            />
          </Tooltip>

          <Tooltip title="Year in which you were born">
            <TextField
              id="yearBorn"
              label="Birth Year"
              className={classNames(classes.margin, classes.textField)}
              value={settings.yearBorn}
              InputProps={{ inputComponent: NumberFormatYear }}
              onChange={(event) => {
                changeSetting(event.target.id, parseFloat(event.target.value));
                this.forceUpdate();
              }}
            />
          </Tooltip>

          <Tooltip title="Current calendar year">
            <TextField
              id="yearStart"
              label="Beginning Year"
              className={classNames(classes.margin, classes.textField)}
              value={settings.yearStart}
              InputProps={{ inputComponent: NumberFormatYear }}
              onChange={(event) => {
                changeSetting(event.target.id, parseFloat(event.target.value));
                this.forceUpdate();
              }}
            />
          </Tooltip>
        </Paper>

        <Paper className={classes.paper}>
          <Typography variant="h6" id="modal-title">
            Inflation
          </Typography>
          <Tooltip title="Inflation rate as a percentage. (Used to increase the cost of living)">
            <TextField
              id="inflationBase"
              label="Inflation %"
              className={classNames(classes.margin, classes.textField)}
              value={settings.inflationBase}
              InputProps={{ inputComponent: NumberFormatPercentage }}
              onChange={(event) => {
                changeSetting(event.target.id, event.target.floatValue);
                this.forceUpdate();
              }}
            />
          </Tooltip>
          <Tooltip title="Percent of income which goes to taxes">
            <TextField
              id="taxIncome"
              label="Income Tax Rate %"
              className={classNames(classes.margin, classes.textField)}
              value={settings.taxIncome}
              InputProps={{ inputComponent: NumberFormatPercentage }}
              onChange={(event) => {
                changeSetting(event.target.id, event.target.floatValue);
                this.forceUpdate();
              }}
            />
          </Tooltip>
          <Tooltip title="Capital gains taxes (tax on interest earned)">
            <TextField
              id="taxCapitalGains"
              label="Capital Gains Tax Rate %"
              className={classNames(classes.margin, classes.textField)}
              value={settings.taxCapitalGains}
              InputProps={{ inputComponent: NumberFormatPercentage }}
              onChange={(event) => {
                changeSetting(event.target.id, event.target.floatValue);
                this.forceUpdate();
              }}
            />
          </Tooltip>
          <Tooltip title="Cost of living during retirement relative to current cost of living as a percentage. Note that it is already accounted for that you will not be saving for retirement or college funds.  This is a blanket decrease in the amount of money spent on all expenses (including food, housing, medical, travel, etc.)">
            <TextField
              id="retirementCostOfLiving"
              label="Cost of Living %"
              className={classNames(classes.margin, classes.textField)}
              value={settings.retirementCostOfLiving}
              InputProps={{ inputComponent: NumberFormatPercentage }}
              onChange={(event) => {
                console.log(event.target.id);
                changeSetting(event.target.id, event.target.floatValue);
                this.forceUpdate();
              }}
            />
          </Tooltip>
        </Paper>
      </div>
    );
  }
}
Settings.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  setAppBarTitle: PropTypes.func.isRequired,
  settings: PropTypes.objectOf(PropTypes.number || PropTypes.string).isRequired,
  changeSetting: PropTypes.func.isRequired,
};

Settings.defaultProps = {};

const mapStateToProps = state => ({
  settings: state.data.settings,
});

const mapDispatchToProps = dispatch => ({
  setAppBarTitle: titleInput => dispatch({ type: SET_APP_BAR_TITLE, title: titleInput }),
  changeSetting: (nameInput, valueInput) => dispatch({ type: UPDATE_SETTING, name: nameInput, value: valueInput }),
});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps,
  ),
)(Settings);
