use crate::signals::*;

#[derive(Clone, Copy, Debug)]
pub struct AmplitudeMod {
    pub harmonics: [Harmonic; 2],
    pub n: u64,
}

impl AmplitudeMod {
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
        Ok(AmplitudeMod {
            harmonics: [Harmonic::new(harm1, n), Harmonic::new(harm2, n)],
            n,
        })
    }
}

impl Named for AmplitudeMod {
    const NAME: &'static str = "ampltd_mod";
}

impl SignalBox for AmplitudeMod {
    fn set(anchor: &GtkBox) {
        set_harmony(anchor, 1.0, 151.0, 0.0);
        set_separator(anchor);
        set_harmony(anchor, 1.0, 2.0, 0.0);
        set_separator(anchor);
        set_discrete(anchor);
    }

    fn get(anchor: &GtkBox) -> ResultParse<Self> {
        AmplitudeMod::parse_anchor(
            AmplitudeMod::raise_anchor(anchor).expect("AmplitudeMod invariants were not satisfied"),
        )
    }
}

impl Signal for AmplitudeMod {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        let n_big = (*self).n as f64;
        let carrier = (*self).harmonics[0];
        let message = (*self).harmonics[1];
        Box::new(move |n| {
            let n = n as f64;
            let x = carrier.ampltd * f64::sin(2.0 * PI * carrier.frqnz * n / n_big);
            let y = f64::sin(2.0 * PI * (carrier.frqnz + message.frqnz) * n / n_big + message.phi);
            let z = f64::sin(2.0 * PI * (carrier.frqnz - message.frqnz) * n / n_big - message.phi);
            x + 0.5 * message.ampltd * (y + z)
        })
    }

    fn draw(
        &self,
        path: &str,
        path_frqnz: &str,
        path_phi: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(
            0..self.n + 1,
            None,
            self.function(),
            path,
            path_frqnz,
            path_phi,
        )
    }
}
