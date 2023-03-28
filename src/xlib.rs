use crate::util::str_to_c_char;
use std::ffi::{c_char, c_int, c_uint, c_ulong,c_long, c_ushort, CStr, CString};
use std::mem::MaybeUninit;
use std::ptr::{addr_of, null_mut};
use x11::xlib::*;

type Buffer = c_ulong;

pub type EventMask = c_ulong;

// Re-Exports
macro_rules! export {
    ($cons:ident,$type_:ident) => {
        pub const $cons: $type_ = x11::xlib::$cons as $type_;
    };
}

macro_rules! cast {
    ($val:ident,$type_:ty) => {
        $val as $type_
    };
}

// EventMasks
export!(NoEventMask, EventMask);
export!(KeyPressMask, EventMask);
export!(KeyReleaseMask, EventMask);
export!(ButtonPressMask, EventMask);
export!(ButtonReleaseMask, EventMask);
export!(EnterWindowMask, EventMask);
export!(LeaveWindowMask, EventMask);
export!(PointerMotionMask, EventMask);
export!(PointerMotionHintMask, EventMask);
export!(Button1MotionMask, EventMask);
export!(Button2MotionMask, EventMask);
export!(Button3MotionMask, EventMask);
export!(Button4MotionMask, EventMask);
export!(Button5MotionMask, EventMask);
export!(KeymapStateMask, EventMask);
export!(ExposureMask, EventMask);
export!(VisibilityChangeMask, EventMask);
export!(StructureNotifyMask, EventMask);
export!(ResizeRedirectMask, EventMask);
export!(SubstructureNotifyMask, EventMask);
export!(SubstructureRedirectMask, EventMask);
export!(FocusChangeMask, EventMask);
export!(PropertyChangeMask, EventMask);
export!(ColormapChangeMask, EventMask);
export!(OwnerGrabButtonMask, EventMask);

pub type WindowClass = c_uint;

export!(InputOutput, WindowClass);
export!(InputOnly, WindowClass);

pub type WindowAttribute = c_ulong;

export!(CWBackPixmap, WindowAttribute);
export!(CWBackPixel, WindowAttribute);
export!(CWBorderPixmap, WindowAttribute);
export!(CWBorderPixel, WindowAttribute);
export!(CWBitGravity, WindowAttribute);
export!(CWWinGravity, WindowAttribute);
export!(CWBackingStore, WindowAttribute);
export!(CWBackingPlanes, WindowAttribute);
export!(CWOverrideRedirect, WindowAttribute);
export!(CWSaveUnder, WindowAttribute);
export!(CWEventMask, WindowAttribute);
export!(CWDontPropagate, WindowAttribute);
export!(CWColormap, WindowAttribute);
export!(CWCursor, WindowAttribute);

pub trait AsRaw<T> {
    fn as_raw(&self) -> T;
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Arc {
    pub x:u32,
    pub y:u32,
    pub width:u32,
    pub height:u32,
    pub angle1:u32,
    pub angle2:u32,
    pub pixel: Pixel
}

pub enum ControlFlow {
    Wait,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color<'a> {
    pixel: Pixel,
    red: u16,
    green: u16,
    blue: u16,
    flags: &'a str,
    pad: &'a str,
}

impl<'a> Color<'a> {
    pub fn from_rgb(display: &Display, cmap: &ColorMap, r: u16, g: u16, b: u16) -> Self {
        let mut color: XColor = unsafe { MaybeUninit::uninit().assume_init() };

        color.red = r as c_ushort;
        color.green = g as c_ushort;
        color.blue = b as c_ushort;

        unsafe {
            XAllocColor(display.display, cmap.cmap, &mut color);
        }

        Self {
            pixel: Pixel { pixel: color.pixel },
            red: color.red as u16,
            green: color.green as u16,
            blue: color.blue as u16,
            flags: unsafe { CStr::from_ptr(addr_of!(color.flags)).to_str().unwrap() },
            pad: unsafe { CStr::from_ptr(addr_of!(color.pad)).to_str().unwrap() },
        }
    }

