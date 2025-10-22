import * as wasm from "./pkg/nesk_web.js";

document.addEventListener("DOMContentLoaded", async () => {
    await wasm.default();
    alert("Rust output: " + wasm.run())
});