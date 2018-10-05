import React from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';

class CustomTooltip extends React.PureComponent {
  isNumOrStr = (value) => {
    if ((typeof value === 'number') || (typeof value === 'string')) {
      return true;
    }
    return false;
  };

  yearlySum = (payload) => {
    let total = 0;
    payload.forEach((object) => {
      total += object.value;
    });
    return total;
  }

  renderContent() {
    const { payload, separator, formatter, itemStyle, itemSorter } = this.props;

    if (payload && payload.length) {
      const listStyle = { padding: 0, margin: 0 };

      const items = payload.sort(itemSorter)
        .map((entry, i) => {
          const finalItemStyle = {
            display: 'block',
            paddingTop: 2,
            paddingBottom: 2,
            color: entry.color || '#000',
            ...itemStyle,
          };
          const hasName = this.isNumOrStr(entry.name);
          const finalFormatter = entry.formatter || formatter;

          if (entry.value !== 0) { // don't show an entry in the tooltip if the value is zero
            return (
              <li className="recharts-tooltip-item" key={`tooltip-item-${i}`} style={finalItemStyle}>
                {hasName ? <span className="recharts-tooltip-item-name">{entry.name}</span> : null}
                {hasName ? <span className="recharts-tooltip-item-separator">{separator}</span> : null}
                <span className="recharts-tooltip-item-value">
                  {finalFormatter ? finalFormatter(entry.value, entry.name, entry, i) : entry.value}
                </span>
                <span className="recharts-tooltip-item-unit">{entry.unit || ''}</span>
              </li>
            );
          }
          return null;
        });

      return <ul className="recharts-tooltip-item-list" style={listStyle}>{items}</ul>;
    }

    return null;
  }


  render() {
    const {
      active,
      wrapperClassName,
      contentStyle,
      labelClassName,
      labelStyle,
      label,
      labelFormatter,
      formatter,
    } = this.props;

    const finalStyle = {
      margin: 0,
      padding: 10,
      backgroundColor: '#fff',
      border: '1px solid #ccc',
      whiteSpace: 'nowrap',
      ...contentStyle,
    };
    const finalLabelStyle = {
      margin: 0,
      ...labelStyle,
    };

    const hasLabel = this.isNumOrStr(label);
    let finalLabel = hasLabel ? label : '';
    const wrapperCN = classNames('recharts-default-tooltip', wrapperClassName);
    const labelCN = classNames('recharts-tooltip-label', labelClassName);

    if (hasLabel && labelFormatter) { finalLabel = labelFormatter(label); }


    if (active) {
      const { payload } = this.props;
      if (payload) {
        return (
          <div className={wrapperCN} style={finalStyle}>
            <p className={labelCN} style={finalLabelStyle}>{`${finalLabel} total: ${formatter(this.yearlySum(payload))}`}</p>
            {this.renderContent()}
          </div>
        );
      }
      return null;
    }
    return null;
  }
}

CustomTooltip.propTypes = {
  separator: PropTypes.string,
  wrapperClassName: PropTypes.string,
  labelClassName: PropTypes.string,
  formatter: PropTypes.func,
  contentStyle: PropTypes.object,
  itemStyle: PropTypes.object,
  labelStyle: PropTypes.object,
  labelFormatter: PropTypes.func,
  label: PropTypes.any,
  payload: PropTypes.arrayOf(PropTypes.shape({
    name: PropTypes.any,
    value: PropTypes.oneOfType([PropTypes.number, PropTypes.string, PropTypes.array]),
    unit: PropTypes.any,
  })),
  itemSorter: PropTypes.func,
  active: PropTypes.bool,
};

CustomTooltip.defaultProps = {
  separator: ' : ',
  contentStyle: {},
  itemStyle: {},
  labelStyle: {},
  active: false,
};

export default CustomTooltip;
