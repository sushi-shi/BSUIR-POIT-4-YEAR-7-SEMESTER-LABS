use crate::signals::*;
use hound;
use std::fs;

pub const ERR_OPEN: &str = "Cannot open file";
pub const ERR_SELECT: &str = "File is not selected";

pub const WAV_PATH: &str = "/home/sheep/Documents/labs/ZOS/lab1/example/";

#[derive(Clone)]
pub struct Wav {
    samples: Vec<i32>,
}

impl Wav {
    pub fn new(path: &str) -> Result<Wav, &'static str> {
        let mut reader = hound::WavReader::open(path).map_err(|_| ERR_OPEN)?;
        let samples = reader.samples::<i32>().filter_map(Result::ok).collect();
        Ok(Wav {
            samples,
        })
    }
}

impl Named for Wav {
    const NAME: &'static str = "wav";
}

impl SignalBox for Wav {
    fn set(anchor: &GtkBox) -> () { 
        let file_selector = gtk::ComboBoxText::new();
        let iter = fs::read_dir(WAV_PATH).unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name() != ".." && entry.file_name() != ".");
        for entry in iter {
            file_selector.append_text(entry.file_name().to_str().unwrap())
        }
        anchor.append(&file_selector)
    }

    fn get(anchor: &GtkBox)  -> ResultParse<Self> {
        let file_selector = anchor.first_child().unwrap().downcast::<gtk::ComboBoxText>().unwrap();
        let name = file_selector.active_text();
        let name = name.as_ref().ok_or(ERR_SELECT)?;
        
        Wav::new(&format!("{}{}", WAV_PATH, name))
    }
    
}

impl Signal for Wav {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        let wav = self.clone();
        Box::new(move |n| {
            wav.samples[n as usize] as f64
        })
    }

    fn draw(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(0 .. self.samples.len() as u64 / 400, None, self.function(), path)
    }
}
