use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib::{Value, ParamFlags, ParamSpec}, glib, CompositeTemplate, Label, Entry};
use once_cell::sync::Lazy;


// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(file = "input.ui")]
pub struct Input {
    #[template_child(id = "input_label")]
    pub label: TemplateChild<Label>,

    #[template_child(id = "input_entry")]
    pub entry: TemplateChild<Entry>,
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
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpec::new_int(
                    "width",
                    "width",
                    "width",
                    0,
                    i32::MAX,
                    6,
                    ParamFlags::READWRITE,
                ),
                ParamSpec::new_string(
                    "label",
                    "label",
                    "label",
                    None,
                    ParamFlags::READWRITE,
                ),
                ParamSpec::new_boolean(
                    "readonly",
                    "readonly",
                    "readonly",
                    false,
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "width" => {
                let width = value.get().expect("The value needs to be of type `i32`.");
                self.label.set_width_chars(width)
            }
            "label" => {
                let text = value.get().expect("The value need to be of type `String`.");
                self.label.set_text(text)
            }
            "readonly" => {
                let is_readonly: bool = value.get().expect("The value need to be of type `bool`.");
                self.entry.set_can_focus(!is_readonly);
                self.entry.set_editable(!is_readonly);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "width" => {
                self.label.width_chars().to_value()
            }
            "label" => {
                self.label.text().to_value()
            }
            "readonly" => {
                self.entry.can_focus().to_value()
            }
            _ => unimplemented!(),
        }
    }


    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}


// Trait shared by all widgets
impl WidgetImpl for Input {
    fn grab_focus(&self, _widget: &Self::Type) -> bool {
        self.entry.grab_focus()
    }
}

impl BoxImpl for Input {}

