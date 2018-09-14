import React, { Component } from 'react';
import { bindActionCreators } from 'redux';
import { connect } from 'react-redux';

import FileTest from '../components/FileTest';
import * as FileActions from '../actions/file';

class FileTestPage extends Component {
  render() {
    return <FileTest {...this.props} />;
  }
}

FileTestPage.propTypes = {};

const mapStateToProps = state => ({
  content: state.file.content
});

const mapDispatchToProps = dispatch =>
  bindActionCreators(FileActions, dispatch);

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(FileTestPage);
