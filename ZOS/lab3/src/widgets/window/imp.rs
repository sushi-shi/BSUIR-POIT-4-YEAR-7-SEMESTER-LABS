use glib::clone;
use glib::subclass::InitializingObject;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Box as GtkBox, Scale, Button, ComboBoxText, CompositeTemplate, Picture, Switch};

use crate::*;
use crate::signals::{draw_generic, apply_filters};
use crate::widgets::input::Input;

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

    #[template_child]
    pub min_switch: TemplateChild<Switch>,
    #[template_child]
    pub min_scale: TemplateChild<Scale>,

    #[template_child]
    pub max_switch: TemplateChild<Switch>,
    #[template_child]
    pub max_scale: TemplateChild<Scale>,

    #[template_child]
    pub mov_switch: TemplateChild<Switch>,
    #[template_child]
    pub mov_input: TemplateChild<Input>,
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

        self.min_scale.set_range(0., 100.);
        self.max_scale.set_range(0., 100.);
        self.min_scale.set_value(0.);
        self.max_scale.set_value(100.);
        self.mov_input.set_text("1");

        let signal_selector = self.signal_selector.get();
        let anchor = self.anchor.get();
        let picture = self.picture.get();
        let picture_frqnz = self.picture_frqnz.get();

        let min_scale = self.min_scale.get();
        let max_scale = self.max_scale.get();
        let mov_input = self.mov_input.get();

        let min_switch = self.min_switch.get();
        let max_switch = self.max_switch.get();
        let mov_switch = self.mov_switch.get();

        signals::widget::initialize(&signal_selector);

        self.signal_selector.connect_changed(clone!(
                @weak anchor,
                => move |signal_selector: &ComboBoxText | {

            signals::widget::set(&anchor, signal_selector);
        }));

        self.calc_button.connect_clicked(clone!(
                @weak signal_selector,
                @weak anchor,

                @weak min_scale,
                @weak max_scale,
                @weak mov_input,

                @weak min_switch,
                @weak max_switch,
                @weak mov_switch,

                @weak picture,
                @weak picture_frqnz,
                => move |_button: &Button| {
            let mov = match mov_input.text().parse::<usize>() {
                Ok(v) if v > 0 => v,
                _ => {
                    display_error_msg("Not positive");
                    return;
                }
            };

            match signals::widget::get(&anchor, &signal_selector) {
                Err(e) => display_error_msg(e),
                Ok(signal) => {
                    let tmp = TempDir::new("signal_drawing").expect("Coulnd't create temporary directory");

                    let buffer = tmp.path().join("graph.png");
                    let path: &str = buffer.to_str().unwrap();

                    let signal = signal.signal();
                    let ys = {
                        let max = signal.iter().fold(f64::NAN, |x, y| f64::max(x, *y));
                        let min = signal.iter().fold(f64::NAN, |x, y| f64::min(x, *y));
                        min..max
                    };
                    let fourier_signal = apply_filters(
                        signal.clone(),
                        if mov_switch.state() {
                            Some(mov)
                        } else {
                            None
                        },
                        if min_switch.state() {
                            Some(signal.len() * min_scale.value() as usize / 200)
                        } else {
                            None
                        },
                        if max_switch.state() {
                            Some(signal.len() * max_scale.value() as usize / 200)
                        } else {
                            None
                        },
                    );

                    let _ = draw_generic(
                        signal, 
                        fourier_signal,
                        ys,
                        path, 
                    );

                    picture.set_filename(Some(path));
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
