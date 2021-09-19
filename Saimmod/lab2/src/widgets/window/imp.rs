use glib::{
    subclass::InitializingObject,
    GString,
};
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate, Entry, Picture,
    ComboBoxText, Box,
};

use crate::widgets::distribution;

use tempdir::TempDir;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "mainwindow.ui")]
pub struct Window {
    #[template_child]
    pub sequence_selector: TemplateChild<ComboBoxText>,

    #[template_child]
    pub calc_button: TemplateChild<Button>,

    #[template_child]
    pub anchor: TemplateChild<Box>,

    #[template_child]
    pub mean_value: TemplateChild<Entry>,
    #[template_child]
    pub disp_value: TemplateChild<Entry>,
    #[template_child]
    pub devi_value: TemplateChild<Entry>,

    #[template_child]
    pub picture: TemplateChild<Picture>,

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

        let mean_value = self.mean_value.get();
        let disp_value = self.disp_value.get();
        let devi_value = self.devi_value.get();
        let sequence_selector = self.sequence_selector.get();
        let anchor = self.anchor.get();
        let picture = self.picture.get();

        // FIXME: It seems like it is possible to use ComboBox 
        // and store the whole enum in there
        // This way we can go back to our cozy typed world
        self.sequence_selector.append_text("uniform");
        self.sequence_selector.append_text("gaussian");
        self.sequence_selector.append_text("exponential");
        self.sequence_selector.append_text("gamma");
        self.sequence_selector.append_text("triangle");
        self.sequence_selector.append_text("simpson");

        self.sequence_selector.connect_changed(clone!(
                @weak anchor,
                => move |sequence_selector: &ComboBoxText | {

            // Delete all children
            let mut m_child = anchor.first_child();
            loop {
                match m_child {
                    None => break,
                    Some(child) => { 
                        m_child = child.next_sibling();
                        anchor.remove(&child);
                    }
                }
            }
            match sequence_selector.active_text().as_ref().map(GString::as_str) {
                Some("uniform") => distribution::set_uniform(&anchor),
                Some("gaussian") => distribution::set_gaussian(&anchor),
                Some("exponential") => distribution::set_exponential(&anchor),
                Some("gamma") => distribution::set_gamma(&anchor),
                Some("triangle") => distribution::set_triangle(&anchor),
                Some("simpson") => distribution::set_simpson(&anchor),
                Some(_) => unimplemented!(),
                None => (),
            }
        }));

        self.calc_button.connect_clicked(clone!(
                @weak mean_value, 
                @weak disp_value,
                @weak devi_value,

                @weak sequence_selector,
                @weak anchor,

                @weak picture,
                => move |_button: &Button| {
            // FIXME: We can use threads instead
            let stats_m =  
                match sequence_selector.active_text().as_ref().map(GString::as_str) {
                    Some("uniform") => distribution::calc_uniform(&anchor),
                    Some("gaussian") => distribution::calc_gaussian(&anchor),
                    Some("exponential") => distribution::calc_exponential(&anchor),
                    Some("gamma") => distribution::calc_gamma(&anchor),
                    Some("triangle") => distribution::calc_triangle(&anchor),
                    Some("simpson") => distribution::calc_simpson(&anchor),
                    Some(_) => unimplemented!(),
                    None => {
                        display_error_msg("First choose distribution");
                        None
                    }, 
                };
            match stats_m {
                None => display_error_msg("Check input"),
                Some((mean, disp, devi, cols)) => {
                    mean_value.set_text(&mean.to_string());
                    disp_value.set_text(&disp.to_string());
                    devi_value.set_text(&devi.to_string());

                    let tmp = TempDir::new("lemer").expect("Coulnd't create temporary directory");
                    // FIXME: This is ugly
                    let path: String = tmp.path().join("graph.png").to_str().expect("Couldn't name temp dir").to_string();

                    crate::picture::draw_graphs(&cols, &path).expect("Couldn't draw a graph");
                    picture.set_filename(Some(&path));
                }
            };
        }));

    }
}



fn display_error_msg(text: &'static str) {
    gtk::glib::MainContext::default().spawn_local(dialog(text));
}


async fn dialog(text: &str) {
    let msg = gtk::MessageDialog::builder()
        .message_type(gtk::MessageType::Error)
        .buttons(gtk::ButtonsType::Ok)
        .text("Error!")
        .secondary_text(text)
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
