const { parallel, series, watch } = require('gulp');
const electron = require('./electron');
const hotreload = require('./hotreload');
const assets = require('./assets');
const scripts = require('./scripts');

function watchMainScripts() {
  return watch(['app/main/**/*.js'], series(scripts.developBuild, electron.stop, electron.start));
}

function watchRendererScripts() {
  //return watch(['app/renderer/**/*.js', 'app/background/**/*.js'], series(scripts.developBuild, hotreload.reload));
  return watch(['app/renderer/**/*.js', 'app/background/**/*.js'], series(scripts.developBuild, electron.stop, electron.start)); // hotreload wasn't working so just restart electron
}


function watchHtml() {
  return watch(
    ['app/renderer/index.html', 'app/background/index.html'],
    //series(assets.copyHtml, hotreload.inject, hotreload.reload),
    series(assets.copyHtml, electron.stop, electron.start), // hotreload wasn't working so just restart electron
  );
}

watchMainScripts.displayName = 'watch-main-scripts';
watchRendererScripts.displayName = 'watch-renderer-scripts';
watchHtml.displayName = 'watch-html';

exports.start = series(
  assets.copyHtml,
  scripts.developBuild,
  hotreload.start,
  electron.start,
  parallel(watchMainScripts, watchRendererScripts, watchHtml),
);
