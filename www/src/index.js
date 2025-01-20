import * as oni_tools from "oni-tools";

// This exports my wasm to be available in my html.
window.wasm = oni_tools;

oni_tools.greet();
