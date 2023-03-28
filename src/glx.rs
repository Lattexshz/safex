use crate::xlib::{AsRaw, CWColormap, CWEventMask, ColorMap, ControlFlow, Display, ExposureMask, InputOutput, KeyPressMask, Screen, Visual, Window, WindowAttributesBuilder, WindowEvent, VisualInfo, WindowClass, Mask};
use std::ffi::{c_int, c_uchar, c_void};
use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;
use x11::glx::*;
use x11::xlib::*;

pub type GLXAttribute = c_int;

macro_rules! export {
    ($cons:ident,$type_:ident) => {
        pub const $cons: $type_ = x11::glx::$cons as $type_;
    };
}

// Config caveats
export!(GLX_SLOW_CONFIG, GLXAttribute);
export!(GLX_NON_CONFORMANT_CONFIG, GLXAttribute);

// Drawable type mask
export!(GLX_WINDOW_BIT, GLXAttribute);
export!(GLX_PIXMAP_BIT, GLXAttribute);
export!(GLX_PBUFFER_BIT, GLXAttribute);

// Frame buffer attributes
export!(GLX_USE_GL, GLXAttribute);
export!(GLX_BUFFER_SIZE, GLXAttribute);
export!(GLX_LEVEL, GLXAttribute);
export!(GLX_DOUBLEBUFFER, GLXAttribute);
export!(GLX_STEREO, GLXAttribute);
export!(GLX_AUX_BUFFERS, GLXAttribute);
export!(GLX_RED_SIZE, GLXAttribute);
export!(GLX_GREEN_SIZE, GLXAttribute);
export!(GLX_BLUE_SIZE, GLXAttribute);
export!(GLX_ALPHA_SIZE, GLXAttribute);
export!(GLX_DEPTH_SIZE, GLXAttribute);
export!(GLX_STENCIL_SIZE, GLXAttribute);
export!(GLX_ACCUM_RED_SIZE, GLXAttribute);
export!(GLX_ACCUM_GREEN_SIZE, GLXAttribute);
export!(GLX_ACCUM_BLUE_SIZE, GLXAttribute);
export!(GLX_ACCUM_ALPHA_SIZE, GLXAttribute);
export!(GLX_CONFIG_CAVEAT, GLXAttribute);
export!(GLX_X_VISUAL_TYPE, GLXAttribute);
export!(GLX_TRANSPARENT_TYPE, GLXAttribute);
export!(GLX_TRANSPARENT_INDEX_VALUE, GLXAttribute);
export!(GLX_TRANSPARENT_RED_VALUE, GLXAttribute);
export!(GLX_TRANSPARENT_GREEN_VALUE, GLXAttribute);
export!(GLX_TRANSPARENT_BLUE_VALUE, GLXAttribute);
export!(GLX_TRANSPARENT_ALPHA_VALUE, GLXAttribute);
export!(GLX_VISUAL_ID, GLXAttribute);
export!(GLX_SCREEN, GLXAttribute);
export!(GLX_DRAWABLE_TYPE, GLXAttribute);
export!(GLX_RENDER_TYPE, GLXAttribute);
export!(GLX_X_RENDERABLE, GLXAttribute);
export!(GLX_FBCONFIG_ID, GLXAttribute);
export!(GLX_MAX_PBUFFER_WIDTH, GLXAttribute);
export!(GLX_MAX_PBUFFER_HEIGHT, GLXAttribute);
export!(GLX_MAX_PBUFFER_PIXELS, GLXAttribute);
export!(GLX_SAMPLE_BUFFERS, GLXAttribute);
export!(GLX_SAMPLES, GLXAttribute);

// Misc
export!(GLX_DONT_CARE, GLXAttribute);
export!(GLX_NONE, GLXAttribute);

export!(GLX_RGBA, GLXAttribute);

pub struct GLXContext {
    glc: x11::glx::GLXContext
}

impl GLXContext {
    pub fn create(display: &Display, vi: &VisualInfo, glc: Option<GLXContext>, flag: i32) -> Self {
        let mut vi = XVisualInfo {
            visual: vi.visual.as_raw(),
            visualid: vi.visualid as VisualID,
            screen: vi.screen.as_raw(),
            depth: vi.depth,
            class: vi.class as c_int,
            red_mask: vi.red_mask,
            green_mask: vi.green_mask,
            blue_mask: vi.blue_mask,
            colormap_size: vi.colormap_size,
            bits_per_rgb: vi.bits_per_rgb,
        };

        let glc = unsafe { glXCreateContext(display.as_raw(), addr_of_mut!(vi),std::ptr::null_mut(), flag as c_int) };
        Self { glc }
    }

    pub fn get_proc_address(&self,display: &Display,screen: &Screen) -> Option<unsafe extern "C" fn()> {
        unsafe {
            let string = glXGetClientString(display.as_raw(),screen.as_raw());
            glXGetProcAddress(string as *const c_uchar)
        }
    }
}

impl AsRaw<x11::glx::GLXContext> for GLXContext {
    fn as_raw(&self) -> x11::glx::GLXContext {
        self.glc
    }
}

pub struct GLXWindow {
    inner: Window,
    display: *mut x11::xlib::Display,
}

impl GLXWindow {
    pub fn new(display: &Display, screen: &Screen, vi: &VisualInfo) -> Result<Self, ()> {
        let root = Window::root_window(display, screen);
        let visual = unsafe { Visual::from_raw(vi.visual.as_raw()) };

        let cmap = ColorMap::create(display, &root, &visual);

        let attribute = WindowAttributesBuilder::new()
            .colormap(cmap)
            .event_mask(ExposureMask | KeyPressMask);

        let inner = Window::create(
            &display,
            &screen,
            None,
            Some(root),
            0,
            0,
            100,
            100,
            1,
            vi.depth,
            InputOutput,
            &visual,
            CWColormap | CWEventMask,
            attribute,
        );

        Ok(Self {
            inner,
            display: display.as_raw(),
        })
    }

    pub fn glx_swap_buffers(&self) {
        unsafe {
            glXSwapBuffers(self.display, self.inner.as_raw());
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
                    _ => {}
                }
            }
        }
    }
}

pub fn glx_choose_visual(display: &Display,attrs: &mut [GLXAttribute]) -> Result<VisualInfo,()> {
    let mut vi = unsafe {
        let mut vi = glXChooseVisual(display.as_raw(), 0, attrs.as_mut_ptr());
        if vi == std::ptr::null_mut() {
            return Err(());
        }
        vi
    };

    let vi = unsafe {
        VisualInfo {
            visual: Visual::from_raw((*vi).visual),
            visualid: (*vi).visualid as u64,
            screen: Screen::from_raw((*vi).screen),
            depth: (*vi).depth as i32,
            class: (*vi).class as WindowClass,
            red_mask: (*vi).red_mask as Mask,
            green_mask: (*vi).green_mask as Mask,
            blue_mask: (*vi).blue_mask as Mask,
            colormap_size: (*vi).colormap_size as i32,
            bits_per_rgb: (*vi).bits_per_rgb as i32,
        }
    };

    Ok(vi)
}

pub fn glx_make_current(display: &Display,window: &GLXWindow,glx: GLXContext) {
    unsafe {
        glXMakeCurrent(display.as_raw(),window.inner.as_raw(),glx.as_raw());
    }
}
