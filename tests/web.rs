//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

mod utils;

use std::env::current_dir;
use std::path::PathBuf;

use wasm_bindgen_test::*;

const JPEG_420_EFIX: &[u8; 768608] = include_bytes!("../assets/jpeg420exif.jpg");

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let file = utils::read_file(JPEG_420_EFIX, "image/jpeg");

    assert_eq!(1 + 1, 2);
}
