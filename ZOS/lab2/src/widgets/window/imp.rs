use glib::clone;
use glib::subclass::InitializingObject;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Box as GtkBox, Button, ComboBoxText, CompositeTemplate, Picture};

use crate::*;

use tempdir::TempDir;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "mainwindow.ui")]
pub struct Window {
    #[template_child]
    pub signal_selector: TemplateChild<ComboBoxText>,

    #[template_child]
    pub calc_button: TemplateChild<Button>,

    #[template_child]
    pub anchor: TemplateChild<GtkBox>,

    #[template_child]
    pub picture: TemplateChild<Picture>,

    #[template_child]
    pub picture_frqnz: TemplateChild<Picture>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        let signal_selector = self.signal_selector.get();
        let anchor = self.anchor.get();
        let picture = self.picture.get();
        let picture_frqnz = self.picture_frqnz.get();

        signals::widget::initialize(&signal_selector);

        self.signal_selector.connect_changed(clone!(
                @weak anchor,
                => move |signal_selector: &ComboBoxText | {

            signals::widget::set(&anchor, signal_selector);
        }));

        self.calc_button.connect_clicked(clone!(
                @weak signal_selector,
                @weak anchor,

                @weak picture,
                @weak picture_frqnz,
                => move |_button: &Button| {

            match signals::widget::get(&anchor, &signal_selector) {
                Err(e) => display_error_msg(e),
                Ok(signal) => {
                    let tmp = TempDir::new("signal_drawing").expect("Coulnd't create temporary directory");

                    let buffer = tmp.path().join("graph.png");
                    let path: &str = buffer.to_str().unwrap();

                    let tmp = TempDir::new("signal_drawing").expect("Coulnd't create temporary directory");

                    let buffer = tmp.path().join("graph.png");
                    let path_frqnz: &str = buffer.to_str().unwrap();

                    let _ = signals::Signal::draw(&signal, path, path_frqnz);
                    picture.set_filename(Some(path));
                    picture_frqnz.set_filename(Some(path_frqnz));
                }
            };
        }));
    }
}

fn display_error_msg(text: &'static str) {
    gtk::glib::MainContext::default().spawn_local(dialog(text));
}

async fn dialog(text: &'static str) {
    let msg = gtk::MessageDialog::builder()
        .message_type(gtk::MessageType::Error)
        .buttons(gtk::ButtonsType::Ok)
        .text("Error!")
        .secondary_text(text) // Cursed
        .build();
    let _response = msg.run_future().await;
    msg.close();
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application
impl ApplicationWindowImpl for Window {}
