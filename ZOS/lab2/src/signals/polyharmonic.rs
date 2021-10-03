use crate::signals::*;

#[derive(Clone, Debug)]
pub struct Polyharmonic {
    pub harmonics: Vec<Harmony<f64>>,
    pub n: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Harmony<T> {
    pub ampltd: T,
    pub frqnz: T,
    pub phi: T,
}

impl Harmony<f64> {
    fn function(&self, n_big: u64) -> Box<dyn Fn(u64) -> f64> {
        let harm = *self; // self-harm, haha
        Box::new(move |n| {
            harm.ampltd * f64::sin((2.0 * PI * harm.frqnz * n as f64) / n_big as f64 + harm.phi)
        })
    }
}

fn get_harmony(widget: Option<Widget>) -> OptionBox<Harmony<GString>> {
    let ampltd_input = widget?.downcast::<Input>().ok()?;
    let frqnz_input = ampltd_input.next_sibling()?.downcast::<Input>().ok()?;
    let phi_input = frqnz_input.next_sibling()?.downcast::<Input>().ok()?;
    let widget = phi_input.next_sibling();

    Some((
        widget,
        Harmony {
            ampltd: ampltd_input.text(),
            frqnz: frqnz_input.text(),
            phi: phi_input.text(),
        },
    ))
}

fn parse_harmony(inputs: Harmony<GString>) -> ResultParse<Harmony<f64>> {
    let ampltd = parse_f64(&inputs.ampltd, ERROR_PARSE_AMPLITUDE)?;
    let frqnz = parse_f64(&inputs.frqnz, ERROR_PARSE_FREQUENCY)?;
    let phi = parse_f64(&inputs.phi, ERROR_PARSE_PHI)?;

    Ok(Harmony { ampltd, frqnz, phi })
}

impl Polyharmonic {
    pub fn raise_anchor(anchor: &GtkBox) -> Option<(Vec<Harmony<GString>>, GString)> {
        let (mut widget, _) = get_child(anchor)?;
        let mut harmony;
        let mut harmonies = Vec::new();

        for _ in 0..3 {
            let tmp = get_harmony(widget)?;
            widget = tmp.0;
            harmony = tmp.1;

            let tmp = get_separator(widget)?;
            widget = tmp.0;

            harmonies.push(harmony);
        }

        let (_widget, n) = get_discrete(widget)?;

        Some((harmonies, n))
    }

    pub fn parse_anchor(inputs: (Vec<Harmony<GString>>, GString)) -> ResultParse<Self> {
        let harmonies = inputs
            .0
            .into_iter()
            .map(parse_harmony)
            .collect::<ResultParse<Vec<Harmony<f64>>>>()?;
        let n = parse_discrete(&inputs.1)?;

        Ok(Polyharmonic {
            harmonics: harmonies,
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
        let poly = self.clone();
        Box::new(move |n| {
            poly.harmonics
                .iter()
                .map(|harm| harm.function(poly.n)(n))
                .sum()
        })
    }

    fn draw(&self, path: &str, path_frqnz: &str) -> Result<(), Box<dyn std::error::Error>> {
        draw_generic(0..self.n + 1, None, self.function(), path, path_frqnz)
    }
}
