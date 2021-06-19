use fltk::enums::Font;
use fltk::window::DoubleWindow;
use fltk::{app, button::Button, frame::Frame, group::Pack, prelude::*, window::Window};

fn main() {
    println!(
        "starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // Create the wrapper application
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    println!("initialized FLTK");

    // Create the window
    let mut window = create_window(concat!(
        env!("CARGO_PKG_NAME"),
        " v",
        env!("CARGO_PKG_VERSION")
    ));
    println!("created window");

    // Show the window and start the app
    window.show();
    println!("displayed window");
    app.run().unwrap();
    println!("exiting");
}

fn create_window(title: &'static str) -> DoubleWindow {
    // Create the window
    let mut wind = Window::new(100, 100, 400, 300, title);
    wind.make_resizable(true);

    // Create the pack
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_inc = Button::default().with_size(0, 40).with_label("+");
    but_inc.set_label_font(Font::Courier);
    let mut frame = Frame::default().with_size(0, 40).with_label("0");
    let mut but_dec = Button::default().with_size(0, 40).with_label("-");
    pack.end(); // Finish the pack
    wind.end(); // Finish window creation

    wind
}