    pub fn get_pixel(&self) -> Pixel {
        self.pixel
    }
}

pub struct ColorMap {
    cmap: c_ulong,
}

impl ColorMap {
    pub fn default(display: &Display, screen: &Screen) -> Self {
        let cmap = unsafe { XDefaultColormap(display.display, screen.screen) };
        Self { cmap }
    }
}

impl AsRaw<c_ulong> for ColorMap {
    fn as_raw(&self) -> c_ulong {
        self.cmap
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

    pub fn flush(&self) {
        unsafe {
            XFlush(self.display);
        }
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

impl AsRaw<*mut x11::xlib::Display> for Display {
    fn as_raw(&self) -> *mut x11::xlib::Display {
        self.display
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub depth: u32,
}

#[derive(Clone,Debug,PartialEq)]
pub struct GC(x11::xlib::GC);

impl GC {

}

impl AsRaw<x11::xlib::GC> for GC {
    fn as_raw(&self) -> x11::xlib::GC {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
    pixel: c_ulong,
}

impl Pixel {
    pub fn black(display: &Display, screen: &Screen) -> Self {
        let pixel = unsafe { XBlackPixel(display.display, screen.screen) };

        Self { pixel }
    }
    pub fn white(display: &Display, screen: &Screen) -> Self {
        let pixel = unsafe { XWhitePixel(display.display, screen.screen) };

        Self { pixel }
    }
    pub fn from_rgb(display: &Display,cmap: &ColorMap,r: u16, g: u16, b: u16) -> Self {
        Color::from_rgb(display,cmap,r,g,b).pixel
    }
}

impl AsRaw<c_ulong> for Pixel {
    fn as_raw(&self) -> c_ulong {
        self.pixel
    }
}

pub struct PixMap {
    pixmap: c_ulong,
}

impl PixMap {
    pub fn create(display: &Display, window: &Window, width: u32, height: u32, depth: u32) -> Self {
        let pixmap = unsafe {
            XCreatePixmap(
                display.display,
                window.buffer,
                width as c_uint,
                height as c_uint,
                depth as c_uint,
            )
        };
        Self { pixmap }
    }

    pub fn from_raw(display: &Display, window: c_ulong, width: u32, height: u32, depth: u32) -> Self {
        let pixmap = unsafe {
            XCreatePixmap(
                display.display,
                window,
                width as c_uint,
                height as c_uint,
                depth as c_uint,
            )
        };
        Self { pixmap }
    }
}

impl AsRaw<c_ulong> for PixMap {
    fn as_raw(&self) -> c_ulong {
        self.pixmap
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rectangle {
    pub x:u32,
    pub y:u32,
    pub width:u32,
    pub height:u32,
    pub pixel: Pixel
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

impl AsRaw<c_int> for Screen {
    fn as_raw(&self) -> c_int {
        self.screen
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

impl AsRaw<*mut x11::xlib::Visual> for Visual {
    fn as_raw(&self) -> *mut x11::xlib::Visual {
        self.visual
    }
}

pub struct WindowAttributesBuilder {
    attributes: XSetWindowAttributes,
}

impl WindowAttributesBuilder {
    pub fn new() -> Self {
        let attributes: XSetWindowAttributes =
            unsafe { MaybeUninit::uninit().assume_init() };

        Self { attributes }
    }

    pub fn override_redirect(mut self, b: bool) -> Self {
        self.attributes.override_redirect = b as i32;
        self
    }

    pub fn background_pixel(mut self, pixel: Pixel) -> Self {
        self.attributes.background_pixel = pixel.as_raw();
        self
    }

    pub fn backing_pixel(mut self, pixel: Pixel) -> Self {
        self.attributes.backing_pixel = pixel.as_raw();
        self
    }

    pub fn colormap(mut self, cmap: ColorMap) -> Self {
        self.attributes.colormap = cmap.as_raw();
        self
    }

    pub fn event_mask(mut self,mask: EventMask) -> Self {
        self.attributes.event_mask = mask as c_long;
        self
    }
}

impl AsRaw<XSetWindowAttributes> for WindowAttributesBuilder {
    fn as_raw(&self) -> XSetWindowAttributes {
        self.attributes
    }
}

pub enum WindowEvent {
    Expose,
}

pub struct Window {
    window: Buffer,
    buffer: Buffer,
    display: *mut x11::xlib::Display,
    gc: GC,
}

impl Window {
    pub fn root_window(display: &Display, screen: &Screen) -> Self {
        let window = unsafe { XRootWindow(display.display, screen.screen) };
        let gc = GC(unsafe { XDefaultGC(display.display, screen.screen) });
        unsafe { XSelectInput(display.display, window, ExposureMask as c_long) };
        Self {
            window,
            buffer: window,
            display: display.display,
            gc,
        }
    }

    pub fn create(
        display: &Display,
        screen: &Screen,
        buffer: Option<()>,
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
                Some(p) => p.buffer,
            };

            let gc = GC(XDefaultGC(display.display, screen.screen));

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

            let buffer = match buffer {
                None => window,
                Some(_) => PixMap::from_raw(&display, window, width, height, depth as u32).pixmap
            };

            unsafe { XSelectInput(display.display, window, ExposureMask as c_long) };

            Self {
                window,
                buffer,
                display: display.display,
                gc
            }
        }
    }

    pub fn create_simple(
        display: &Display,
        screen: &Screen,
        buffer: Option<()>,
        parent: Option<Window>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: u64,
        pixel: Pixel,
    ) -> Self {
        unsafe {
            let parent = match parent {
                None => 0,
                Some(p) => p.buffer,
            };

            let gc = GC(XDefaultGC(display.display, screen.screen));

            let window = XCreateSimpleWindow(
                display.display,
                parent,
                x as c_int,
                y as c_int,
                width as c_uint,
                height as c_uint,
                border_width as c_uint,
                border as c_ulong,
                pixel.pixel,
            );

            let geometry = _get_geometry(display.display,window);

            let buffer = match buffer {
                None => window,
                Some(_) => PixMap::from_raw(&display,window,width,height,geometry.depth).pixmap
            };

            unsafe { XSelectInput(display.display, window, ExposureMask as c_long) };

            Self {
                window,
                buffer,
                display: display.display,
                gc,
            }
        }
    }

    pub fn set_window_title(&self, title: &str) {
        unsafe {
            let title_str = CString::new(title).unwrap();
            XStoreName(self.display, self.window, title_str.as_ptr() as *mut c_char);
        }
    }

    pub fn set_background_pixel(&self, pixel: Pixel) {
        unsafe {
            XSetBackground(self.display, self.gc.as_raw(), pixel.pixel);
        }
    }

    pub fn set_foreground_pixel(&self, pixel: Pixel) {
        unsafe {
            XSetForeground(self.display, self.gc.as_raw(), pixel.pixel);
        }
    }

    pub fn set_window_background(&self, pixel: Pixel) {
        unsafe {
            let geometry = self.get_geometry();
            let rect = Rectangle {
                x: 0,
                y: 0,
                width: geometry.width,
                height: geometry.height,
                pixel,
            };

            self.fill_rectangle(rect);
        }
    }

    pub fn fill_rectangle(&self,rect:Rectangle) {
        unsafe {
            XSetForeground(self.display,self.gc.as_raw(),rect.pixel.pixel);
            XSetBackground(self.display,self.gc.as_raw(),rect.pixel.pixel);
            XFillRectangle(self.display,self.buffer,self.gc.as_raw(),rect.x as c_int,rect.y as c_int,rect.width as c_uint,rect.height as c_uint);
        }
    }

    pub fn draw_rectangle(&self,rect:Rectangle) {
        unsafe {
            XSetForeground(self.display,self.gc.as_raw(),rect.pixel.pixel);
            XSetBackground(self.display,self.gc.as_raw(),rect.pixel.pixel);
            XDrawRectangle(self.display,self.buffer,self.gc.as_raw()    ,rect.x as c_int,rect.y as c_int,rect.width as c_uint,rect.height as c_uint);
        }
    }

    pub fn fill_arc(&self,arc:Arc) {
        unsafe {
            XSetForeground(self.display,self.gc.as_raw(),arc.pixel.pixel);
            XSetBackground(self.display,self.gc.as_raw(),arc.pixel.pixel);
            XFillArc(self.display,self.buffer,self.gc.as_raw(),arc.x as c_int,arc.y as c_int,arc.width as c_uint,arc.height as c_uint,arc.angle1 as c_int,arc.angle2 as c_int);
        }
    }

    pub fn draw_arc(&self,arc:Arc) {
        unsafe {
            XSetForeground(self.display,self.gc.as_raw(),arc.pixel.pixel);
            XSetBackground(self.display,self.gc.as_raw(),arc.pixel.pixel);
            XDrawArc(self.display,self.buffer,self.gc.as_raw(),arc.x as c_int,arc.y as c_int,arc.width as c_uint,arc.height as c_uint,arc.angle1 as c_int,arc.angle2 as c_int);
        }
    }

    pub fn draw_string(&self,string:&str,x:i32,y:i32,color:Pixel) {
        unsafe {
            XSetForeground(self.display,self.gc.as_raw(),color.pixel);
            XSetBackground(self.display,self.gc.as_raw(),color.pixel);
            let len = string.len();
            XDrawString(self.display,self.buffer,self.gc.as_raw(),x as c_int,y as c_int,CString::new(string).unwrap().as_ptr(),len as c_int);
        }
    }

    pub fn get_geometry(&self) -> Geometry {
        unsafe {
            _get_geometry(self.display,self.window)
        }
    }

    pub fn get_gc(&self) -> &GC {
        &self.gc
    }

    pub fn copy_to_buffer(&self) {
        unsafe {
            let geometry = self.get_geometry();

            XCopyArea(
                self.display,
                self.buffer,
                self.window,
                self.gc.as_raw(),
                0,
                0,
                geometry.width as c_uint,
                geometry.height as c_uint,
                0,
                0,
            );
        }
    }

    pub fn flush_gc(&self) {
        unsafe {
            XFlushGC(self.display, self.gc.as_raw());
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
            let mut control_flow = ControlFlow::Wait;
            loop {
                let event = unsafe {
                    let mut event = MaybeUninit::uninit();
                    XNextEvent(self.display, event.as_mut_ptr());
                    event.assume_init()
                };

                match event.type_ {
                    Expose => {
                        func(WindowEvent::Expose, &mut control_flow);
                    }
                    _ => {

                    }
                }
            }
        }
    }

    pub unsafe fn from_raw(display: &Display, screen: &Screen, window: c_ulong,buffer:Option<()>) -> Self {
        let gc = GC(XDefaultGC(display.display, screen.screen));
        let geometry = _get_geometry(display.display,window);
        let buffer = match buffer {
            None => window,
            Some(_) => PixMap::from_raw(&display,window,geometry.width,geometry.height,geometry.depth).pixmap
        };

        Self {
            window,
            buffer,
            display: display.display,
            gc,
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if self.buffer != 0 {
                XDestroyWindow(self.display, self.window);
            }
        }
    }
}

impl AsRaw<c_ulong> for Window {
    fn as_raw(&self) -> c_ulong {
        self.window
    }
}

fn _get_geometry(display: *mut x11::xlib::Display,window:c_ulong) -> Geometry {
    unsafe {
        let mut root = 0;
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;
        let mut border_width = 0;
        let mut depth = 0;

        XGetGeometry(
            display,
            window,
            &mut root,
            &mut x,
            &mut y,
            &mut width,
            &mut height,
            &mut border_width,
            &mut depth,
        );

        Geometry {
            x: cast!(x, i32),
            y: cast!(y, i32),
            width: cast!(width, u32),
            height: cast!(height, u32),
            border_width: cast!(border_width, u32),
            depth: cast!(depth, u32),
        }
    }
}