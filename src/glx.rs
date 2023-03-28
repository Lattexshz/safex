use std::ffi::c_int;
use crate::xlib::{Display, AsRaw, Window};
use x11::glx::*;

pub type GLXAttribute = c_int;

macro_rules! export {
    ($cons:ident,$type_:ident) => {
        pub const $cons: $type_ = x11::xlib::$cons as $type_;
    };
}

export!(GLX_SLOW_CONFIG,GLXAttribute);
export!(GLX_NON_CONFORMANT_CONFIG,GLXAttribute);

pub struct GLXContext {

}

pub struct GLXWindow {
    inner: Window
}

impl GLXWindow {
    pub fn new() {
        
    }
}