
import { ipcRenderer } from 'electron';

// Send logs as messages to the main thread to show on the console
function log(value) {
  ipcRenderer.send('to-main', `${process.pid} : ${value}`);
}
// let the main thread know this thread is ready to process something
function ready() {
  ipcRenderer.send('ready')
}

// do some work that will tie up the processor for a while
function work() {
  // see https://gist.github.com/tkrueger/3500612 for generating load
  const start = new Date().getTime()
  let result = 0
  console.log(result);
  let finished = false
  while(!finished) {
    result += Math.random() * Math.random()
    finished = new Date().getTime() > start + 10000
  }
}

// if message is received, pass it back to the renderer via the main thread
ipcRenderer.on('message', (event, arg) => {
  log(`received ${arg}`)
  ipcRenderer.send('for-renderer', `${process.pid} : reply to ${arg}`)
  ready()
});
ipcRenderer.on('task', (event, arg) => {
  log(`starting ${arg}`)
  work()
  log(`finished ${arg}`)
  ready()
});

ipcRenderer.on('for-background', (event, arg) => {
  log(`background ${event}`);
  log(`background ${arg}`);
});

ready();