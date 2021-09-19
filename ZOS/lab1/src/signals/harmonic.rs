use crate::signals::{
    *,
    utils::{
        set_separator,
        get_separator,
        set_discrete,
        get_discrete,
        parse_discrete,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct Harmonic {
    pub ampltd: f64,
    pub frqnz: f64,
    pub phi: f64,
    pub n: u64,
}

impl Harmonic {
    pub fn new(harm: (f64, f64, f64), n: u64) -> Harmonic {
        Harmonic {
            ampltd: harm.0,
            frqnz: harm.1,
            phi: harm.2,
            n,
        }
    }

    fn drop_anchor(anchor: &GtkBox) {
        set_harmony(anchor, 1.0, 1.0, 0.0);
        set_separator(anchor);
        set_discrete(anchor);
    }

    fn raise_anchor(anchor: &GtkBox) -> Option<((GString, GString, GString), GString)> {
        let (widget, ()) = get_child(anchor)?;
        let (widget, (ampltd, frqnz, phi)) = get_harmony(widget)?;
        let (widget, ()) = get_separator(widget)?;
        let (_widget, n) = get_discrete(widget)?;
        Some(((ampltd, frqnz, phi), n))
    }

    pub fn parse_anchor(inputs: ((GString, GString, GString), GString)) -> ResultParse<Harmonic> {
        let (ampltd, frqnz, phi) = parse_harmony(inputs.0)?;
        let n = parse_discrete(&inputs.1)?;

        Ok(Harmonic {
            ampltd, frqnz, phi, n
        })
    }
}

impl Named for Harmonic {
    const NAME: &'static str = "harmonic";
}

impl SignalBox for Harmonic {
    fn set(anchor: &GtkBox) -> () {
        Harmonic::drop_anchor(anchor)
    }

    fn get(anchor: &GtkBox) -> ResultParse<Self> {
        Harmonic::parse_anchor(
            Harmonic::raise_anchor(anchor).expect("Is not harmonic")
        )
    }
}

impl Signal for Harmonic {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        let harm = self.clone(); // self-harm, haha
        Box::new(move |n| {
            harm.ampltd * f64::sin((2.0 * PI * harm.frqnz * n as f64) / harm.n as f64 + harm.phi)
        })
    }

    fn draw(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(0 .. self.n + 1, Some(-self.ampltd .. self.ampltd), self.function(), path)
    }
}
