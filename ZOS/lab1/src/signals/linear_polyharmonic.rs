use crate::signals::*;

pub const ERROR_PARSE_LINEAR: &str = "Values must be in [0.8 .. 1.2] range";

#[derive(Clone, Copy, Debug)]
pub struct LinearPolyharmonic {
    polyharmonic: Polyharmonic,
    ampltd_linear: f64,
    frqnz_linear: f64,
    phi_linear: f64,
}

impl LinearPolyharmonic {
    fn raise_anchor(anchor: &GtkBox) -> Option<(([StringHarmony; 3], GString), StringHarmony)> {
        let (widget, _) = get_child(anchor)?;

        let (widget, harm_1) = get_harmony(widget)?;
        let (widget, _) = get_separator(widget)?;

        let (widget, harm_2) = get_harmony(widget)?;
        let (widget, _) = get_separator(widget)?;

        let (widget, harm_3) = get_harmony(widget)?;
        let (widget, _) = get_separator(widget)?;

        let (widget, n) = get_discrete(widget)?;

        let (widget, _) = get_separator(widget)?;
        let (_widget, harm_4) = get_harmony(widget)?;

        Some((([harm_1, harm_2, harm_3], n), harm_4))
    }

    fn parse_anchor(inputs: (([StringHarmony; 3], GString), StringHarmony)) -> ResultParse<Self> {
        let polyharmonic = Polyharmonic::parse_anchor(inputs.0)?;
        let (a, f, p) = parse_harmony(inputs.1)?;

        for x in &[a, f, p] {
            if !(0.8..=1.2).contains(x) {
                return Err(ERROR_PARSE_LINEAR);
            }
        }

        Ok(LinearPolyharmonic {
            polyharmonic,
            ampltd_linear: a,
            frqnz_linear: f,
            phi_linear: p,
        })
    }
}

impl Named for LinearPolyharmonic {
    const NAME: &'static str = "linear-polyharmonic";
}

impl SignalBox for LinearPolyharmonic {
    fn set(anchor: &GtkBox) {
        Polyharmonic::set(anchor);
        set_separator(anchor);
        set_harmony(anchor, 1.0, 1.0, 1.0);
    }

    fn get(anchor: &GtkBox) -> ResultParse<Self> {
        Self::parse_anchor(
            Self::raise_anchor(anchor).expect("Couldn't uphold LinearPolyharmonic invariants"),
        )
    }
}

impl Signal for LinearPolyharmonic {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        let linear = *self;
        let harms = linear.polyharmonic.harmonics;
        Box::new(move |n| {
            let n = n as f64;
            harms
                .iter()
                .map(|harm| {
                    let period = harm.n as f64 / harm.frqnz;
                    let power = n / period as usize as f64;
                    let ampltd = harm.ampltd * f64::powf(linear.ampltd_linear, power);
                    let frqnz = harm.frqnz * f64::powf(linear.frqnz_linear, power);
                    let phi = harm.phi * f64::powf(linear.phi_linear, power);
                    ampltd * f64::sin((2.0 * PI * frqnz * n) / harm.n as f64 + phi)
                })
                .sum()
        })
    }

    fn draw(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(0..self.polyharmonic.n + 1, None, self.function(), path)
    }
}
