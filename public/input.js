export function check_input() {
  console.log("check input");
  postMessage({
    id: 3,
    readRequest: true,
    content: '',
  });

  onmessage = e => {
    console.log('come back!!! in check_input', e);
    if (e.data.content == 0) {
      return 0;
    }
    return e.data.content.charCodeAt(0);
  }
}

export function write_to_buffer(b) {
  postMessage({
    id: 3,
    readRequest: false,
    content: String.fromCharCode(b)
  });
}
