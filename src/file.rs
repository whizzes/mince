use js_sys::Uint8Array;
use wasm_bindgen_futures::JsFuture;
use web_sys::File;

pub async fn into_bytes(file: File) -> Vec<u8> {
    let array_buffer = JsFuture::from(file.array_buffer()).await.unwrap();
    let uint8_array = Uint8Array::new(&array_buffer);
    let bytes = uint8_array.to_vec();

    bytes
}

pub async fn compress_jpeg(file: File) {
    let bytes = into_bytes(file).await;
}
