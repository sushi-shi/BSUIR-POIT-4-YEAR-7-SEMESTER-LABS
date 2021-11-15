use crate::signals::*;
use gtk::{Box as GtkBox, ComboBoxText};

pub const ERROR_NO_SIGNAL: &str = "Signal has not been selected";

pub fn initialize(signal_selector: &ComboBoxText) {
    signal_selector.append_text(Polyharmonic::NAME);
    signal_selector.append_text(Sawtooth::NAME);
    signal_selector.append_text(Triangular::NAME);
    signal_selector.append_text(Wav::NAME);
}

pub fn set(anchor: &GtkBox, signal_selector: &ComboBoxText) {
    clear_signal(anchor);

    match signal_selector.active_text().as_ref().map(GString::as_str) {
        Some(Polyharmonic::NAME) => Polyharmonic::set(anchor),
        Some(Sawtooth::NAME) => Sawtooth::set(anchor),
        Some(Triangular::NAME) => Triangular::set(anchor),
        Some(Wav::NAME) => Wav::set(anchor),
        Some(_) => unimplemented!(),
        None => (),
    }
}

pub fn get(
    anchor: &GtkBox,
    signal_selector: &ComboBoxText,
) -> Result<Box<dyn Signal>, &'static str> {
    match signal_selector.active_text().as_ref().map(GString::as_str) {
        Some(Polyharmonic::NAME) => {
            Polyharmonic::get(anchor).map(|x| Box::new(x) as Box<dyn Signal>)
        }
        Some(Sawtooth::NAME) => Sawtooth::get(anchor).map(|x| Box::new(x) as Box<dyn Signal>),
        Some(Triangular::NAME) => Triangular::get(anchor).map(|x| Box::new(x) as Box<dyn Signal>),
        Some(Wav::NAME) => Wav::get(anchor).map(|x| Box::new(x) as Box<dyn Signal>),
        Some(_) => unimplemented!(),
        None => Err(ERROR_NO_SIGNAL),
    }
}
