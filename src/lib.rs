use js_sys::Function;
use nesk::{AVG_FPS, HEIGHT, NES, WIDTH};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

#[wasm_bindgen]
pub fn run(rom: Box<[u8]>, render: Function) -> Result<i32, String> {
    let mut nes = Rc::new(RefCell::new(NES::from_ines(&rom)?));

    let closure = Closure::wrap(Box::new(move || {
        let mut nes = nes.borrow_mut();
        let screen_output = loop {
            nes.tick();
            if let Some(screen_output) = nes.get_screen_output() {
                break screen_output;
            }
        };

        let mut pixels = Vec::with_capacity(WIDTH * HEIGHT * 4);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                pixels.extend_from_slice(&screen_output[y][x]);
                pixels.push(255);
            }
        }

        let js_pixels = js_sys::Uint8Array::new_with_length(pixels.len() as u32);
        js_pixels.copy_from(&pixels);
        render.clone().call1(&JsValue::NULL, &js_pixels).unwrap();
    }) as Box<dyn FnMut()>);

    let window = window().unwrap();
    let interval_id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            (1000.0 / (AVG_FPS)) as i32,
        )
        .unwrap();
    closure.forget();
    Ok(interval_id)
}