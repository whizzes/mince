use std::io::Cursor;

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::File;

/// Supported image formats for `Mince`
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Format {
    Jpeg,
    Png,
    Gif,
    #[default]
    Unsupported,
}

impl From<ImageFormat> for Format {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::Jpeg => Format::Jpeg,
            ImageFormat::Png => Format::Png,
            ImageFormat::Gif => Format::Gif,
            _ => Format::Unsupported,
        }
    }
}

/// Metadata for a `Mince` instance, which provides relevant details on the
/// contained image instance
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Metadata {
    pub width: u32,
    pub height: u32,
    pub format: Format,
}

#[wasm_bindgen]
impl Metadata {
    pub fn new(width: u32, height: u32, format: Format) -> Self {
        Self {
            width,
            height,
            format,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Mince {
    inner: Box<DynamicImage>,
    meta: Metadata,
}

#[wasm_bindgen]
impl Mince {
    /// Creates a new `Mince` instance from a Image RS `DynamicImage`
    fn new(dynamic_image: DynamicImage) -> Mince {
        let meta = Metadata::default();

        Self {
            inner: Box::new(dynamic_image),
            meta,
        }
    }

    /// Reads a browser file into a `Mince` instance
    pub async fn from_file(file: File) -> Mince {
        let array_buffer = JsFuture::from(file.array_buffer()).await.unwrap();
        let uint8_array = Uint8Array::new(&array_buffer);
        let inner = uint8_array.to_vec();
        let cursor = Cursor::new(inner);
        let reader = ImageReader::new(cursor).with_guessed_format().unwrap();
        let format = reader.format().unwrap();
        let image = reader.decode().unwrap();
        let meta = Metadata::new(image.width(), image.height(), format.into());

        Self {
            inner: Box::new(image),
            meta,
        }
    }

    pub fn meta(&self) -> Metadata {
        self.meta
    }

    /// Resizes the image and returns a new instance of `Mince` containing it
    pub fn resize(&self, width: u32, height: u32) -> Self {
        use image::imageops;

        let buf = imageops::resize(self.inner.as_ref(), width, height, FilterType::Lanczos3);
        let dynamic_image = DynamicImage::ImageRgba8(buf);

        Mince::new(dynamic_image)
    }
}
