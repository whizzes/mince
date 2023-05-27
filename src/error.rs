use thiserror::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub type Result<T> = std::result::Result<T, MinceError>;

#[wasm_bindgen]
#[derive(Debug, Error)]
pub enum MinceError {
    #[error("IO error")]
    Generic,
    #[error("Failed to read file into bytes")]
    FileRead,
    #[error("Failed to detect image format")]
    DetectImageFormat,
    #[error("Failed to decode image")]
    DecodeImage,
    #[error("Failed to encode image")]
    EncodeImage,
}

impl Into<JsValue> for MinceError {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}
