use glib::clone;
use glib::subclass::InitializingObject;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Box as GtkBox, Button, ComboBoxText, CompositeTemplate, Picture};

use crate::signals::fourier::correlate;
use crate::signals::*;
use crate::*;

use tempdir::TempDir;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "mainwindow.ui")]
pub struct Window {
    #[template_child]
    pub signal_f_selector: TemplateChild<ComboBoxText>,
    #[template_child]
    pub anchor_f: TemplateChild<GtkBox>,

    #[template_child]
    pub signal_g_selector: TemplateChild<ComboBoxText>,
    #[template_child]
    pub anchor_g: TemplateChild<GtkBox>,

    #[template_child]
    pub calc_button: TemplateChild<Button>,

    #[template_child]
    pub picture_f: TemplateChild<Picture>,
    #[template_child]
    pub picture_g: TemplateChild<Picture>,
    #[template_child]
    pub picture_c: TemplateChild<Picture>,
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

        let signal_f_selector = self.signal_f_selector.get();
        let signal_g_selector = self.signal_g_selector.get();

        let anchor_f = self.anchor_f.get();
        let anchor_g = self.anchor_g.get();

        let picture_f = self.picture_f.get();
        let picture_g = self.picture_g.get();
        let picture_c = self.picture_c.get();

        signals::widget::initialize(&signal_f_selector);
        signals::widget::initialize(&signal_g_selector);

        self.signal_f_selector.connect_changed(clone!(
                @weak anchor_f,
                => move |signal_selector: &ComboBoxText | {

            signals::widget::set(&anchor_f, signal_selector);
        }));

        self.signal_g_selector.connect_changed(clone!(
                @weak anchor_g,
                => move |signal_selector: &ComboBoxText | {

            signals::widget::set(&anchor_g, signal_selector);
        }));

        self.calc_button.connect_clicked(clone!(
                @weak signal_f_selector,
                @weak signal_g_selector,
                @weak anchor_f,
                @weak anchor_g,

                @weak picture_f,
                @weak picture_g,
                @weak picture_c,
                => move |_button: &Button| {

            let signal_f = match signals::widget::get(&anchor_f, &signal_f_selector) {
                Err(e) => {
                    display_error_msg(e);
                    return;
                }
                Ok(signal) => signal,
            };
            let signal_g = match signals::widget::get(&anchor_g, &signal_g_selector) {
                Err(e) => {
                    display_error_msg(e);
                    return;
                }
                Ok(signal) => signal,
            };

            let tmp = TempDir::new("signal_drawing").expect("Coulnd't create temporary directory");
            let buffer = tmp.path().join("graph.png");
            let path_f: &str = buffer.to_str().unwrap();

            let tmp = TempDir::new("signal_drawing").expect("Coulnd't create temporary directory");
            let buffer = tmp.path().join("graph.png");
            let path_g: &str = buffer.to_str().unwrap();

            let tmp = TempDir::new("signal_drawing").expect("Coulnd't create temporary directory");
            let buffer = tmp.path().join("graph.png");
            let path_c: &str = buffer.to_str().unwrap();


            let signal_f = signal_f.signal();
            let ys_f = {
                let max = signal_f.iter().fold(f64::NAN, |x, y| f64::max(x, *y));
                let min = signal_f.iter().fold(f64::NAN, |x, y| f64::min(x, *y));
                min..max
            };

            let signal_g = signal_g.signal();
            let ys_g = {
                let max = signal_g.iter().fold(f64::NAN, |x, y| f64::max(x, *y));
                let min = signal_g.iter().fold(f64::NAN, |x, y| f64::min(x, *y));
                min..max
            };

            let signal_c = correlate(&signal_f, &signal_g);
            let ys_c = {
                let max = signal_c.iter().fold(f64::NAN, |x, y| f64::max(x, *y));
                let min = signal_c.iter().fold(f64::NAN, |x, y| f64::min(x, *y));
                min..max
            };

            let _ = draw_generic(signal_f, ys_f, plotters::prelude::RED, path_f);
            let _ = draw_generic(signal_g, ys_g, plotters::prelude::BLUE, path_g);
            println!("{:#?}", signal_c.iter().take(100).collect::<Vec<_>>());
            let _ = draw_generic(signal_c, ys_c, plotters::prelude::BLACK, path_c);

            // let _ = draw_generic(
            //     signal,
            //     fourier_signal,
            //     ys,
            //     path,
            //     path_frq,
            // );

            picture_f.set_filename(Some(path_f));
            picture_g.set_filename(Some(path_g));
            picture_c.set_filename(Some(path_c));
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
