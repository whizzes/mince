use std::sync::Arc;

use js_sys::Uint8Array;
use std::io::{Seek, SeekFrom};
use wasm_bindgen::prelude::Closure;
use web_sys::{Event, FileReader};

type Callback = Box<dyn Fn(Vec<u8>) -> ()>;

pub struct File {
    inner: web_sys::File,
    offset: u64,
}

impl File {
    /// Creates a new `File` instance using the provided bytes
    pub fn new(file_name: &str, bytes: Vec<u8>) -> Self {
        let uint8_array = Uint8Array::new_with_length(bytes.len() as u32);

        uint8_array.copy_from(bytes.as_slice());

        let inner = web_sys::File::new_with_u8_array_sequence(&uint8_array, file_name)
            .expect("Failed to create file from bytes sequence");

        Self { inner, offset: 0 }
    }

    /// Retrieves the size of the `Blob` object contained by this `File`
    /// instance
    pub fn size(&self) -> u64 {
        let size = self.inner.size();

        size as u64
    }

    #[inline]
    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub async fn read(&mut self, cb: Callback) -> Result<usize, Box<dyn std::error::Error>> {
        let file_reader = FileReader::new().unwrap();
        let file_reader = Arc::new(file_reader);
        let offset = self.offset;
        let blob = self
            .inner
            .slice_with_f64_and_f64(offset as f64, self.size() as f64)
            .unwrap();
        let file_reader_clone = Arc::clone(&file_reader);
        let on_loadend = Closure::wrap(Box::new(move |_: Event| {
            // The FileReader interface's readAsArrayBuffer() method is used
            // to start reading the contents of a specified Blob or File. When
            // the read operation is finished, the readyState becomes DONE, and
            // the loadend is triggered. At that time, the result attribute
            // contains an ArrayBuffer representing the file's data.
            //
            // Source: https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer
            let result = file_reader_clone.result().unwrap();
            let unint8_array = Uint8Array::new(&result);
            let read_bytes = unint8_array.byte_length();
            let _ = usize::try_from(read_bytes).expect("read too many bytes at once");

            let mut temp: Vec<u8> = Vec::new();
            unint8_array.copy_to(&mut temp);
            cb(temp);
        }) as Box<dyn FnMut(_)>)
        .into_js_value();
        let func = js_sys::Function::from(on_loadend);
        file_reader.set_onloadend(Some(&func));
        file_reader.read_as_array_buffer(&blob).unwrap();

        Ok(1)
    }
}

impl From<web_sys::File> for File {
    fn from(value: web_sys::File) -> Self {
        Self {
            inner: value,
            offset: 0,
        }
    }
}

impl Seek for File {
    fn seek(&mut self, style: SeekFrom) -> Result<u64, std::io::Error> {
        // Seek impl copied from std::io::Cursor
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.offset = n;
                return Ok(n);
            }
            SeekFrom::End(n) => (self.size(), n),
            SeekFrom::Current(n) => (self.offset, n),
        };
        match u64::checked_add_signed(base_pos, offset) {
            Some(n) => {
                self.offset = n;
                Ok(self.offset)
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    #![cfg(target_arch = "wasm32")]

    use std::io::{Read, Seek, SeekFrom};

    use super::File;

    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn builds_file_from_bytes() {
        let bytes: Vec<u8> = vec![100, 101, 102, 103, 104, 105];
        let file = File::new("untitled", bytes.clone());

        assert_eq!(file.size(), (bytes.len() * 3) as u64);
    }

    #[wasm_bindgen_test]
    fn initial_offset_is_0() {
        let bytes: Vec<u8> = vec![100, 101, 102, 103, 104, 105];
        let file = File::new("untitled", bytes.clone());

        assert_eq!(file.offset(), 0);
    }

    #[wasm_bindgen_test]
    fn file_name_is_used_internally() {
        let bytes: Vec<u8> = vec![100, 101, 102, 103, 104, 105];
        let file = File::new("Lorem Ipsum", bytes.clone());

        assert_eq!(file.inner.name(), "Lorem Ipsum");
    }

    #[wasm_bindgen_test]
    async fn read_file_instance() {
        let bytes: Vec<u8> = vec![100, 101, 102, 103, 104, 105];
        let mut file = File::new("testing", bytes);
        file.seek(SeekFrom::Start(0))
            .expect("failed to seek to offset");

        // 1-byte buffer because we only want to read one byte
        let mut buf = [0];

        file.read(Box::new(|bytes| {
            panic!("{:?}", bytes);
        }))
        .await
        .expect("failed to read bytes");

        assert_eq!(0, 0);
    }
}
