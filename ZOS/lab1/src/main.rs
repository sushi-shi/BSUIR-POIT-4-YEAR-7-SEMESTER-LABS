mod picture;
mod widgets;
mod signals;

use crate::{
    widgets::window::Window,
};

use gtk::{
    Application,
    prelude::*,
};

fn build_ui(app: &Application)  {
    let window = Window::new(app);
    window.present();
}

fn main() {
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

