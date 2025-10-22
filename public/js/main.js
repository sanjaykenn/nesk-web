import * as wasm from "../pkg/nesk_web.js";

const WIDTH = 256;
const HEIGHT = 240;
let nes_interval_id = undefined;
let player1_keys = {
    ' ': 0,        // A
    'SHIFT': 1,        // B
    'CONTROL': 2,    // Select
    'ENTER': 3,    // Start
    'W': 4,  // Up
    'S': 5, // Down
    'A': 6, // Left
    'D': 7 // Right
};

let player2_keys = {
}

document.addEventListener("DOMContentLoaded", async () => {
    await wasm.default();

    document.addEventListener('keydown', (event) => {
        if (player1_keys.hasOwnProperty(event.key.toUpperCase())) {
            wasm.press_button(0, player1_keys[event.key.toUpperCase()]);
        }

        if (player2_keys.hasOwnProperty(event.key.toUpperCase())) {
            wasm.press_button(1, player2_keys[event.key.toUpperCase()]);
        }
    });

    document.addEventListener('keyup', (event) => {
        if (player1_keys.hasOwnProperty(event.key.toUpperCase())) {
            wasm.release_button(0, player1_keys[event.key.toUpperCase()]);
        }

        if (player2_keys.hasOwnProperty(event.key.toUpperCase())) {
            wasm.release_button(1, player2_keys[event.key.toUpperCase()]);
        }
    });

    document.querySelector("#file-input").addEventListener("change", function (e) {
        const reader = new FileReader();
        reader.onload = function (e) {
            if (nes_interval_id !== undefined) {
                wasm.stop(nes_interval_id);
                nes_interval_id = undefined;
            }

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

            nes_interval_id = wasm.run(romData, render);
        }

        reader.readAsArrayBuffer(e.target.files[0]);
    })

    document.querySelectorAll(".control-settings").forEach(((element, index) => {
        let controls = index === 0 ? player1_keys : player2_keys;
        element.querySelectorAll(".control-setting").forEach((element, button) => {
            let label = Object.keys(controls).find(key => controls[key] === button);
            if (label === ' ') {
                label = 'SPACE'
            }

            element.querySelector("input[type='button']").value = label ?? ""

            element.querySelector("input[type='button']").addEventListener("click", (event) => {
                event.target.classList.toggle("active")
            })
        })
    }))

    document.addEventListener('keydown', (event) => {
        document.querySelectorAll(".control-settings").forEach(((element, index) => {
            let controls = index === 0 ? player1_keys : player2_keys;
            element.querySelectorAll(".control-setting").forEach((e, button) => {
                let btn = e.querySelector("input[type='button']");
                if (btn.classList.contains("active")) {
                    let key = Object.keys(controls).find(key => controls[key] === button);
                    if (key) {
                        delete controls[key];
                    }

                    let newKey = event.key.toUpperCase();
                    controls[newKey] = button;
                    btn.value = newKey === " " ? "SPACE" : newKey;
                    btn.classList.remove("active");
                }
            })
        }))
    });
});