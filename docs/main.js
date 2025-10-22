import * as wasm from "../pkg/nesk_ui.js";

document.addEventListener("DOMContentLoaded", async () => {
    await wasm.default();
    alert("Rust output: " + wasm.run())
});