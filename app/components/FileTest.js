import React, { Component } from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import { Link } from 'react-router-dom';
import Button from '@material-ui/core/Button';

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
  render() {
    const { classes, content, openFile } = this.props;

    return (
      <div>
        <div className={classes.backButton}>
          <Link to={routes.HOME}>
            <i className="fa fa-arrow-left fa-3x" />
          </Link>
        </div>
        <div className={classes.text}>{content}</div>
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
            onClick={() =>
              dialog.showMessageBox({
                type: 'info', // Can be "none", "info", "error", "question" or "warning".
                buttons: ['asdf', 'dfgh'], // Array of texts for buttons. On Windows, an empty array will result in one button labeled "OK".
                defaultId: 0, // Index of the button in the buttons array which will be selected by default when the message box opens.
                title: 'This is a title', // Title of the message box, some platforms will not show it.
                message: 'This is the message', // Content of the message box.
                detail: 'This is the detail' // xtra information of the message.
              })
            }
            type="button"
          >
            message
          </Button>
          <Button
            className={classes.button}
            variant="contained"
            onClick={() =>
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
              )
            }
            type="button"
          >
            open
          </Button>
          <Button
            className={classes.button}
            variant="contained"
            onClick={() =>
              dialog.showSaveDialog({
                title: 'save file',
                buttonLabel: 'save the file',
                filters: [
                  { name: 'Text', extensions: ['txt', 'json'] },
                  { name: 'All Files', extensions: [] }
                ]
              })
            }
            type="button"
          >
            save
          </Button>
        </div>
      </div>
    );
  }
}

FileTest.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  content: PropTypes.string,
  openFile: PropTypes.func.isRequired
};

FileTest.defaultProps = {
  content: 'default string'
};

export default compose(withStyles(muiStyles))(FileTest);
