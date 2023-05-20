use std::io::Cursor;

use anyhow::Result;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::File;

use crate::log;

#[wasm_bindgen]
pub struct Mince {
    inner: Vec<u8>,
    format: ImageFormat,
}

#[wasm_bindgen]
impl Mince {
    pub async fn from_file(file: File) -> Mince {
        let array_buffer = JsFuture::from(file.array_buffer()).await.unwrap();
        let uint8_array = Uint8Array::new(&array_buffer);
        let inner = uint8_array.to_vec();
        let cursor = Cursor::new(inner.clone());
        let reader = ImageReader::new(cursor).with_guessed_format().unwrap();
        let format = reader.format().unwrap();

        Self { inner, format }
    }

    pub fn format(&self) -> String {
        format!("{:?}", self.format)
    }
}
