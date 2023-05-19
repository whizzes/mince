use std::io::Cursor;

use anyhow::Result;
use image::io::Reader as ImageReader;
use image::DynamicImage;

pub struct Compress {
    image: DynamicImage,
}

impl Compress {
    pub fn new(bytes: &[u8]) -> Result<Self> {
        let cursor = Cursor::new(bytes);
        let image = ImageReader::new(cursor).with_guessed_format()?.decode()?;

        Ok(Self { image })
    }
}
