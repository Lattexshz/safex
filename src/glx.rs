use crate::xlib::{Display, AsRaw, Window};
use x11::glx::*;

pub fn enable_glx(window: &Window,display: &Display) -> Result<(),()> {
    unsafe {
        let attr = [ GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, GLX_NONE ];
        let mut vi = glXChooseVisual(display.as_raw(), 0, attr.as_mut_ptr());
        if vi = std::ptr::null_mut() {
            return Err(());
        }
    }
    Ok(())
}