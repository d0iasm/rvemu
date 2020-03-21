//const buffer8 = document.getElementById("buffer8");

export function check_input() {
  console.log("check input");
  return 42;
}

export function write_to_buffer(b) {
  postMessage(String.fromCharCode(b));
  console.log(String.fromCharCode(b));
}
