mod compress;
mod file;
mod utils;

// pub use file::File;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub async fn read_file_into_bytes(f: web_sys::File) -> js_sys::Uint8Array {
    let bytes = file::into_bytes(f).await;
    js_sys::Uint8Array::from(bytes.as_slice())
}
