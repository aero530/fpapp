const { src, dest } = require('gulp');

function copyHtml() {
  src('app/background/index.html').pipe(dest('build/background'));
  return src('app/renderer/index.html').pipe(dest('build/renderer'));
}

function copyIcons() {
  src('resources/icon.icns').pipe(dest('dist-assets'));
  src('resources/icon.png').pipe(dest('dist-assets'));
  src('resources/icons/*').pipe(dest('dist-assets/icons'));
  return src('resources/icon.ico').pipe(dest('dist-assets'));
}

copyHtml.displayName = 'copy-html';
copyIcons.displayName = 'copy-icons';

exports.copyHtml = copyHtml;
exports.copyIcons = copyIcons;
