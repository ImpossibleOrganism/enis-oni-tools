import * as wasm from "wasm-oni-tools";

// This exports my wasm to be available in my html.
window.wasm = wasm;

wasm.greet();
