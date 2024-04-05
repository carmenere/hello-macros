use std::ffi::CString;
use hello_macros::io;

pub(crate) fn create_file(path: CString) {
    io::create_file(path)
}