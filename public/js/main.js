import * as wasm from "../pkg/nesk_web.js";

document.addEventListener("DOMContentLoaded", async () => {
    await wasm.default();
    console.log("Rust output: " + wasm.run())
});