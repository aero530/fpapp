import React, { Component } from 'react';
import compose from 'recompose/compose';
import PropTypes from 'prop-types';
import { withStyles } from '@material-ui/core/styles';
import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';

const { dialog } = require('electron').remote;

const muiStyles = theme => ({
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

  render() {
    const { classes, content } = this.props;

    return (
      <div>
        <TextField
          className={classes.text}
          multiline
          rows="10"
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
        </div>
      </div>
    );
  }
}

FileTest.propTypes = {
  classes: PropTypes.objectOf(PropTypes.string).isRequired,
  content: PropTypes.string,
  editFileStateContent: PropTypes.func.isRequired
};

FileTest.defaultProps = {
  content: 'default string',
};

export default compose(withStyles(muiStyles))(FileTest);
