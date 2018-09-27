import React from 'react';
import NumberFormat from 'react-number-format';
import PropTypes from 'prop-types';

export function NumberFormatPercentage(props) {
  const { inputRef, onChange, ...other } = props;

  return (
    <NumberFormat
      {...other}
      getInputRef={inputRef}
      onValueChange={(values, event) => {
        onChange({
          target: {
            value: values.value,
            floatValue: values.floatValue,
            id: event.target.id,
          },
        });
      }}
      suffix="%"
    />
  );
}

NumberFormatPercentage.propTypes = {
  inputRef: PropTypes.func.isRequired,
  onChange: PropTypes.func.isRequired,
};

function formatDollarPercentage(val) {
  if (parseFloat(val) <= 100) {
    return `${val}%`;
  }
  return `$${val.replace(/\B(?=(\d{3})+(?!\d))/g, ',')}`;
}

export function NumberFormatDollarPercentage(props) {
  const { inputRef, onChange, ...other } = props;

  return (
    <NumberFormat
      {...other}
      getInputRef={inputRef}
      onValueChange={(values, event) => {
        onChange({
          target: {
            value: values.value,
            floatValue: values.floatValue,
            id: event.target.id,
          },
        });
      }}
      format={formatDollarPercentage}
      thousandSeparator
    />
  );
}

NumberFormatDollarPercentage.propTypes = {
  inputRef: PropTypes.func.isRequired,
  onChange: PropTypes.func.isRequired,
};

export function NumberFormatDollar(props) {
  const { inputRef, onChange, ...other } = props;

  return (
    <NumberFormat
      {...other}
      getInputRef={inputRef}
      onValueChange={(values, event) => {
        onChange({
          target: {
            value: values.value,
            floatValue: values.floatValue,
            id: event.target.id,
          },
        });
      }}
      thousandSeparator
      prefix="$"
    />
  );
}

NumberFormatDollar.propTypes = {
  inputRef: PropTypes.func.isRequired,
  onChange: PropTypes.func.isRequired,
};

export function NumberFormatYear(props) {
  const { inputRef, onChange, ...other } = props;

  return (
    <NumberFormat
      {...other}
      getInputRef={inputRef}
      onValueChange={(values, event) => {
        onChange({
          target: {
            value: values.value,
            floatValue: values.floatValue,
            id: event.target.id,
          },
        });
      }}
      format="####"
    />
  );
}

NumberFormatYear.propTypes = {
  inputRef: PropTypes.func.isRequired,
  onChange: PropTypes.func.isRequired,
};
