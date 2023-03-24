use crate::util::str_to_c_char;
use std::ffi::{c_char, c_int, c_uint, c_ulong, CStr};
use std::ptr::null_mut;
use x11::xlib::*;

type _Window = c_ulong;

pub type EventMask = c_ulong;

// Re-Exports
macro_rules! export {
    ($cons:ident,$type_:ident) => {
        pub const $cons:$type_ = x11::xlib::$cons as $type_;
    }
}

macro_rules! cast {
    ($val:ident,$type_:ty) => {
        $val as $type_
    }
}

// EventMasks
export!(NoEventMask,EventMask);
export!(KeyPressMask,EventMask);
export!(KeyReleaseMask,EventMask);
export!(ButtonPressMask,EventMask);
export!(ButtonReleaseMask,EventMask);
export!(EnterWindowMask,EventMask);
export!(LeaveWindowMask,EventMask);
export!(PointerMotionMask,EventMask);
export!(PointerMotionHintMask,EventMask);
export!(Button1MotionMask,EventMask);
export!(Button2MotionMask,EventMask);
export!(Button3MotionMask,EventMask);
export!(Button4MotionMask,EventMask);
export!(Button5MotionMask,EventMask);
export!(KeymapStateMask,EventMask);
export!(ExposureMask,EventMask);
export!(VisibilityChangeMask,EventMask);
export!(StructureNotifyMask,EventMask);
export!(ResizeRedirectMask,EventMask);
export!(SubstructureNotifyMask,EventMask);
export!(SubstructureRedirectMask,EventMask);
export!(FocusChangeMask,EventMask);
export!(PropertyChangeMask,EventMask);
export!(ColormapChangeMask,EventMask);
export!(OwnerGrabButtonMask,EventMask);

pub type WindowClass = c_uint;

export!(InputOutput,WindowClass);
export!(InputOnly,WindowClass);

pub type WindowAttribute = c_ulong;

export!(CWBackPixmap,WindowAttribute);
export!(CWBackPixel,WindowAttribute);
export!(CWBorderPixmap,WindowAttribute);
export!(CWBorderPixel,WindowAttribute);
export!(CWBitGravity,WindowAttribute);
export!(CWWinGravity,WindowAttribute);
export!(CWBackingStore,WindowAttribute);
export!(CWBackingPlanes,WindowAttribute);
export!(CWOverrideRedirect,WindowAttribute);
export!(CWSaveUnder,WindowAttribute);
export!(CWEventMask,WindowAttribute);
export!(CWDontPropagate,WindowAttribute);
export!(CWColormap,WindowAttribute);
export!(CWCursor,WindowAttribute);

pub enum ControlFlow {
    Wait,
    Exit,
}

pub struct ColorMap {
    cmap: c_ulong
}

impl ColorMap {
    pub fn default(display:&Display,screen:&Screen) -> Self {
        let cmap = unsafe { XDefaultColormap(display.display,screen.screen) };
        Self {
            cmap
        }
    }
}

pub struct Display {
    display: *mut x11::xlib::Display,
}

impl Display {
    pub fn open(display_name: Option<&str>) -> Self {
        let display = match display_name {
            None => unsafe { XOpenDisplay(null_mut()) },
            Some(display_name) => unsafe { XOpenDisplay(str_to_c_char(display_name)) },
        };

        Self { display }
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
        unsafe { XVendorRelease(self.display) as i32 }
    }

    pub fn from_raw(display: *mut x11::xlib::Display) -> Self {
        Self { display }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        if !self.display.is_null() {
            unsafe {
                XCloseDisplay(self.display);
            }
        }
    }
}

pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub depth:u32
}

pub struct Pixel {
    pixel:c_ulong
}

