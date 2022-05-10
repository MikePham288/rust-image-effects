use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::decode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[wasm_bindgen]
pub fn grayScale(encoded_file: &str) {
    log(&"Grayscale method called".into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"image decoded in Rust".into());
}