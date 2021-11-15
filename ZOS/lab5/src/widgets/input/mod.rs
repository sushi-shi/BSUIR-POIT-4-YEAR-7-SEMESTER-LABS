mod imp;

use glib::{GString, Object};

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct Input(ObjectSubclass<imp::Input>)
    @extends gtk::Box, gtk::Widget,

    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, gtk::Editable;
}

impl Input {
    pub fn new(label: &str, width_chars: i32) -> Self {
        let self_ = Object::new(&[]).expect("Failed to create `Input`.");
        let imp = imp::Input::from_instance(&self_);
        imp.label.set_text(label);
        imp.label.set_width_chars(width_chars);

        self_
    }

    pub fn new_default(label: &str, width_chars: i32, default: &str) -> Self {
        let self_ = Input::new(label, width_chars);
        let imp = imp::Input::from_instance(&self_);
        imp.entry.set_text(default);

        self_
    }

    pub fn new_no_focus(label: &str, width_chars: i32) -> Self {
        let self_ = Input::new(label, width_chars);
        let imp = imp::Input::from_instance(&self_);
        imp.entry.set_can_focus(false);
        imp.entry.set_editable(false);

        self_
    }

    pub fn set_label(&self, label: &str) {
        let imp = imp::Input::from_instance(self);
        imp.label.set_text(label);
    }

    pub fn set_text(&self, text: &str) {
        let imp = imp::Input::from_instance(self);
        imp.entry.set_text(text);
    }

    pub fn text(&self) -> GString {
        let imp = imp::Input::from_instance(self);
        imp.entry.text()
    }
}
