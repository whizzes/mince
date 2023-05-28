use std::io::Cursor;

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat, ImageOutputFormat};
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, FilePropertyBag};

use crate::console;
use crate::error::{MinceError, Result};

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

impl Format {
    pub fn mime(&self) -> &'static str {
        match self {
            Format::Jpeg => "image/jpeg",
            Format::Png => "image/png",
            Format::Gif => "image/gif",
            _ => "image/unsupported",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Format::Jpeg => "jpeg",
            Format::Png => "png",
            Format::Gif => "gif",
            _ => "unsupported",
        }
    }
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

impl Into<ImageOutputFormat> for Format {
    fn into(self) -> ImageOutputFormat {
        match self {
            Format::Jpeg => ImageOutputFormat::Jpeg(100),
            Format::Png => ImageOutputFormat::Png,
            Format::Gif => ImageOutputFormat::Gif,
            _ => unreachable!(),
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
    pub size: u64,
}

#[wasm_bindgen]
impl Metadata {
    pub fn new(width: u32, height: u32, format: Format, size: u64) -> Self {
        Self {
            width,
            height,
            format,
            size,
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
    fn new(dynamic_image: DynamicImage, meta: Metadata) -> Mince {
        Self {
            inner: Box::new(dynamic_image),
            meta,
        }
    }

    /// Reads a browser file into a `Mince` instance
    #[wasm_bindgen(js_name = fromFile)]
    pub async fn from_file(file: File) -> Result<Mince> {
        let bytes = Self::file_bytes(file).await?;
        let size = bytes.len() as u64;
        let cursor = Cursor::new(bytes);
        let reader = ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|err| {
                console::error(&format!("Error reading file: {:?}", err));
                MinceError::FileRead
            })?;
        let format = reader.format().ok_or(MinceError::DetectImageFormat)?;
        let image = reader.decode().map_err(|err| {
            console::error(&format!("Error decoding file: {:?}", err));
            MinceError::DecodeImage
        })?;
        let meta = Metadata::new(image.width(), image.height(), format.into(), size);

        Ok(Self {
            inner: Box::new(image),
            meta,
        })
    }

    pub fn meta(&self) -> Metadata {
        self.meta
    }

    /// Resizes the image and returns a new instance of `Mince` containing it
    pub fn resize(&self, width: u32, height: u32) -> Self {
        use image::imageops;

        let buf = imageops::resize(self.inner.as_ref(), width, height, FilterType::Lanczos3);
        let dynamic_image = DynamicImage::ImageRgba8(buf);

        Mince::new(dynamic_image, self.meta())
    }

    #[wasm_bindgen(js_name = toFile)]
    pub fn to_file(&self) -> Result<File> {
        let format = self.meta.format;
        let mut bytes: Vec<u8> = Vec::with_capacity(self.meta.size as usize);
        let mut buf = Cursor::new(&mut bytes);

        self.inner.write_to(&mut buf, format).map_err(|err| {
            console::error(&format!("Error encoding image: {:?}", err));
            MinceError::EncodeImage
        })?;

        Ok(Self::write_file(&bytes, &self.filename(), format.mime()))
    }

    /// Reads a browser file and returns a `Vec<u8>` containing the bytes
    async fn file_bytes(file: File) -> Result<Vec<u8>> {
        let array_buffer = JsFuture::from(file.array_buffer()).await.map_err(|err| {
            console::error(&format!("Error reading file: {:?}", err));
            MinceError::FileRead
        })?;
        let uint8_array = Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();

        Ok(bytes)
    }

    fn write_file(bytes: &[u8], filename: &str, mime: &str) -> File {
        let uint8_array = Uint8Array::from(bytes);
        let sequence = Array::new();
        sequence.push(&uint8_array.buffer());

        let mut file_options = FilePropertyBag::new();
        file_options.type_(mime);

        File::new_with_blob_sequence_and_options(&sequence, filename, &file_options)
            .expect("Failed to create File from Blob")
    }

    fn filename(&self) -> String {
        format!("mince_image.{}", self.meta.format.extension())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_mime() {
        assert_eq!(Format::Jpeg.mime(), "image/jpeg");
        assert_eq!(Format::Png.mime(), "image/png");
        assert_eq!(Format::Gif.mime(), "image/gif");
        assert_eq!(Format::Unsupported.mime(), "image/unsupported");
    }

    #[test]
    fn test_format_extension() {
        assert_eq!(Format::Jpeg.extension(), "jpeg");
        assert_eq!(Format::Png.extension(), "png");
        assert_eq!(Format::Gif.extension(), "gif");
        assert_eq!(Format::Unsupported.extension(), "unsupported");
    }
}
