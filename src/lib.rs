use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::decode;
use image::load_from_memory;



#[wasm_bindgen]
pub fn grayScale(encoded_file: &str) {
    log(&"Grayscale method called".into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded in Rust".into());
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());

    img = img.grayscale();
    log(&"grayscale effect applied".into());
}