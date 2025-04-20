use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn double(x: i32) -> i32 {
    x * 2
}