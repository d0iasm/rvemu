let inputBuffer = "";

onmessage = e => {
  if (e.data.content == 0) {
    return;
  }
  console.log('come back!!! in check_input', e);
  inputBuffer += e.data.content;
}

export function check_input() {
  console.log("check input");
  postMessage({
    id: 3,
    readRequest: true,
    content: '',
  });
}

export function get_input() {
  console.log("check input", inputBuffer);
  if (inputBuffer.length <= 0) {
    return 0;
  }
  const c = inputBuffer[0];
  inputBuffer = inputBuffer.substring(1);
  return c.charCodeAt(0);
}

export function write_to_buffer(b) {
  postMessage({
    id: 3,
    readRequest: false,
    content: String.fromCharCode(b),
  });
}
