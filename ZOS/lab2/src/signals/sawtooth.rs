use crate::signals::*;

const DEFAULT_DISCREDITATION: usize = 4096;

const ERROR_NOT_POWER_TWO: &str = "Given discretization is not power of two";
const ERROR_DISCRETIZATION_TOO_BIG: &str = "Given discretization is higher 4096";
const ERROR_PARSE_HEIGHT: &str = "Couldn't parse height";

#[derive(Clone, Debug)]
pub struct Sawtooth {
    signal: Vec<f64>,
    step: usize,

    height: f64,
    frqnz: f64,
    n: usize,
}

/// Only one tooth is going to be stored in memory. So modular math it is.
/// `DEFAULT_DISCREDITATION` represents the number of step on the whole graph.
/// 1. Which means our vector should have the size equal to `DEFAULT_DISCREDITATION / frqnz`
/// 2. But we must also skip steps if given discretization is smaller
impl Sawtooth {
    fn new(height: f64, frqnz: f64, n: usize) -> Result<Sawtooth, &'static str> {
        if n > DEFAULT_DISCREDITATION {
            return Err(ERROR_DISCRETIZATION_TOO_BIG);
        }
        if n & (n - 1) != 0 {
            return Err(ERROR_NOT_POWER_TWO);
        }

        let step = DEFAULT_DISCREDITATION / n;
        let len = (DEFAULT_DISCREDITATION as f64 / frqnz) as usize;
        let delta = height / len as f64;
        let signal = (0..=len).map(|x| x as f64 * delta).collect();

        Ok(Sawtooth {
            signal,
            step,

            height,
            frqnz,
            n,
        })
    }

    fn drop_anchor(anchor: &GtkBox) {
        {
            anchor.append(&Input::new_default("height", DEFAULT_WIDTH, "10"));
            anchor.append(&Input::new_default("frqnz", DEFAULT_WIDTH, "4"));
        }
        set_separator(anchor);
        set_discrete(anchor);
    }

    fn raise_anchor(anchor: &GtkBox) -> Option<(GString, GString, GString)> {
        let (widget, ()) = get_child(anchor)?;
        let (widget, h) = get_discrete(widget)?;
        let (widget, f) = get_discrete(widget)?;
        let (widget, ()) = get_separator(widget)?;
        let (_idget, n) = get_discrete(widget)?;

        Some((h, f, n))
    }

    fn parse_anchor(inputs: (GString, GString, GString)) -> ResultParse<Self> {
        let (h, f, n) = inputs;

        let h = parse_f64(&h, ERROR_PARSE_HEIGHT)?;
        let f = parse_f64(&f, ERROR_PARSE_FREQUENCY)?;
        let n = parse_discrete(&n)? as usize;

        Sawtooth::new(h, f, n)
    }
}

impl Named for Sawtooth {
    const NAME: &'static str = "sawtooth";
}

impl SignalBox for Sawtooth {
    fn set(anchor: &GtkBox) {
        Sawtooth::drop_anchor(anchor)
    }

    fn get(anchor: &GtkBox) -> ResultParse<Self> {
        Sawtooth::parse_anchor(Sawtooth::raise_anchor(anchor).expect("Is not sawtoothy"))
    }
}

impl Signal for Sawtooth {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        let tooth = self.clone();
        Box::new(move |n| tooth.signal[n as usize * tooth.step % tooth.signal.len()])
    }

    fn draw(
        &self,
        path: &str,
        path_frqnz: &str,
        path_phi: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(
            0..self.n as u64 + 1,
            Some(0. ..2. * self.height),
            self.function(),
            path,
            path_frqnz,
            path_phi,
        )
    }
}
