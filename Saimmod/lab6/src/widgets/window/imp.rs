use glib::{
    subclass::InitializingObject,
    clone,
};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Button, CompositeTemplate, 
    Box,
};

use crate::widgets::input::Input;
use crate::{simulation, simulation::pretty_state};



// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "mainwindow.ui")]
pub struct Window {
    #[template_child]
    pub rho: TemplateChild<Input>,
    #[template_child]
    pub mu: TemplateChild<Input>,
    #[template_child]
    pub lambda: TemplateChild<Input>,

    #[template_child]
    pub calc_button: TemplateChild<Button>,

    #[template_child]
    pub anchor: TemplateChild<Box>,

    #[template_child]
    pub chance_reject_1: TemplateChild<Input>,
    #[template_child]
    pub chance_reject_2: TemplateChild<Input>,
    #[template_child]
    pub relative_throughput_1: TemplateChild<Input>,
    #[template_child]
    pub relative_throughput_2: TemplateChild<Input>,
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

        // let mean_value = self.mean_value.get();
        let anchor = self.anchor.get();

        let rho = self.rho.get();
        rho.set_text("0.4");
        let mu = self.mu.get();
        mu.set_text("0.5");

        let lambda = self.lambda.get();
        lambda.set_text("0.45");

        let chance_reject_1 = self.chance_reject_1.get();
        let chance_reject_2 = self.chance_reject_2.get();
        let relative_throughput_1 = self.relative_throughput_1.get();
        let relative_throughput_2 = self.relative_throughput_2.get();

        self.calc_button.connect_clicked(clone!(
                @weak anchor,

                @weak rho,
                @weak mu,
                @weak lambda,

                @weak chance_reject_1,
                @weak chance_reject_2,
                @weak relative_throughput_1,
                @weak relative_throughput_2,


                => move |_button: &Button| {
            if let Some((rho, mu, lambda)) = {
                let rho = rho.text().parse::<f32>();
                let mu = mu.text().parse::<f32>();
                let lambda = lambda.text().parse::<f32>();
                match (rho, mu, lambda) {
                    (Ok(rho), Ok(mu), Ok(lambda)) => Some((rho, mu, lambda)),
                    _ => None,
                }} {
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

                    let simulation = simulation::simulate(mu, lambda, rho);

                    const DEFAULT_WIDTH: i32 = 6;
                    let mut states = simulation.states;
                    states.sort_by_key(|x| x.0);
                    for (name, chance) in states {
                        anchor.append(&Input::new_readonly(&pretty_state(name), &chance.to_string(), DEFAULT_WIDTH));
                    }
                    chance_reject_1.set_text(&simulation.chance_reject_1.to_string());
                    chance_reject_2.set_text(&simulation.chance_reject_2.to_string());
                    relative_throughput_1.set_text(&simulation.relative_throughput_1.to_string());
                    relative_throughput_2.set_text(&simulation.relative_throughput_2.to_string());
            } else {
                display_error_msg("Check ya input");
            }
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
