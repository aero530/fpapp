// cSpell: ignore reorderable

import React from 'react';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import classNames from 'classnames';
import IconButton from '@material-ui/core/IconButton';
import TextField from '@material-ui/core/TextField';
import Input from '@material-ui/core/Input';

import AddIcon from '@material-ui/icons/Add';
import DeleteIcon from '@material-ui/icons/Delete';
import PromoteIcon from '@material-ui/icons/ArrowUpward';
import DemoteIcon from '@material-ui/icons/ArrowDownward';

import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableFooter from '@material-ui/core/TableFooter';
import TableRow from '@material-ui/core/TableRow';

const styles = theme => ({
  root: {
    marginTop: theme.spacing.unit * 2,
  },
  editableTableStyle: {
  },
  headerRowStyle: {
    height: '1rem',
  },
  footerRowStyle: {
    height: '1rem',
  },
  dataRowStyle: {
    height: '1rem',
  },
  cellStyle: {
    margin: 0,
    padding: 0,
    borderBottom: '0',
  },
  tableButton: {
    fontSize: '1rem',
    height: '1.5rem',
    width: '1.5rem',
    margin: 0,
    marginTop: '0.2rem',
    padding: 0,
  },
  actionCell: {
    width: '80px',
    display: 'flex',
  },
  textField: {
    fontSize: '.75rem',
  },
});

