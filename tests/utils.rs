#![cfg(target_arch = "wasm32")]

use js_sys::{Array, Uint8Array};
use wasm_bindgen_test::*;
use web_sys::{File as JsFile, FilePropertyBag};

pub const JPEG_420_EFIX: &[u8; 768608] = include_bytes!("../assets/jpeg420exif.jpg");

pub fn read_file(bytes: &[u8], mime: &str) -> JsFile {
    // Prepare a Blob from the file bytes
    let uint8_array = Uint8Array::from(bytes);
    let sequence = Array::new();
    sequence.push(&uint8_array.buffer());

    let mut file_options = FilePropertyBag::new();
    file_options.type_(mime);

    JsFile::new_with_blob_sequence_and_options(&sequence, "test", &file_options)
        .expect("Failed to create File from Blob")
}

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn makes_file_from_bytes() {
    let file = read_file(JPEG_420_EFIX, "image/jpeg");

    assert_eq!(file.size() as usize, 768608);
}
