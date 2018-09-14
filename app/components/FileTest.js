import React, { Component } from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import { Link } from 'react-router-dom';
import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';

import routes from '../constants/routes.json';

const { dialog } = require('electron').remote;

const muiStyles = theme => ({
  backButton: {
    position: 'absolute'
  },

  btnGroup: {
    position: 'relative',
    top: '500px',
    width: '480px',
    margin: '0 auto'
  },

  button: {
    margin: theme.spacing.unit
  },

  text: {
    position: 'absolute',
    top: '30%',
    left: '5%',
    fontSize: '1rem',
    fontWeight: 'bold',
    letterSpacing: '-0.025em'
  }
});

class FileTest extends Component {
  handleChange = () => event => {
    const { editFileStateContent } = this.props;

    editFileStateContent(event.target.value);
  };

  handleOpenClick = () => {
    const { modified, filename, saveFile } = this.props;

    if (modified) {
      dialog.showMessageBox(
        {
          type: 'warning', // Can be "none", "info", "error", "question" or "warning".
          buttons: ['Save', 'Discard'], // Array of texts for buttons. On Windows, an empty array will result in one button labeled "OK".
          defaultId: 0, // Index of the button in the buttons array which will be selected by default when the message box opens.
          title: 'File has not been saved', // Title of the message box, some platforms will not show it.
          message: 'Unsaved file', // Content of the message box.
          detail: 'Do you want to save or discard your current edits?' // xtra information of the message.
        },
        selection => {
          if (selection === 0) {
            saveFile(filename);
            this.handleOpen();
          } else if (selection === 1) {
            this.handleOpen();
          }
        }
      );
    } else {
      this.handleOpen();
    }
  };

  handleOpen = () => {
    const { openFile } = this.props;

    dialog.showOpenDialog(
      {
        title: 'Open file',
        buttonLabel: 'open the file',
        filters: [
          { name: 'Text', extensions: ['txt', 'json'] },
          { name: 'All Files', extensions: [] }
        ]
      },
      openFile
    );
  };

  render() {
    const { classes, content, filename, saveFile } = this.props;

    return (
      <div>
        <div className={classes.backButton}>
          <Link to={routes.HOME}>
            <i className="fa fa-arrow-left fa-3x" />
          </Link>
        </div>
        <TextField
          className={classes.text}
          multiline
          rows="6"
          value={content}
          onChange={this.handleChange()}
        />
        <div className={classes.btnGroup}>
          <Button
            className={classes.button}
            variant="contained"
            onClick={() => dialog.showErrorBox('error title', 'error content')}
            type="button"
          >
            error
          </Button>
          <Button
            className={classes.button}
            variant="contained"
            onClick={() => this.handleOpenClick()}
            type="button"
          >
            open
          </Button>
          <Button
            className={classes.button}
            variant="contained"
            onClick={() => saveFile(filename)}
            type="button"
          >
            save
          </Button>
          <Button
            className={classes.button}
            variant="contained"
            onClick={() =>
              dialog.showSaveDialog(
                {
                  title: 'save file',
                  buttonLabel: 'save the file',
                  filters: [
                    { name: 'Text', extensions: ['txt', 'json'] },
                    { name: 'All Files', extensions: [] }
                  ]
                },
                saveFile
              )
            }
            type="button"
          >
            save as
          </Button>
        </div>
      </div>
    );
  }
}

FileTest.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  content: PropTypes.string,
  filename: PropTypes.string,
  modified: PropTypes.bool.isRequired,
  openFile: PropTypes.func.isRequired,
  saveFile: PropTypes.func.isRequired,
  editFileStateContent: PropTypes.func.isRequired
};

FileTest.defaultProps = {
  content: 'default string',
  filename: null
};

export default compose(withStyles(muiStyles))(FileTest);
