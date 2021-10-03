use crate::signals::*;
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Entry, Label};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "input.ui")]
pub struct Input {
    #[template_child(id = "input_label")]
    pub label: TemplateChild<Label>,

    #[template_child(id = "input_entry")]
    pub entry: TemplateChild<Entry>,

    #[template_child(id = "input_minus")]
    pub minus: TemplateChild<Button>,

    #[template_child(id = "input_plus")]
    pub plus: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Input {
    const NAME: &'static str = "Input";
    type Type = super::Input;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Input {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let entry = self.entry.get();

        self.minus.connect_clicked(clone!(
            @weak entry
            => move |_button: &Button| {
                entry.set_text(
                    &(parse_f64(&entry.text(), "").unwrap() - 1.).to_string()
                );
        }));

        self.plus.connect_clicked(clone!(
            @weak entry
            => move |_button: &Button| {
                entry.set_text(
                    &(parse_f64(&entry.text(), "").unwrap() + 1.).to_string()
                );
        }));
    }
}

// Trait shared by all widgets
impl WidgetImpl for Input {
    fn grab_focus(&self, _widget: &Self::Type) -> bool {
        self.entry.grab_focus()
    }
}

impl BoxImpl for Input {}

// FIXME: it panics when you try to use it, sad
impl EditableImpl for Input {
    fn delegate(&self, _input: &Self::Type) -> Option<gtk::Editable> {
        Some(self.entry.clone().upcast())
    }
}
