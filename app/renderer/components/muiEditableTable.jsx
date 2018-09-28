import React from 'react';
import IconButton from '@material-ui/core/IconButton';
import TextField from '@material-ui/core/TextField';
import AddIcon from '@material-ui/icons/Add';
import DeleteIcon from '@material-ui/icons/Delete';
import PromoteIcon from '@material-ui/icons/ArrowUpward';
import DemoteIcon from '@material-ui/icons/ArrowDownward';

class MuiEditableTable extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      containerStyle: this.props.containerStyle || {},
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

    const buttons = [this.iconButton(index, 'delete', this.onDeleteRow(index), <DeleteIcon />)];

    if (reorderable) {
      if (index < (rowData.length - 1) && rowData.length > 1) {
        buttons.push(this.iconButton(index, 'demote', this.onReorderRow(index, +1), <DemoteIcon />));
      }
      if (index > 0) {
        buttons.push(this.iconButton(index, 'promote', this.onReorderRow(index, -1), <PromoteIcon />));
      }
    }

    return (
      <div>
        {buttons}
      </div>
    );
  }

  iconButton = (rowKey, action, clickEvent, muiIcon) => {
    return (
      <div className="cell action" key={`action ${action} ${rowKey}`} style={{ width: '45px', display: 'inline' }}>
        <IconButton
          className={`action-button ${action}-row-button${rowKey}`}
          primary
          onClick={clickEvent}
          style={{ minWidth: '45px' }}
        >
          {muiIcon}
        </IconButton>
      </div>
    );
  }

  renderInputField = (column, index, rowData) => {
    if (column.isReadOnly && column.isReadOnly(rowData)) {
      return (<div style={{ width: column.width }} />);
    }

    if (column.inputType === 'TextField') {
      return (
        <TextField
          id={column.fieldName + index}
          style={{ width: column.width }}
          value={column.fieldName in rowData ? rowData[column.fieldName] : ''}
          onChange={this.onFieldChange(index, column.fieldName)}
        />
      );
    }

    console.log(`Input field type ${column.inputType} not supported`);
  }

  renderRow = (dataRow, index) => {
    const dataRowStyle = {
      width: '100%',
      display: 'flex',
      flexFlow: 'row nowrap',
      border: '0',
      height: '40px',
      borderBottom: '1px solid rgb(224, 224, 224)',
    };
    const { colSpec } = this.state;

    return (
      <div className="mui-editable-table-row" key={index} style={dataRowStyle}>
        {colSpec.map(col => (
          <div
            className={`cell ${col.fieldName}`}
            key={col.fieldName + index}
            style={{ width: col.width }}
          >
            {this.renderInputField(col, index, dataRow)}
          </div>
        ))}
        {this.renderRowButtons(index)}
      </div>
    );
  }

  renderHeader = () => {
    const headerRowStyle = {
      width: '100%',
      display: 'flex',
      flexFlow: 'row nowrap',
      border: '0',
      height: '40px',
      color: 'rgb(158, 158, 158)',
      fontSize: '12px',
      borderBottom: '1px solid #ccc',
      paddingTop: '10px',
    };

    const { colSpec } = this.state;

    return (
      <div className="mui-editable-table-row header-row" style={headerRowStyle}>
        {colSpec.map(col => (
          <div
            className={`row-cell header-cell ${col.fieldName}`}
            key={col.fieldName}
            style={{ width: col.width }}
          >
            {col.title}
          </div>
        ))}
        <div className="row-cell header-cell action" style={{ width: '100px' }}>
          {this.iconButton('', 'add', this.onAddRow(), <AddIcon />)}
        </div>
      </div>
    );
  }

  onAddRow = () => {
    const self = this;
    const { colSpec } = self.state;
    return () => {
      const tempDataRow = [...self.state.rowData];

      const newRow = {};
      colSpec.map(column => newRow[column.fieldName] = column.defaultValue || '');

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
    const editableTableStyle = {
      display: 'flex',
      flexFlow: 'column nowrap',
      justifyContent: 'space-between',
      alignItems: 'center',
      fontFamily: 'Roboto, sans-serif',
    };

    const { containerStyle, rowData } = this.state;

    return (
      <div className="container" style={containerStyle}>
        <div className="mui-editable-table" style={editableTableStyle}>
          {this.renderHeader()}

          {rowData.map((dataRow, i) => (
            this.renderRow(dataRow, i)
          ))}
          <input
            type="hidden"
            id="mui-editable-table-count"
            value={rowData.length}
            readOnly="readOnly"
          />
        </div>
      </div>
    );
  }
}

export default MuiEditableTable;
