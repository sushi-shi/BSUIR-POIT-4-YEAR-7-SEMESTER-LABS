use plotters::prelude::*;
use gtk::{
    Widget,
    Box as GtkBox,
    prelude::*,
    Separator, Orientation,
    glib::GString,
};

use crate::widgets::input::Input;

mod harmonic;
mod linear_polyharmonic;
mod polyharmonic;
mod sawtooth;
mod triangular;
mod wav;
mod utils;
pub mod widget;

pub use harmonic::*;
pub use polyharmonic::*;
pub use linear_polyharmonic::*;
pub use sawtooth::*;
pub use triangular::*;
pub use wav::*;

pub use utils::*;

pub use std::ops::Range;
pub use std::f64::consts::PI;

type OptionBox<T> = Option<(Option<Widget>, T)>;
type ResultParse<T> = Result<T, &'static str>;
type StringHarmony = (GString, GString, GString);


pub trait Named {
    const NAME: &'static str;
}

pub trait SignalBox {
    fn set(anchor: &GtkBox) where Self: Sized;
    fn get(anchor: &GtkBox) -> ResultParse<Self> where Self: Sized;
}

pub trait Signal {
    fn function(&self) -> Box<dyn Fn(u64) -> f64>;
    fn draw(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

impl<S: Signal + ?Sized> Signal for Box<S> {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        (**self).function()
    }

    fn draw(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        (**self).draw(path)
    }
}

pub fn clear_signal(anchor: &GtkBox) -> () {
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

