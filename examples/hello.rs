use safex::xlib::*;

fn main() {
    let display = Display::open(None);
    let screen = Screen::default(&display);
    let root = Window::root_window(&display, &screen);
    let visual = Visual::default(&display, &screen);

    let attributes = WindowAttributesBuilder::new().background_pixel(Pixel::white(&display,&screen));
    let window = Window::create(
        &display,
        Some(root),
        0,
        0,
        500,
        500,
        1,
        24,
        InputOutput,
        &visual,
        0,
        attributes,
    );

    window.map(&display);
    window.run(|event, control_flow| match event {
        WindowEvent::Expose => {}
    })
}
