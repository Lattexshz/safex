use std::ffi::c_int;
use crate::xlib::{Display, AsRaw, Window};
use x11::glx::*;

pub type GLXAttribute = c_int;

macro_rules! export {
    ($cons:ident,$type_:ident) => {
        pub const $cons: $type_ = x11::glx::$cons as $type_;
    };
}

// Config caveats
export!(GLX_SLOW_CONFIG,GLXAttribute);
export!(GLX_NON_CONFORMANT_CONFIG,GLXAttribute);

// Drawable type mask
export!(GLX_WINDOW_BIT,GLXAttribute);
export!(GLX_PIXMAP_BIT,GLXAttribute);
export!(GLX_PBUFFER_BIT,GLXAttribute);

// Frame buffer attributes
export!(GLX_USE_GL,GLXAttribute);
export!(GLX_BUFFER_SIZE,GLXAttribute);
export!(GLX_LEVEL,GLXAttribute);
export!(GLX_DOUBLEBUFFER,GLXAttribute);
export!(GLX_STEREO,GLXAttribute);
export!(GLX_AUX_BUFFERS,GLXAttribute);
export!(GLX_RED_SIZE,GLXAttribute);
export!(GLX_GREEN_SIZE,GLXAttribute);
export!(GLX_BLUE_SIZE,GLXAttribute);
export!(GLX_ALPHA_SIZE,GLXAttribute);
export!(GLX_DEPTH_SIZE,GLXAttribute);
export!(GLX_STENCIL_SIZE,GLXAttribute);
export!(GLX_ACCUM_RED_SIZE,GLXAttribute);
export!(GLX_ACCUM_GREEN_SIZE,GLXAttribute);
export!(GLX_ACCUM_BLUE_SIZE,GLXAttribute);
export!(GLX_ACCUM_ALPHA_SIZE,GLXAttribute);
export!(GLX_CONFIG_CAVEAT,GLXAttribute);
export!(GLX_X_VISUAL_TYPE,GLXAttribute);
export!(GLX_TRANSPARENT_TYPE,GLXAttribute);
export!(GLX_TRANSPARENT_INDEX_VALUE,GLXAttribute);
export!(GLX_TRANSPARENT_RED_VALUE,GLXAttribute);
export!(GLX_TRANSPARENT_GREEN_VALUE,GLXAttribute);
export!(GLX_TRANSPARENT_BLUE_VALUE,GLXAttribute);
export!(GLX_TRANSPARENT_ALPHA_VALUE,GLXAttribute);
export!(GLX_VISUAL_ID,GLXAttribute);
export!(GLX_SCREEN,GLXAttribute);
export!(GLX_DRAWABLE_TYPE,GLXAttribute);
export!(GLX_RENDER_TYPE,GLXAttribute);
export!(GLX_X_RENDERABLE,GLXAttribute);
export!(GLX_FBCONFIG_ID,GLXAttribute);
export!(GLX_MAX_PBUFFER_WIDTH,GLXAttribute);
export!(GLX_MAX_PBUFFER_HEIGHT,GLXAttribute);
export!(GLX_MAX_PBUFFER_PIXELS,GLXAttribute);
export!(GLX_SAMPLE_BUFFERS,GLXAttribute);
export!(GLX_SAMPLES,GLXAttribute);

// Misc
export!(GLX_DONT_CARE,GLXAttribute);
export!(GLX_NONE,GLXAttribute);

pub struct GLXContext {

}

pub struct GLXWindow {
    inner: Window
}

impl GLXWindow {
    pub fn new() {

    }
}