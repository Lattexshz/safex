#[cfg(feature="xlib")]
pub mod xlib;

pub(crate) mod util {
    use std::ffi::{c_char, CString};

    pub fn str_to_c_char(str: &str) -> *const c_char {
        let cstring = CString::new(str).unwrap();
        cstring.as_ptr()
    }
}