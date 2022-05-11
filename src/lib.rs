use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use base64::{encode,decode};



#[wasm_bindgen]
pub fn grayScale(encoded_file: &str)->String {
    log(&"Grayscale method called".into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded in Rust".into());
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());

    img = img.grayscale();
    log(&"grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New Image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}",encoded_img);

    data_url
}