use safex::glx::*;
use safex::xlib::*;

fn main() {
    let display = Display::open(None);
    let screen = Screen::default(&display);
    let window = GLXWindow::new(
        &display,
        &screen,
        &mut [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, GLX_NONE],
    )
    .unwrap();
    window.run(|event, control_flow| match event {
        WindowEvent::Expose => {
            window.glx_swap_buffers();
        }
    })
}
