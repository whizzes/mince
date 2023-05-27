//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

mod utils;

use std::assert_eq;

use wasm_bindgen_test::*;

use mince::image::Mince;

use utils::{read_file, JPEG_420_EFIX};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn reads_file_metadata() {
    let file = read_file(JPEG_420_EFIX, "image/jpeg");
    let mince = Mince::from_file(file).await.unwrap();
    let meta = mince.meta();

    assert_eq!(meta.width, 2048);
    assert_eq!(meta.height, 1536);
    assert_eq!(meta.format, mince::image::Format::Jpeg);
}

#[wasm_bindgen_test]
async fn encodes_jpeg() {
    let file = read_file(JPEG_420_EFIX, "image/jpeg");
    let mince = Mince::from_file(file).await.unwrap();
    let output_file = mince.to_file().expect("Failed to encode image");

    assert_eq!(output_file.name(), "mince_image.jpeg");
    assert_eq!(output_file.type_(), "image/jpeg");
    assert_eq!(output_file.size(), 2068725.);
}
