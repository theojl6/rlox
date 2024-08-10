import * as wasm from "rlox";

const button = document.getElementById("submit");
const input = document.getElementById("code");

const runCode = function() {
  document.getElementById("output").innerHTML = wasm.run_lox(input.value);
}

button.onclick = runCode;
input.onkeydown = function(ev) {
  if (ev.key == 'Enter') {
    runCode();
  }
}