impl Pixel {
    pub fn black(display:&Display,screen:&Screen) -> Self {
        let pixel = unsafe { XBlackPixel(display.display,screen.screen) };

        Self {
            pixel
        }
    }
    pub fn white(display:&Display,screen:&Screen) -> Self {
        let pixel = unsafe { XWhitePixel(display.display,screen.screen) };

        Self {
            pixel
        }
    }
    pub fn from_rgb(r:u8,g:u8,b:u8) -> Self {
        Self {
            pixel: 0
        }
    }
}

pub struct Screen {
    screen: c_int,
}

impl Screen {
    pub fn default(display: &Display) -> Self {
        let screen = unsafe { XDefaultScreen(display.display) };
        Self { screen }
    }

    pub fn from_raw(screen: c_int) -> Self {
        Self { screen }
    }
}

pub struct Visual {
    visual: *mut x11::xlib::Visual,
}

impl Visual {
    pub fn default(display: &Display, screen: &Screen) -> Self {
        let visual = unsafe { XDefaultVisual(display.display, screen.screen) };
        Self { visual }
    }
}

pub struct WindowAttributesBuilder {
    attributes: XSetWindowAttributes,
}

impl WindowAttributesBuilder {
    pub fn new() -> Self {
        let attributes: XSetWindowAttributes =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        Self { attributes }
    }

    pub fn override_redirect(mut self, b: bool) -> Self {
        self.attributes.override_redirect = b as i32;
        self
    }

    pub fn background_pixel(mut self,pixel: Pixel) -> Self {
        self.attributes.background_pixel = pixel.pixel;
        self
    }

    pub fn backing_pixel(mut self,pixel: Pixel) -> Self {
        self.attributes.backing_pixel = pixel.pixel;
        self
    }
}

pub enum WindowEvent {
    Expose,
}

pub struct Window {
    window: _Window,
    display: *mut x11::xlib::Display
}

impl Window {
    pub fn root_window(display: &Display, screen: &Screen) -> Self {
        let window = unsafe { XRootWindow(display.display, screen.screen) };
        Self { window,display:display.display }
    }

    pub fn create(
        display: &Display,
        parent: Option<Window>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        depth: i32,
        class: WindowClass,
        visual: &Visual,
        valuemask: WindowAttribute,
        mut attributes: WindowAttributesBuilder,
    ) -> Self {
        unsafe {
            let parent = match parent {
                None => 0,
                Some(p) => p.window,
            };

            let window = XCreateWindow(
                display.display,
                parent,
                x as c_int,
                y as c_int,
                width as c_uint,
                height as c_uint,
                border_width as c_uint,
                depth as c_int,
                class as c_uint,
                visual.visual,
                valuemask as c_ulong,
                &mut attributes.attributes,
            );

            Self { window, display: display.display }
        }
    }

    pub fn set_window_title(&self,title: &str) {
        unsafe {
            XStoreName(self.display,self.window,str_to_c_char(title) as *mut c_char);
        }
    }

    pub fn get_geometry(&self) -> Geometry {
        unsafe {
            let root = null_mut();
            let x = null_mut();
            let y = null_mut();
            let width = null_mut();
            let height = null_mut();
            let border_width = null_mut();
            let depth = null_mut();

            XGetGeometry(self.display, self.window, root, x,y,width,height,border_width,depth);

            Geometry {
                x: cast!(x,i32),
                y: cast!(y,i32),
                width: cast!(width,u32),
                height: cast!(height,u32),
                border_width: cast!(border_width,u32),
                depth: cast!(depth,u32),
            }
        }
    }

    pub fn map(&self, display: &Display) {
        unsafe {
            XMapWindow(display.display, self.window);
        }
    }

    pub fn run<F>(&self, func: F)
    where
        F: Fn(WindowEvent, &mut ControlFlow),
    {
        unsafe {
            let mut event = std::mem::MaybeUninit::uninit().assume_init();

            let mut control_flow = ControlFlow::Wait;
            loop {
                XNextEvent(self.display, &mut event);

                match event.type_ {
                    Expose => {
                        func(WindowEvent::Expose, &mut control_flow);
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            XDestroyWindow(self.display,self.window);
        }
    }
}
