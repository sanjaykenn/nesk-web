import * as wasm from "../pkg/nesk_web.js";

const WIDTH = 256;
const HEIGHT = 240;

document.addEventListener("DOMContentLoaded", async () => {
    await wasm.default();

    document.querySelector("#file-input").addEventListener("change", function (e) {
        const reader = new FileReader();
        reader.onload = function (e) {
            let romData = new Uint8Array(e.target.result);

            const canvas = document.querySelector("#nes-canvas");
            const ctx = canvas.getContext("2d");
            canvas.width = WIDTH;
            canvas.height = HEIGHT;

            function render(pixels) {
                const imageData = new ImageData(
                    new Uint8ClampedArray(pixels),
                    WIDTH,
                    HEIGHT
                );
                ctx.putImageData(imageData, 0, 0);
            }

            wasm.run(romData, render);
        }

        reader.readAsArrayBuffer(e.target.files[0]);
    })
});