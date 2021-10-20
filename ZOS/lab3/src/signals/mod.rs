use gtk::{glib::GString, prelude::*, Box as GtkBox, Orientation, Separator, Widget};
use plotters::prelude::*;

use crate::widgets::input::Input;

mod fourier;
mod polyharmonic;
mod sawtooth;
mod triangular;
mod utils;
mod wav;
pub mod widget;

pub use polyharmonic::*;
pub use sawtooth::*;
pub use triangular::*;
pub use wav::*;

pub use utils::*;

pub use std::f64::consts::PI;
pub use std::ops::Range;

type OptionBox<T> = Option<(Option<Widget>, T)>;
type ResultParse<T> = Result<T, &'static str>;

pub trait Named {
    const NAME: &'static str;
}

pub trait SignalBox {
    fn set(anchor: &GtkBox)
    where
        Self: Sized;
    fn get(anchor: &GtkBox) -> ResultParse<Self>
    where
        Self: Sized;
}

pub trait Signal {
    fn function(&self) -> Box<dyn Fn(u64) -> f64>;
    fn signal(&self) -> Vec<f64>;
}

impl<S: Signal + ?Sized> Signal for Box<S> {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        (**self).function()
    }
    fn signal(&self) -> Vec<f64> {
        (&**self).signal()
    }
}

pub fn clear_signal(anchor: &GtkBox) {
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
}
