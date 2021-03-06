import React from 'react';
import { Link } from 'react-router-dom';

import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import ListSubheader from '@material-ui/core/ListSubheader';
import DashboardIcon from '@material-ui/icons/Dashboard';
import BarChartIcon from '@material-ui/icons/BarChart';
import ShowChartIcon from '@material-ui/icons/ShowChart';
import TableChartIcon from '@material-ui/icons/TableChart';
import HomeIcon from '@material-ui/icons/Home';
import CreditCardIcon from '@material-ui/icons/CreditCard';
import LocalPharmacyIcon from '@material-ui/icons/LocalPharmacy';
import AttachMoneyIcon from '@material-ui/icons/AttachMoney';
import SchoolIcon from '@material-ui/icons/School';
import SaveAltIcon from '@material-ui/icons/SaveAlt';
import HotTubIcon from '@material-ui/icons/HotTub';
import ShareIcon from '@material-ui/icons/Share';
import SettingsIcon from '@material-ui/icons/Settings';
import SupervisorAccountIcon from '@material-ui/icons/SupervisorAccount';

export const mainListItems = (
  <div>
    <ListItem dense button component={Link} to="/app">
      <ListItemIcon>
        <DashboardIcon />
      </ListItemIcon>
      <ListItemText primary="Dashboard" />
    </ListItem>

    <ListItem dense button component={Link} to="/app/settings">
      <ListItemIcon>
        <SettingsIcon />
      </ListItemIcon>
      <ListItemText primary="Settings" />
    </ListItem>

    <ListItem dense button component={Link} to="/app/graphs">
      <ListItemIcon>
        <BarChartIcon />
      </ListItemIcon>
      <ListItemText primary="Graphs" />
    </ListItem>

    <ListItem dense button component={Link} to="/app/graphs/detail">
      <ListItemIcon>
        <ShowChartIcon />
      </ListItemIcon>
      <ListItemText primary="Detail Graphs" />
    </ListItem>

    <ListItem dense button component={Link} to="/app/results">
      <ListItemIcon>
        <TableChartIcon />
      </ListItemIcon>
      <ListItemText primary="Results" />
    </ListItem>
  </div>
);

export const accountListItems = (
  <div>
    <ListSubheader inset>Accounts</ListSubheader>
    <ListItem dense button component={Link} to="/app/accounts/income">
      <ListItemIcon>
        <AttachMoneyIcon />
      </ListItemIcon>
      <ListItemText primary="Income" />
    </ListItem>
    <ListItem dense button component={Link} to="/app/accounts/retirement">
      <ListItemIcon>
        <HotTubIcon />
      </ListItemIcon>
      <ListItemText primary="Retirement" />
    </ListItem>
    <ListItem dense button component={Link} to="/app/accounts/ssa">
      <ListItemIcon>
        <SupervisorAccountIcon />
      </ListItemIcon>
      <ListItemText primary="Social Security" />
    </ListItem>
    <ListItem dense button component={Link} to="/app/accounts/college">
      <ListItemIcon>
        <SchoolIcon />
      </ListItemIcon>
      <ListItemText primary="College" />
    </ListItem>
    <ListItem dense button component={Link} to="/app/accounts/expense">
      <ListItemIcon>
        <CreditCardIcon />
      </ListItemIcon>
      <ListItemText primary="Expense" />
    </ListItem>
    <ListItem dense button component={Link} to="/app/accounts/loan">
      <ListItemIcon>
        <ShareIcon />
      </ListItemIcon>
      <ListItemText primary="Loan" />
    </ListItem>
    <ListItem dense button component={Link} to="/app/accounts/mortgage">
      <ListItemIcon>
        <HomeIcon />
      </ListItemIcon>
      <ListItemText primary="Mortgage" />
    </ListItem>
    <ListItem dense button component={Link} to="/app/accounts/savings">
      <ListItemIcon>
        <SaveAltIcon />
      </ListItemIcon>
      <ListItemText primary="Savings" />
    </ListItem>
    <ListItem dense button component={Link} to="/app/accounts/hsa">
      <ListItemIcon>
        <LocalPharmacyIcon />
      </ListItemIcon>
      <ListItemText primary="HSA" />
    </ListItem>
  </div>
);
