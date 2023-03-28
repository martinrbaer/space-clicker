use std::sync::Mutex;
use wasm_bindgen::prelude::*;

mod constants;
mod physics;

use crate::physics::Bodies;

#[macro_use]
extern crate lazy_static;
// Global variable for the bodies
lazy_static! {
    static ref BODIES: Mutex<Bodies> = {
        let data = Bodies::new_empty();
        Mutex::new(data)
    };
}

// Imported JS function from draw_body.js
#[wasm_bindgen]
extern "C" {
    pub fn draw_body(x: f64, y: f64, color: &str, size: i32);
}

// Exported Rust functions to be used by initialiser.js
#[wasm_bindgen]
pub fn create_bodies(w: f64, h: f64, num: usize) {
    BODIES.lock().unwrap().canvas_width = w;
    BODIES.lock().unwrap().canvas_height = h;
    BODIES.lock().unwrap().create(num);
}
#[wasm_bindgen]
pub fn render_bodies() {
    BODIES.lock().unwrap().draw();
    BODIES.lock().unwrap().update();
}
