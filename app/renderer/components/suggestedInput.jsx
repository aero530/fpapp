import React from 'react';
import PropTypes from 'prop-types';
import Autosuggest from 'react-autosuggest';
import TextField from '@material-ui/core/TextField';
import Paper from '@material-ui/core/Paper';
import MenuItem from '@material-ui/core/MenuItem';

import Tooltip from '@material-ui/core/Tooltip';
import { withStyles } from '@material-ui/core/styles';

function renderInputComponent(inputProps) {
  const {
    classes,
    title,
    titleLocation,
    inputRef = () => {},
    ref,
    ...other
  } = inputProps;

  return (
    <Tooltip title={title} placement={titleLocation}>
      <TextField
        InputProps={{
          inputRef: node => {
            ref(node);
            inputRef(node);
          },
          classes: {
            input: classes.input
          }
        }}
        {...other}
      />
    </Tooltip>
  );
}

function renderSuggestion(suggestion) {
  return (
    <MenuItem component="div">
      <div>{suggestion.label}</div>
    </MenuItem>
  );
}

function getSuggestions(value, suggestionsList) {
  const inputValue = value.trim().toLowerCase();
  const inputLength = inputValue.length;
  let count = 0;

  return inputLength === 0
    ? []
    : suggestionsList.filter(suggestion => {
        const keep =
          count < 5 &&
          suggestion.label.slice(0, inputLength).toLowerCase() === inputValue;

        if (keep) {
          count += 1;
        }

        return keep;
      });
}

function getSuggestionValue(suggestion) {
  return suggestion.label;
}

const styles = theme => ({
  root: {
    height: 250,
    flexGrow: 1
  },
  container: {
    position: 'relative',
    display: 'inline-flex'
  },
  suggestionsContainerOpen: {
    position: 'absolute',
    zIndex: 1,
    marginTop: theme.spacing.unit * 7,
    left: 0,
    right: 0
  },
  suggestion: {
    display: 'block'
  },
  suggestionsList: {
    margin: 0,
    padding: 0,
    listStyleType: 'none'
  },
  divider: {
    height: theme.spacing.unit * 2
  }
});

class SuggestedInput extends React.Component {
  constructor(props, context) {
    super(props, context);

    // initialize the state
    this.state = {
      enteredText: this.props.value,
      suggestions: []
    };
  }

  handleSuggestionsFetchRequested = ({ value }) => {
    const { suggestionsList } = this.props;
    this.setState({
      suggestions: getSuggestions(value, suggestionsList)
    });
  };

  handleSuggestionsClearRequested = () => {
    this.setState({
      suggestions: []
    });
  };

  handleChange = id => (event, { newValue }) => {
    const { onInputChange } = this.props;
    this.setState({
      enteredText: newValue
    });

    onInputChange(id, newValue);
  };

  render() {
    const {
      classes,
      className,
      id,
      label,
      helperText,
      title,
      titleLocation
    } = this.props;

    const autosuggestProps = {
      renderInputComponent,
      suggestions: this.state.suggestions,
      onSuggestionsFetchRequested: this.handleSuggestionsFetchRequested,
      onSuggestionsClearRequested: this.handleSuggestionsClearRequested,
      getSuggestionValue,
      renderSuggestion
    };

    return (
      <Autosuggest
        {...autosuggestProps}
        id={id}
        inputProps={{
          classes,
          className,
          placeholder: helperText,
          value: this.state.enteredText,
          onChange: this.handleChange(id),
          label,
          title,
          titleLocation
        }}
        theme={{
          container: classes.container,
          suggestionsContainerOpen: classes.suggestionsContainerOpen,
          suggestionsList: classes.suggestionsList,
          suggestion: classes.suggestion
        }}
        renderSuggestionsContainer={options => (
          <Paper {...options.containerProps} square>
            {options.children}
          </Paper>
        )}
      />
    );
  }
}

SuggestedInput.propTypes = {
  classes: PropTypes.object.isRequired,
  onInputChange: PropTypes.func,
  helperText: PropTypes.string,
  value: PropTypes.string,
  label: PropTypes.string
};

SuggestedInput.defaultProps = {
  onInputChange: () => null,
  helperText: 'helper text',
  value: '',
  label: ''
};

export default withStyles(styles)(SuggestedInput);
