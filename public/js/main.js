import * as wasm from "../pkg";

document.addEventListener("DOMContentLoaded", async () => {
    await wasm.default();
    alert("Rust output: " + wasm.run())
});