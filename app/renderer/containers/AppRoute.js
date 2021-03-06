// cSpell: ignore mixins, WhatsHot

import React from 'react';
import { Route, Switch } from 'react-router-dom';

import compose from 'recompose/compose';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import classNames from 'classnames';

import { withStyles } from '@material-ui/core/styles';
import Drawer from '@material-ui/core/Drawer';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemText from '@material-ui/core/ListItemText';

import Divider from '@material-ui/core/Divider';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import IconButton from '@material-ui/core/IconButton';
import NotificationsIcon from '@material-ui/icons/Notifications';
import MenuIcon from '@material-ui/icons/Menu';
import ChevronLeftIcon from '@material-ui/icons/ChevronLeft';
import Badge from '@material-ui/core/Badge';
import DialogTitle from '@material-ui/core/DialogTitle';
import Dialog from '@material-ui/core/Dialog';
import Typography from '@material-ui/core/Typography';

import {
  mainListItems,
  accountListItems,
} from './menuItems';

import Dashboard from './dashboard';
import Accounts from './accounts';
import Settings from './settings';
import Results from './results';
import Graphs from './graphs';
import GraphsDetail from './graphs/detail';

const drawerWidth = 240;

const styles = theme => ({
  root: {
    display: 'flex',
  },
  toolbar: {
    paddingRight: 24, // keep right padding when drawer closed
  },
  toolbarIcon: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'flex-end',
    padding: '0 8px',
    ...theme.mixins.toolbar,
  },
  appBar: {
    zIndex: theme.zIndex.drawer + 1,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
  },
  appBarShift: {
    marginLeft: drawerWidth,
    width: `calc(100% - ${drawerWidth}px)`,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  },
  menuButton: {
    marginLeft: 12,
    marginRight: 36,
  },
  menuButtonHidden: {
    display: 'none',
  },
  title: {
    flexGrow: 1,
  },
  drawerPaper: {
    position: 'relative',
    whiteSpace: 'nowrap',
    width: drawerWidth,
    transition: theme.transitions.create('width', {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  },
  drawerPaperClose: {
    overflowX: 'hidden',
    transition: theme.transitions.create('width', {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
    width: theme.spacing(7),
    [theme.breakpoints.up('sm')]: {
      width: theme.spacing(9),
    },
  },
  appBarSpacer: theme.mixins.toolbar,
  content: {
    flexGrow: 1,
    padding: theme.spacing(3),
    height: '100vh',
    overflow: 'auto',
  },
  chartContainer: {
    marginLeft: -22,
  },
  tableContainer: {
    height: 320,
  },
});

class AppRoute extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      drawerOpen: false,
      dialogOpen: false,
    };
  }

  handleDialogOpen = () => {
    this.setState({ dialogOpen: true });
  };

  handleDialogClose = () => {
    this.setState({ dialogOpen: false });
  };

  handleDrawerOpen = () => {
    this.setState({ drawerOpen: true });
  };

  handleDrawerClose = () => {
    this.setState({ drawerOpen: false });
  };

  render() {
    const { classes, appBarTitle, errorCount, errors } = this.props;
    const { drawerOpen, dialogOpen } = this.state;

    return (
      <div className={classes.root}>
        <AppBar
          position="absolute"
          className={classNames(
            classes.appBar,
            drawerOpen && classes.appBarShift,
          )}
        >
          <Toolbar>
            <IconButton
              color="inherit"
              aria-label="open drawer"
              onClick={this.handleDrawerOpen}
              className={classNames(
                classes.menuButton,
                drawerOpen && classes.menuButtonHidden,
              )}
            >
              <MenuIcon />
            </IconButton>
            <Typography
              variant="h6"
              color="inherit"
              noWrap
              className={classes.title}
            >
              {appBarTitle}
            </Typography>

            {errorCount > 0 ? (
              <IconButton color="inherit" onClick={this.handleDialogOpen}>
                <Badge badgeContent={errorCount} color="secondary">
                  <NotificationsIcon />
                </Badge>
              </IconButton>
            ) : (
              <span />
            )}
          </Toolbar>
        </AppBar>
        <Drawer
          variant="permanent"
          classes={{
            paper: classNames(
              classes.drawerPaper,
              !drawerOpen && classes.drawerPaperClose,
            ),
          }}
          open={drawerOpen}
        >
          <div className={classes.toolbarIcon}>
            <IconButton onClick={this.handleDrawerClose}>
              <ChevronLeftIcon />
            </IconButton>
          </div>
          <Divider />
          <List>{mainListItems}</List>
          <Divider />
          <List>{accountListItems}</List>
        </Drawer>
        <main className={classes.content}>
          <div className={classes.appBarSpacer} />
          <div className={classes.toolbar} />
          <Switch>
            <Route exact path="/app" component={Dashboard} />
            <Route exact path="/app/results" component={Results} />
            <Route exact path="/app/graphs/detail" component={GraphsDetail} />
            <Route exact path="/app/graphs" component={Graphs} />
            <Route exact path="/app/settings" component={Settings} />
            <Route exact path="/app/accounts/:type" component={Accounts} />
          </Switch>
        </main>

        <Dialog
          onClose={this.handleDialogClose}
          open={dialogOpen}
        >
          <DialogTitle id="simple-dialog-title">Analysis Errors</DialogTitle>
          <div>
            <List>
              {errors.map(error => (
                <ListItem key={`error-item-${error.title}`}>
                  <ListItemText
                    primary={error.title}
                    secondary={error.message}
                  />
                </ListItem>
              ))}
            </List>
          </div>
        </Dialog>
      </div>
    );
  }
}

AppRoute.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  appBarTitle: PropTypes.string,
  errorCount: PropTypes.number,
  errors: PropTypes.arrayOf(PropTypes.object),
};

AppRoute.defaultProps = {
  errorCount: 0,
  errors: [],
  appBarTitle: '',
};

const mapStateToProps = (state) => ({
  appBarTitle: state.app.appBarTitle,
  errorCount: state.results.errorCount,
  errors: state.results.errors,
});

const mapDispatchToProps = () => ({});

export default compose(
  withStyles(styles),
  connect(
    mapStateToProps,
    mapDispatchToProps,
  ),
)(AppRoute);
