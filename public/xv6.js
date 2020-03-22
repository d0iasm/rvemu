// The `buffer`, that will be observed for mutations, stores the output from Rust.
const buffer = document.getElementById("buffer");
// The `buffer8`, that will be observed for mutations, stores the 1 byte from
// Rust.
const buffer8 = document.getElementById("buffer8");
// Options for the observer (which mutations to observe)
const config = { childList: true, subtree: true };

const termContainer = document.getElementById("terminal");
const term  = new Terminal({cursorBlink: true});

const fitAddon = new FitAddon.FitAddon();
const deleteLine = "\x1b[2K\r";

let inputBuffer = "";

// Callback function to execute when mutations are observed.
const callback = function(mutationsList, observer) {
  for(let mutation of mutationsList) {
    if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
      term.write(deleteLine);
      const firstChild = mutation.addedNodes[0];
      const texts = firstChild.innerText.split("\n");
      for (let i=0; i<texts.length; i++) {
        term.writeln(texts[i]);
      } buffer.removeChild(firstChild);
      term.write("% ");
    }
  }
};

let buffer_count = 0;

// Callback function to execute when mutations are observed.
const callback8 = function(mutationsList, observer) {
  for(let mutation of mutationsList) {
    if (buffer_count == 0 && mutation.addedNodes.length > 0) {
      term.write(deleteLine);
    }

    console.log("callback8", mutation);

    if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
      buffer_count++;
      const firstChild = mutation.addedNodes[0];
      let c = firstChild.innerText;
      buffer8.removeChild(firstChild);
      if (c != "\n") {
        term.write(c);
      } else {
        term.writeln("");
      }
    }

    if (mutation.type === 'childList' && mutation.removedNodes.length > 0) {
      buffer_count--;
    }

    if (buffer_count == 0 && mutation.removedNodes.length > 0) {
      term.write("% ");
    }
  }
};

// Create an observer instance linked to the callback function.
const observer = new MutationObserver(callback);
const observer8 = new MutationObserver(callback8);

function initTerminal() {
  term.loadAddon(fitAddon);
  term.open(termContainer);
  fitAddon.fit();

  if (term._initialized) {
      return;
  }

  term._initialized = true;

  term.writeln("Welcome to RISC-V emulator rvemu!");
  term.writeln("This is a work-in-progress project. You can see the progress at https://github.com/d0iasm/rvemu");
  term.writeln("Bug reports and feature requests are always welcome: https://github.com/d0iasm/rvemu/issues");
  term.writeln("");
  term.writeln("Loading operating system ...");

  // Start observing the target node for configured mutations
  observer8.observe(buffer8, config);
  observer.observe(buffer, config);

  let cursor = 0;
  term.onKey(e => {
    const printable = !e.domEvent.altKey && !e.domEvent.altGraphKey && !e.domEvent.ctrlKey && !e.domEvent.metaKey;

    console.log("get key", e.key);
    if (e.domEvent.code == 'Backspace') {
      inputBuffer = inputBuffer.substring(0, str.length - 1);
    } else if (printable) {
      inputBuffer += e.key;
    }
  });
}

if (window.Worker) {
  const emuWorker = new Worker('worker.js', {type: 'module'});
  emuWorker.onmessage = e => {
    // Read request from the emulator in Rust.
    if (e.data.readRequest) {
      const length = inputBuffer.length;
      if (length <= 0) {
        emuWorker.postMessage({
          id: 1,
          readRequest: false,
          content: 0,
        });
        return;
      }

      const c = inputBuffer[0];
      inputBuffer = inputBuffer.substring(1);
      console.log('read request!!!!!!!!!!', e, inputBuffer, c);
      emuWorker.postMessage({
        id: 1,
        readRequest: false,
        content: c,
      });
      return;
    }

    // Write a byte from the emulator in Rust.
    if (e.data.content != "\n") {
      term.write(e.data.content);
    } else {
      term.writeln("");
    }
  }
} else {
  console.error('This browser does not support worker threads yet.');
}

initTerminal();
