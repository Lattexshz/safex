use safex::xlib::*;

fn main() {
    let display = Display::open(None);
    let screen = Screen::default(&display);
    let root = Window::root_window(&display, &screen);
    let visual = Visual::default(&display, &screen);

    let attributes = WindowAttributesBuilder::new().background_pixel(Pixel::white(&display,&screen));
    let window = Window::create(
        &display,
        &screen,
        Some(root),
        0,
        0,
        500,
        500,
        1,
        24,
        InputOutput,
        &visual,
        CWBackPixel,
        attributes,
    );

    window.set_window_title("Hello World");
    let cmap = ColorMap::default(&display,&screen);
    let color = Color::from_rgb(&display,&cmap,65535,0,65535);
    window.set_background_pixel(color.get_pixel());

    window.map(&display);
    window.run(|event, control_flow| match event {
        WindowEvent::Expose => {}
    })
}
