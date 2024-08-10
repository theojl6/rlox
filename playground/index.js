import * as wasm from "rlox";

const inputVal = document.getElementById("code").value;
const output = wasm.run_lox(inputVal);
document.getElementById("output").innerHTML = output;