class MuiEditableTable extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      rowData: [],
      colSpec: [],
      reorderable: false,
      onChange: () => {},
    };

    this.onFieldChange = this.onFieldChange.bind(this);
    this.onAddRow = this.onAddRow.bind(this);
    this.onDeleteRow = this.onDeleteRow.bind(this);
    this.onReorderRow = this.onReorderRow.bind(this);
  }

  componentDidMount() {
    const {
      rowData,
      colSpec,
      reorderable,
      onChange,
    } = this.props;

    this.setState(
      {
        rowData: [...rowData],
        colSpec,
        reorderable: reorderable || false,
        onChange,
      },
    );
  }

  renderRowButtons = (index) => {
    const { reorderable, rowData } = this.state;
    const { classes } = this.props;

    return (
      <TableCell className={classNames(classes.cellStyle, classes.actionCell)}>
        <IconButton color="primary" key={`action delete ${index}`} onClick={this.onDeleteRow(index)} className={classes.tableButton}>
          <DeleteIcon fontSize="inherit" />
        </IconButton>

        {reorderable && (index < (rowData.length - 1) && rowData.length > 1) ? (
          <IconButton color="primary" key={`action demote ${index}`} onClick={this.onReorderRow(index, +1)} className={classes.tableButton}>
            <DemoteIcon fontSize="inherit" />
          </IconButton>
        ) : (
          <IconButton color="primary" key={`action demote ${index}`} className={classes.tableButton} disabled>
            <DemoteIcon fontSize="inherit" />
          </IconButton>
        )}

        {reorderable && (index > 0) ? (
          <IconButton color="primary" key={`action promote ${index}`} onClick={this.onReorderRow(index, -1)} className={classes.tableButton}>
            <PromoteIcon fontSize="inherit" />
          </IconButton>
        ) : (
          <IconButton color="primary" key={`action promote ${index}`} className={classes.tableButton} disabled>
            <PromoteIcon fontSize="inherit" />
          </IconButton>
        )}
      </TableCell>
    );
  }

  renderInputField = (column, index, rowData) => {
    const { classes } = this.props;
    if (column.isReadOnly && column.isReadOnly(rowData)) {
      return (<div style={{ width: column.width }} />);
    }

    if (column.inputType === 'TextField') {
      return (
        <Input
          id={column.fieldName + index}
          style={{ width: column.width }}
          value={column.fieldName in rowData ? rowData[column.fieldName] : ''}
          onChange={this.onFieldChange(index, column.fieldName)}
          className={classNames(classes.textField, classes.cellStyle)}
        />
      );
    }

    console.log(`Input field type ${column.inputType} not supported`);
    return null;
  }

  renderRow = (dataRow, index) => {
    const { classes } = this.props;
    const { colSpec } = this.state;

    return (
      <TableRow key={index} className={classes.dataRowStyle}>
        {colSpec.map(col => (
          <TableCell
            className={classes.cellStyle}
            key={col.fieldName + index}
            style={{ width: col.width, fontSize: '5px' }}
          >
            {this.renderInputField(col, index, dataRow)}
          </TableCell>
        ))}
        {this.renderRowButtons(index)}
      </TableRow>
    );
  }

  renderHeader = () => {
    const { colSpec } = this.state;
    const { classes } = this.props;

    return (
      <TableHead>
        <TableRow className={classes.headerRowStyle}>
          {colSpec.map(col => (
            <TableCell
              className={classes.cellStyle}
              key={col.fieldName}
              style={{ width: col.width }}
            >
              {col.title}
            </TableCell>
          ))}
          <TableCell
            key="actionButtons"
            className={classes.cellStyle}
          >
            Actions
          </TableCell>
        </TableRow>
      </TableHead>
    );
  }

  renderFooter = () => {
    const { colSpec } = this.state;
    const { classes } = this.props;

    return (
      <TableFooter>
        <TableRow className={classes.footerRowStyle}>
          {colSpec.map(col => (
            <TableCell
              key={`empty-${col.fieldName}`}
              style={{ width: col.width }}
              className={classes.cellStyle}
            />
          ))}
          <TableCell className={classes.cellStyle}>
            Add row
            <IconButton color="primary" key="action add" onClick={this.onAddRow()} className={classes.tableButton}>
              <AddIcon fontSize="inherit" />
            </IconButton>
          </TableCell>
        </TableRow>
      </TableFooter>
    );
  }

  onAddRow = () => {
    const self = this;
    const { colSpec } = self.state;
    return () => {
      const tempDataRow = [...self.state.rowData];

      const newRow = {};
      colSpec.forEach((column) => {
        newRow[column.fieldName] = column.defaultValue || '';
      });

      tempDataRow.push(newRow);

      self.setState({ rowData: tempDataRow });
      self.state.onChange(tempDataRow);
    };
  }

  onDeleteRow = (rowId) => {
    const self = this;
    return () => {
      const tempDataRow = [...self.state.rowData];

      tempDataRow.splice(rowId, 1);

      self.setState({ rowData: tempDataRow });
      self.state.onChange(tempDataRow);
    };
  }

  onReorderRow = (rowId, direction) => {
    const self = this;
    return () => {
      const tempDataRow = [...self.state.rowData];

      const oldIndex = rowId;
      const newIndex = rowId + direction;

      tempDataRow.splice(newIndex, 0, tempDataRow.splice(oldIndex, 1)[0]);

      self.setState({ rowData: tempDataRow });
      self.state.onChange(tempDataRow);
    };
  }

  onFieldChange = (rowId, fieldName) => {
    const self = this;
    return (event) => {
      const tempDataRow = [...self.state.rowData];
      tempDataRow[rowId][fieldName] = event.target.value;
      self.setState({ rowData: tempDataRow });
      self.state.onChange(tempDataRow);
    };
  }

  render() {
    const { rowData } = this.state;
    const { classes } = this.props;

    return (
      <div className={classes.root}>
        <Table className={classes.editableTableStyle}>
          {this.renderHeader()}
          <TableBody>
            {rowData.map((dataRow, i) => (
              this.renderRow(dataRow, i)
            ))}
          </TableBody>
          {this.renderFooter()}
        </Table>
        <input
          type="hidden"
          id="mui-editable-table-count"
          value={rowData.length}
          readOnly="readOnly"
        />
      </div>
    );
  }
}

MuiEditableTable.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  onChange: PropTypes.func.isRequired,
  rowData: PropTypes.arrayOf(PropTypes.object).isRequired,
  colSpec: PropTypes.arrayOf(PropTypes.object).isRequired,
  reorderable: PropTypes.bool,
};

MuiEditableTable.defaultProps = {
  reorderable: false,
};

export default withStyles(styles)(MuiEditableTable);
