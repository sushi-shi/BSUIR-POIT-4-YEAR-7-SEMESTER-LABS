use crate::signals::*;

#[derive(Clone, Copy, Debug)]
pub struct FrequencyMod {
    pub harmonics: [Harmonic; 2],
    pub n: u64,
}

impl FrequencyMod {
    pub fn raise_anchor(anchor: &GtkBox) -> Option<([StringHarmony; 2], GString)> {
        let (widget, _) = get_child(anchor)?;

        let (widget, harm_1) = get_harmony(widget)?;
        let (widget, _) = get_separator(widget)?;

        let (widget, harm_2) = get_harmony(widget)?;
        let (widget, _) = get_separator(widget)?;

        let (_widget, n) = get_discrete(widget)?;

        Some(([harm_1, harm_2], n))
    }

    pub fn parse_anchor(inputs: ([StringHarmony; 2], GString)) -> ResultParse<Self> {
        let [h1, h2] = inputs.0;
        let harm1 = parse_harmony(h1)?;
        let harm2 = parse_harmony(h2)?;
        let n = parse_discrete(&inputs.1)?;
        Ok(FrequencyMod {
            harmonics: [
                Harmonic::new(harm1, n),
                Harmonic::new(harm2, n),
            ],
            n,
        })
    }
}

impl Named for FrequencyMod {
    const NAME: &'static str = "frqnz_mod";
}

impl SignalBox for FrequencyMod {
    fn set(anchor: &GtkBox) {
        set_harmony(anchor, 1.0, 1.0, 0.0);
        set_separator(anchor);
        set_harmony(anchor, 1.0, 1.0, 0.0);
        set_separator(anchor);
        set_discrete(anchor);
    }

    fn get(anchor: &GtkBox) -> ResultParse<Self> {
        FrequencyMod::parse_anchor(
            FrequencyMod::raise_anchor(anchor).expect("FrequencyMod invariants were not satisfied"),
        )
    }
}

impl Signal for FrequencyMod {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        let n_big = (*self).n as f64;
        let carrier = (*self).harmonics[0];
        let message = (*self).harmonics[1];
        Box::new(move |n| {
            let t = n as f64 / n_big;
            let x = 2.0 * PI * carrier.frqnz * t; 
            let y = 2.0 * PI * message.ampltd;
            let z: f64 = (0..=n).map(message.function()).sum();
            
           carrier.ampltd * f64::cos(x + y * z)
        })
    }

    fn draw(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(0..self.n + 1, None, self.function(), path)
    }
}

