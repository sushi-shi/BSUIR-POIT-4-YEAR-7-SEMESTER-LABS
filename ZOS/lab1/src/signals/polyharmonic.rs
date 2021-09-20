use crate::signals::*;

#[derive(Clone, Copy, Debug)]
pub struct Polyharmonic {
    pub harmonics: [Harmonic; 3],
    pub n: u64,
}

impl Polyharmonic {
    pub fn raise_anchor(anchor: &GtkBox) -> Option<([StringHarmony; 3], GString)> {
        let (widget, _) = get_child(anchor)?;

        let (widget, harm_1) = get_harmony(widget)?;
        let (widget, _) = get_separator(widget)?;

        let (widget, harm_2) = get_harmony(widget)?;
        let (widget, _) = get_separator(widget)?;

        let (widget, harm_3) = get_harmony(widget)?;
        let (widget, _) = get_separator(widget)?;

        let (_widget, n) = get_discrete(widget)?;

        Some(([harm_1, harm_2, harm_3], n))
    }

    pub fn parse_anchor(inputs: ([StringHarmony; 3], GString)) -> ResultParse<Self> {
        let [h1, h2, h3] = inputs.0;
        let harm1 = parse_harmony(h1)?;
        let harm2 = parse_harmony(h2)?;
        let harm3 = parse_harmony(h3)?;
        let n = parse_discrete(&inputs.1)?;
        Ok(Polyharmonic {
            harmonics: [
                Harmonic::new(harm1, n),
                Harmonic::new(harm2, n),
                Harmonic::new(harm3, n),
            ],
            n,
        })
    }
}

impl Named for Polyharmonic {
    const NAME: &'static str = "polyharmonic";
}

impl SignalBox for Polyharmonic {
    fn set(anchor: &GtkBox) {
        set_harmony(anchor, 1.0, 1.0, 0.0);
        set_separator(anchor);
        set_harmony(anchor, 2.0, 2.0, 0.0);
        set_separator(anchor);
        set_harmony(anchor, 3.0, 3.0, 0.0);
        set_separator(anchor);
        set_discrete(anchor);
    }

    fn get(anchor: &GtkBox) -> ResultParse<Self> {
        Polyharmonic::parse_anchor(
            Polyharmonic::raise_anchor(anchor).expect("Polyharmonic invariants were not satisfied"),
        )
    }
}

impl Signal for Polyharmonic {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        let harms = (*self).harmonics;
        Box::new(move |n| harms.iter().map(|harm| harm.function()(n)).sum())
    }

    fn draw(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(0..self.n + 1, None, self.function(), path)
    }
}
