mod seq;
mod picture;
mod window;

use crate::{
    window::Window,
};

use gtk::prelude::*;
use gtk::{
    Application,
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
    app.run();
}

