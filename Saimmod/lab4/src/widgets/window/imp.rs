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
use crate::{simulation, simulation::state_to_int};



// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "mainwindow.ui")]
pub struct Window {
    #[template_child]
    pub ro_value: TemplateChild<Input>,
    #[template_child]
    pub queue_length_value: TemplateChild<Input>,
    #[template_child]
    pub pi_1_value: TemplateChild<Input>,
    #[template_child]
    pub pi_2_value: TemplateChild<Input>,

    #[template_child]
    pub calc_button: TemplateChild<Button>,

    #[template_child]
    pub anchor: TemplateChild<Box>,

    #[template_child]
    pub chance_reject_value: TemplateChild<Input>,
    #[template_child]
    pub chance_block_value: TemplateChild<Input>,
    #[template_child]
    pub average_queue_num_value: TemplateChild<Input>,
    #[template_child]
    pub average_system_num_value: TemplateChild<Input>,
    #[template_child]
    pub average_queue_time_value: TemplateChild<Input>,
    #[template_child]
    pub average_system_time_value: TemplateChild<Input>,
    #[template_child]
    pub relative_throughput_value: TemplateChild<Input>,
    #[template_child]
    pub absolute_throughput_value: TemplateChild<Input>,
    #[template_child]
    pub average_channel_load_1_value: TemplateChild<Input>,
    #[template_child]
    pub average_channel_load_2_value: TemplateChild<Input>,
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

        let ro_value = self.ro_value.get();
        ro_value.set_text("0.3");
        let queue_length_value = self.queue_length_value.get();
        queue_length_value.set_text("2");
        let pi_1_value = self.pi_1_value.get();
        pi_1_value.set_text("0.8");
        let pi_2_value = self.pi_2_value.get();
        pi_2_value.set_text("0.75");

        let chance_reject_value = self.chance_reject_value.get();
        let chance_block_value = self.chance_block_value.get();
        let average_queue_num_value = self.average_queue_num_value.get();
        let average_system_num_value = self.average_system_num_value.get();
        let average_queue_time_value = self.average_queue_time_value.get();
        let average_system_time_value = self.average_system_time_value.get();
        let relative_throughput_value = self.relative_throughput_value.get();
        let absolute_throughput_value = self.absolute_throughput_value.get();
        let average_channel_load_1_value = self.average_channel_load_1_value.get();
        let average_channel_load_2_value = self.average_channel_load_2_value.get();

        self.calc_button.connect_clicked(clone!(
                @weak anchor,

                @weak ro_value,
                @weak queue_length_value,
                @weak pi_1_value,
                @weak pi_2_value,

                @weak chance_reject_value,
                @weak chance_block_value,
                @weak average_queue_num_value,
                @weak average_system_num_value,
                @weak average_queue_time_value,
                @weak average_system_time_value,
                @weak relative_throughput_value,
                @weak absolute_throughput_value,
                @weak average_channel_load_1_value,
                @weak average_channel_load_2_value,

                => move |_button: &Button| {
            if let Some((ro, pi_1, pi_2, queue_length)) = {
                let ro = ro_value.text().parse::<f32>();
                let pi_1 = pi_1_value.text().parse::<f32>();
                let pi_2 = pi_2_value.text().parse::<f32>();
                let queue_length = queue_length_value.text().parse::<u8>();
                match (ro, pi_1, pi_2, queue_length) {
                    (Ok(ro), Ok(pi_1), Ok(pi_2), Ok(queue_length)) => Some((ro, pi_1, pi_2, queue_length)),
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

                    let simulation = simulation::simulate(ro, queue_length, pi_1, pi_2);

                    const DEFAULT_WIDTH: i32 = 6;
                    let mut states = simulation.states;
                    states.sort_by_key(|x| x.0);
                    for (name, chance) in states {
                        anchor.append(&Input::new_readonly(&state_to_int(name), &chance.to_string(), DEFAULT_WIDTH));
                    }

                    chance_reject_value.set_text(&simulation.chance_reject.to_string());
                    chance_block_value.set_text(&simulation.chance_block.to_string());
                    average_queue_num_value.set_text(&simulation.average_queue_num.to_string());
                    average_system_num_value.set_text(&simulation.average_system_num.to_string());
                    average_queue_time_value.set_text(&simulation.average_queue_time.to_string());
                    average_system_time_value.set_text(&simulation.average_system_time.to_string());
                    relative_throughput_value.set_text(&simulation.relative_throughput.to_string());
                    absolute_throughput_value.set_text(&simulation.absolute_throughput.to_string());
                    average_channel_load_1_value.set_text(&simulation.average_channel_load_1.to_string());
                    average_channel_load_2_value.set_text(&simulation.average_channel_load_2.to_string());

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
