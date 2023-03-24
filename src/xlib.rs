use std::ffi::{c_int, CStr};
use std::ptr::null_mut;
use x11::xlib::*;
use crate::util::str_to_c_char;

pub struct Display {
    display: *mut x11::xlib::Display
}

impl Display {
    pub fn open(display_name: Option<&str>) -> Self {
        let display = match display_name {
            None => unsafe { XOpenDisplay(null_mut()) }
            Some(display_name) => {
                unsafe {
                    XOpenDisplay(str_to_c_char(display_name))
                }
            }
        };

        Self {
            display
        }
    }

    pub fn string(&self) -> String {
        unsafe {
            let string = XDisplayString(self.display);
            let cstr = CStr::from_ptr(string);
            String::from(cstr.to_str().unwrap())
        }
    }

    pub fn protocol_version(&self) -> i32 {
        unsafe { XProtocolVersion(self.display) as i32 }
    }

    pub fn revision_version(&self) -> i32 {
        unsafe { XProtocolRevision(self.display) as i32 }
    }

    pub fn server_vendor(&self) -> String {
        unsafe {
            let string = XServerVendor(self.display);
            let cstr = CStr::from_ptr(string);
            String::from(cstr.to_str().unwrap())
        }
    }

    pub fn vendor_release(&self) -> i32 {
        unsafe {
            XVendorRelease(self.display) as i32
        }
    }

    pub fn from_raw(display: *mut x11::xlib::Display) -> Self {
        Self {
            display
        }
    }
}

pub struct Screen {
    screen: c_int
}

impl Screen {
    pub fn default(display: &Display) -> Self {
        let screen = unsafe { XDefaultScreen(display.display) };
        Self {
            screen
        }
    }

    pub fn from_raw(screen: c_int) -> Self {
        Self {
            screen
        }
    }
}