use glib::{
    MainContext,
    subclass::InitializingObject,
    PRIORITY_DEFAULT,
};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Entry, Button, CompositeTemplate, Label, Picture};

use crate::seq::{
    DEFAULT_ARGUMENTS,
    COLUMN_LENGTH,
    InfiniteLemerSequence,
    analyze_sequence,
};

use tempdir::TempDir;
use glib::clone;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(file = "mainwindow.ui")]
pub struct Window {
    #[template_child]
    pub mult_entry: TemplateChild<Entry>,
    #[template_child]
    pub init_entry: TemplateChild<Entry>,
    #[template_child]
    pub mod_entry: TemplateChild<Entry>,

    #[template_child]
    pub calc_button: TemplateChild<Button>,

    #[template_child]
    pub mean_value: TemplateChild<Label>,
    #[template_child]
    pub disp_value: TemplateChild<Label>,
    #[template_child]
    pub devi_value: TemplateChild<Label>,
    #[template_child]
    pub indir_value: TemplateChild<Label>,
    #[template_child]
    pub len_value: TemplateChild<Label>,
    #[template_child]
    pub period_len_value: TemplateChild<Label>,
    #[template_child]
    pub aperiod_len_value: TemplateChild<Label>,

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
        
        // TODO: Do we take ownership when dereferencing value here?
        // We shouldn't be able to, as it MUST belong to a Window, no?
        //
        // Quote from a book:
        // GObjects are reference-counted and mutable
        //
        // Seems like we just clone it, Entry must be a wrapper similar to Rc.
        let mult_entry: &Entry = &*self.mult_entry;
        let init_entry = &*self.init_entry;
        let mod_entry = &*self.mod_entry;

        let mean_value = &*self.mean_value;
        let disp_value = &*self.disp_value;
        let devi_value = &*self.devi_value;
        let indir_value = &*self.indir_value;

        let len_value = &*self.len_value;
        let period_len_value = &*self.period_len_value;
        let aperiod_len_value = &*self.aperiod_len_value;

        let picture = &*self.picture;
        let button  = &*self.calc_button;

        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        self.calc_button.connect_clicked(clone!(
                @weak mult_entry,
                @weak init_entry,
                @weak mod_entry,
                => move |button: &Button| {
                // Try parsing all given entries
                let a = match mult_entry.buffer().text().parse::<u64>() {
                    Err(_) => {
                        display_error_msg("Couldn't parse multiplier");
                        return
                    },
                    Ok(x) => x,
                };
                let r_0 = match init_entry.buffer().text().parse::<u64>() {
                    Err(_) => {
                        display_error_msg("Couldn't parse initial value");
                        return
                    },
                    Ok(x) => x,
                };
                let m = match mod_entry.buffer().text().parse::<u64>() {
                    Err(_) => {
                        display_error_msg("Couldn't parse modulus");
                        return
                    },
                    Ok(x) => x,
                };

                button.set_sensitive(false);

                // TODO: What is clone doing here exactly?
                let sender = sender.clone();

                std::thread::spawn(move || {
                    let meta = analyze_sequence(InfiniteLemerSequence::new(a, r_0, m), COLUMN_LENGTH);
                    sender.send(meta).expect("Couldn't send through the channel");
                });
        }));

        receiver.attach(
            None,
            clone!(
                @weak button,

                @weak mean_value, 
                @weak disp_value,
                @weak devi_value,
                @weak indir_value,

                @weak len_value,
                @weak period_len_value,
                @weak aperiod_len_value,

                @weak picture,
                => @default-return Continue(false),
                    move |meta| {


                let tmp = TempDir::new("lemer").expect("Coulnd't create temporary directory");
                // FIXME: This is ugly
                let path: String = tmp.path().join("lemer.png").to_str().expect("Couldn't name temp dir").to_string();

                crate::picture::draw_image(&meta, &path).expect("Couldn't draw a graph");
                picture.set_filename(Some(&path));

                mean_value.set_label(&meta.mean.to_string());
                disp_value.set_label(&meta.dispersion.to_string());
                devi_value.set_label(&meta.deviation.to_string());
                indir_value.set_label(&meta.indirect_ratio.to_string());

                len_value.set_label(&meta.len.to_string());
                period_len_value.set_label(&meta.period_len.to_string());
                aperiod_len_value.set_label(&meta.aperiod_len.to_string());

                button.set_sensitive(true);
                Continue(true)
        }));

        self.mult_entry.set_text(DEFAULT_ARGUMENTS.0);
        self.init_entry.set_text(DEFAULT_ARGUMENTS.1);
        self.mod_entry.set_text(DEFAULT_ARGUMENTS.2);

    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application
impl ApplicationWindowImpl for Window {}


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
