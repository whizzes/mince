use js_sys::{Array, Uint8Array};
use wasm_bindgen::JsValue;
use web_sys::{Blob, BlobPropertyBag, File as JsFile, FilePropertyBag};

pub fn read_file(buffer: &[u8], mime: &str) -> JsFile {
    // Prepare a Blob from the file bytes
    let mut blob_options = BlobPropertyBag::new();
    blob_options.type_(mime);
    let uint8_array = Uint8Array::from(buffer);
    let blob =
        Blob::new_with_u8_array_sequence_and_options(&JsValue::from(uint8_array), &blob_options)
            .expect("Failed to create Blob from file bytes");

    let sequence = Array::new();
    sequence.set(0, JsValue::from(blob));

    let mut file_options = FilePropertyBag::new();
    file_options.type_(mime);

    JsFile::new_with_blob_sequence_and_options(&sequence, "test", &file_options)
        .expect("Failed to create File from Blob")
}
