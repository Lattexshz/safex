use std::ffi::c_void;
use safex::glx::*;
use safex::xlib::*;

fn main() {
    let display = Display::open(None);
    let screen = Screen::default(&display);
    let vi = glx_choose_visual(&display,&mut [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, GLX_NONE]).unwrap();
    let window = GLXWindow::new(
        &display,
        &screen,
        &vi,
    )
    .unwrap();

    let glc = GLXContext::create(&display, &vi, None, gl::TRUE as i32);
    glx_make_current(&display,&window,&glc);

    gl::load_with(|string| {
        glc.get_proc_address(string).unwrap() as *mut c_void
    });

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    window.run(|event, control_flow| match event {
        WindowEvent::Expose => {
            unsafe {
                gl::Viewport(0,0,100,100);
                gl::ClearColor(1.0,1.0,1.0,1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            window.glx_swap_buffers();
        }
    })
}
