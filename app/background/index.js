import { ipcRenderer } from 'electron';

// Send logs as messages to the main thread to show on the console
// function log(value) {
//   ipcRenderer.send('to-main', `${process.pid} : ${value}`);
// }

// let the main thread know this thread is ready to process something
function ready() {
  ipcRenderer.send('ready');
}

// do some work that will tie up the processor for a while
function work() {
  // see https://gist.github.com/tkrueger/3500612 for generating load
  const start = new Date().getTime();
  let result = 0;
  let finished = false;
  while (!finished) {
    result += Math.random() * Math.random();
    finished = new Date().getTime() > start + 10000;
  }
  return result;
}

ipcRenderer.on('to-background', (event, arg) => {
  console.log(`background ${arg}`);
  ipcRenderer.send(
    'for-renderer',
    `${process.pid} : processing reply to ${arg}`
  );
  const result = work();
  ipcRenderer.send('for-renderer', `${process.pid} : ${result}`);
});

ready();
