use std::cell::RefCell;
use std::rc::Rc;

use web_sys::FileReaderSync;

pub fn file_reader(file: &Rc<RefCell<Vec<u8>>>) {
    let fr = FileReaderSync::new().expect("Failed to create file reader");
}
