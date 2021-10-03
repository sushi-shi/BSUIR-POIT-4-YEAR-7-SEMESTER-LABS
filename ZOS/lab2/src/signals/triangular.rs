use crate::signals::*;

const DEFAULT_DISCREDITATION: usize = 4096;

const ERROR_NOT_POWER_TWO: &str = "Given discretization is not power of two";
const ERROR_DISCRETIZATION_TOO_BIG: &str = "Given discretization is higher 4096";
const ERROR_PARSE_HEIGHT: &str = "Couldn't parse height";

#[derive(Clone, Debug)]
pub struct Triangular {
    signal: Vec<f64>,
    step: usize,

    height: f64,
    frqnz: f64,
    n: usize,
}

impl Triangular {
    fn new(height: f64, frqnz: f64, n: usize) -> Result<Triangular, &'static str> {
        if n > DEFAULT_DISCREDITATION {
            return Err(ERROR_DISCRETIZATION_TOO_BIG);
        }
        if n & (n - 1) != 0 {
            return Err(ERROR_NOT_POWER_TWO);
        }

        let step = DEFAULT_DISCREDITATION / n;
        let len = (DEFAULT_DISCREDITATION as f64 / (frqnz * 4.0)) as usize;
        let delta = height / len as f64;
        let signal = (-height as usize / 2..=len)
            .map(|x| x as f64 * delta)
            .collect();

        Ok(Triangular {
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

        Triangular::new(h, f, n)
    }
}

impl Named for Triangular {
    const NAME: &'static str = "triangular";
}

impl SignalBox for Triangular {
    fn set(anchor: &GtkBox) {
        Triangular::drop_anchor(anchor)
    }

    fn get(anchor: &GtkBox) -> ResultParse<Self> {
        Triangular::parse_anchor(Triangular::raise_anchor(anchor).expect("Is not Triangulary"))
    }
}

impl Signal for Triangular {
    fn function(&self) -> Box<dyn Fn(u64) -> f64> {
        let tooth = self.clone();
        Box::new(move |n| {
            let len = tooth.signal.len();
            let n = n as usize * tooth.step % (len * 4);
            let mul = if n < 2 * len { 1.0 } else { -1.0 };
            mul * tooth.signal[if n < len {
                n
            } else if n < 2 * len {
                2 * len - n - 1
            } else if n < 3 * len {
                n - 2 * len
            } else {
                4 * len - n - 1
            }]
        })
    }

    fn draw(&self, path: &str, path_frqnz: &str) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(
            0..self.n as u64 + 1,
            Some(-1.0 - self.height..self.height + 1.0),
            self.function(),
            path,
            path_frqnz,
        )
    }
}
