use std::ffi::{c_int, c_uint, c_ulong, CStr};
use std::ptr::null_mut;
use x11::xlib::*;
use crate::util::str_to_c_char;

type _Window = c_ulong;

pub enum ControlFlow {
    Wait,
    Exit
}

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

pub struct Visual {
    visual: *mut x11::xlib::Visual
}

impl Visual {
    pub fn default(display: &Display,screen: &Screen) -> Self {
        let visual = unsafe { XDefaultVisual(display.display,screen.screen) };
        Self {
            visual
        }
    }
}

pub struct WindowAttributesBuilder {
    attributes: XSetWindowAttributes
}

impl WindowAttributesBuilder {
    pub fn new() -> Self {
        let attributes: XSetWindowAttributes = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        Self {
            attributes
        }
    }

    pub fn override_redirect(&mut self,b: bool) -> &mut self {
        self.attributes.override_redirect = b as i32;
        self
    }
}

pub enum WindowEvent {
    Expose,
}

pub struct Window {
    window: _Window
}

impl Window {
    pub fn create(display: &Display, parent: Option<Window>, x:i32, y:i32, width:u32, height:u32, border_width:u32, depth: i32, class:u32, visual:&Visual, valuemask:u64, mut attributes: WindowAttributesBuilder) -> Self {
        unsafe {
            let parent = match parent {
                None => 0,
                Some(p) => p
            };

            let window = XCreateWindow(display.display, parent, x as c_int, y as c_int, width as c_uint, height as c_uint, border_width as c_uint, depth as c_int, class as c_uint, visual.visual, valuemask as c_ulong,&mut attributes.attributes);

            Self {
                window
            }
        }
    }

    pub fn map(&self,display: &Display) {
        unsafe {
            XMapWindow(display.display,self.window);
        }
    }

    pub fn run<F>(func: F)
    where
        F: Fn(WindowEvent,&mut ControlFlow)
    {
        unsafe {
            let mut event = std::mem::MaybeUninit::uninit().assume_init();

            let mut control_flow = ControlFlow::Wait;
            loop {
                XNextEvent(display, &mut event);

                match event.type_ {
                    Expose => {
                        func(WindowEvent::Expose,&mut control_flow);
                    }
                    _ => {
                        break;
                    }
                }
            }


        }
    }
}