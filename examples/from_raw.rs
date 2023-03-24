use safex::xlib::*;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    match window.raw_window_handle() {
        RawWindowHandle::UiKit(_) => {}
        RawWindowHandle::AppKit(_) => {}
        RawWindowHandle::Orbital(_) => {}
        RawWindowHandle::Xlib(handle) => {
            let display = Display::open(None);
            let screen = Screen::default(&Display);
            let window = unsafe { Window::from_raw(&display,&screen,handle.window) };
            window.set_window_title("Hello World from SafeX");
            window.map(&display);
            window.run(|event,control_flow| {
                match event {
                    WindowEvent::Expose => {
                        println!("Expose!");
                    }
                }
            })
        }
        RawWindowHandle::Xcb(_) => {}
        RawWindowHandle::Wayland(_) => {}
        RawWindowHandle::Drm(_) => {}
        RawWindowHandle::Gbm(_) => {}
        RawWindowHandle::Win32(_) => {}
        RawWindowHandle::WinRt(_) => {}
        RawWindowHandle::Web(_) => {}
        RawWindowHandle::AndroidNdk(_) => {}
        RawWindowHandle::Haiku(_) => {}
        _ => {}
    }
}