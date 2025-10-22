use js_sys::Function;
use nesk::{AVG_FPS, HEIGHT, NES, WIDTH};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONTROLLER1_BUTTONS: [AtomicBool; 8] = [
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
    ];
    pub static ref CONTROLLER2_BUTTONS: [AtomicBool; 8] = [
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
        AtomicBool::new(false),
    ];
}

#[wasm_bindgen]
pub fn press_button(controller: i32, index: usize) {
    match controller {
        0 => CONTROLLER1_BUTTONS[index].store(true, std::sync::atomic::Ordering::SeqCst),
        1 => CONTROLLER2_BUTTONS[index].store(true, std::sync::atomic::Ordering::SeqCst),
        _ => {}
    }
}

#[wasm_bindgen]
pub fn release_button(controller: i32, index: usize) {
    match controller {
        0 => CONTROLLER1_BUTTONS[index].store(false, std::sync::atomic::Ordering::SeqCst),
        1 => CONTROLLER2_BUTTONS[index].store(false, std::sync::atomic::Ordering::SeqCst),
        _ => {}
    }
}

#[wasm_bindgen]
pub fn stop(interval_id: i32) {
    window().unwrap().clear_interval_with_handle(interval_id);
}

#[wasm_bindgen]
pub fn run(rom: Box<[u8]>, render: Function, audio: Function) -> Result<i32, String> {
    let mut nes = Rc::new(RefCell::new(NES::from_ines(&rom)?));

    let closure = Closure::wrap(Box::new(move || {
        let mut nes = nes.borrow_mut();

        let controller_1 = CONTROLLER1_BUTTONS.iter().map(|button| button.load(std::sync::atomic::Ordering::SeqCst)).collect::<Vec<_>>().try_into().unwrap();
        let controller_2 = CONTROLLER2_BUTTONS.iter().map(|button| button.load(std::sync::atomic::Ordering::SeqCst)).collect::<Vec<_>>().try_into().unwrap();

        nes.load_buttons(controller_1, controller_2);

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

        let audio_samples = nes.get_speaker_output();
        let audio_array = js_sys::Float64Array::new_with_length(audio_samples.len() as u32);
        audio_array.copy_from(&audio_samples);
        audio.clone().call1(&JsValue::NULL, &audio_array).unwrap();
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