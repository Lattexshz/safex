use safex::xlib::*;

fn main() {
    let display = Display::open(None);
    let screen = Screen::default(&display);
    let root = Window::root_window(&display, &screen);

    let cmap = ColorMap::default(&display, &screen);
    let color = Color::from_rgb(&display, &cmap, 65535, 0, 0);

    let window = Window::create_simple(
        &display,
        &screen,
        Some(()),
        Some(root),
        0,
        0,
        500,
        500,
        1,
        0,
        Color::from_rgb(&display, &cmap, 65535, 65535, 65535).get_pixel(),
    );

    window.set_window_title("Hello World");

    let rect = Rectangle {
        x: 10,
        y: 10,
        width: 100,
        height: 100,
        pixel: color.get_pixel(),
    };

    window.map(&display);
    window.run(|event, control_flow| match event {
        WindowEvent::Expose => {
            window.fill_rectangle(rect);
            window.copy_to_buffer();
        }
    })
}
