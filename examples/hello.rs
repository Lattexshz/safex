use safex::xlib::*;

fn main() {
    let display = Display::open(None);
    let screen = Screen::default(&display);
    let root = Window::root_window(&display, &screen);

    let cmap = ColorMap::default(&display, &screen);
    let color = Color::from_rgb(&display, &cmap, 65535, 0, 0);

    let white = Color::from_rgb(&display, &cmap, 65535, 65535, 65535).get_pixel();
    let black = Color::from_rgb(&display, &cmap, 0, 0, 0).get_pixel();

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
        white,
    );

    window.set_window_title("Hello World");

    let rect = Rectangle {
        x: 10,
        y: 10,
        width: 100,
        height: 100,
        pixel: color.get_pixel(),
    };

    let arc = Arc {
        x: 120,
        y: 10,
        width: 100,
        height: 100,
        angle1: 360 * 64,
        angle2: 360 * 64,
        pixel: color.get_pixel(),
    };

    let rect2 = Rectangle {
        x: 10,
        y: 120,
        width: 100,
        height: 100,
        pixel: color.get_pixel(),
    };

    let arc2 = Arc {
        x: 120,
        y: 120,
        width: 100,
        height: 100,
        angle1: 360 * 64,
        angle2: 360 * 64,
        pixel: color.get_pixel(),
    };

    window.map();
    window.run(|event, control_flow| match event {
        WindowEvent::Expose => {
            window.set_window_background(white);
            window.fill_rectangle(rect);
            window.fill_arc(arc);
            window.draw_rectangle(rect2);
            window.draw_arc(arc2);
            window.draw_string("Hello World", 10, 240, black);
            window.copy_to_buffer();
        }
    })
}
