use crate::signals::fourier::{discrete_fourier, inverse_discrete_fourier};
use crate::signals::*;

pub const ERROR_PARSE_AMPLITUDE: &str = "Couldn't parse amplitude";
pub const ERROR_PARSE_PHI: &str = "Couldn't parse phi";
pub const ERROR_PARSE_FREQUENCY: &str = "Couldn't parse frequency";
pub const ERROR_PARSE_DISCRETE: &str = "Couldn't parse discrete number";

pub const DEFAULT_WIDTH: i32 = 6;

pub fn get_harmony(widget: Option<Widget>) -> OptionBox<(GString, GString, GString)> {
    let ampltd_input = widget?.downcast::<Input>().ok()?;
    let frqnz_input = ampltd_input.next_sibling()?.downcast::<Input>().ok()?;
    let phi_input = frqnz_input.next_sibling()?.downcast::<Input>().ok()?;
    let widget = phi_input.next_sibling();

    Some((
        widget,
        (ampltd_input.text(), frqnz_input.text(), phi_input.text()),
    ))
}

pub fn parse_harmony(inputs: (GString, GString, GString)) -> ResultParse<(f64, f64, f64)> {
    let ampltd = parse_f64(&inputs.0, ERROR_PARSE_AMPLITUDE)?;
    let frqnz = parse_f64(&inputs.1, ERROR_PARSE_FREQUENCY)?;
    let phi = parse_f64(&inputs.2, ERROR_PARSE_PHI)?;

    Ok((ampltd, frqnz, phi))
}

pub fn get_child(anchor: &GtkBox) -> OptionBox<()> {
    Some((anchor.first_child(), ()))
}

pub fn get_separator(widget: Option<Widget>) -> OptionBox<()> {
    Some((widget?.next_sibling(), ()))
}

pub fn get_input(widget: Option<Widget>) -> OptionBox<GString> {
    let widget = widget?.downcast::<Input>().ok()?;
    Some((widget.next_sibling(), widget.text()))
}

pub fn get_discrete(widget: Option<Widget>) -> OptionBox<GString> {
    get_input(widget)
}

pub fn parse_f64(input: &str, err_msg: &'static str) -> ResultParse<f64> {
    input.parse::<f64>().map_err(|_| err_msg)
}

pub fn set_separator(anchor: &GtkBox) {
    anchor.append(&Separator::new(Orientation::Horizontal));
}

pub fn set_discrete(anchor: &GtkBox) {
    anchor.append(&Input::new_default("N", DEFAULT_WIDTH, "512"));
}

pub fn parse_discrete(input: &str) -> ResultParse<u64> {
    input.parse::<u64>().map_err(|_| ERROR_PARSE_DISCRETE)
}

pub fn set_harmony(anchor: &GtkBox, ampltd: f64, frqnz: f64, phi: f64) {
    anchor.append(&Input::new_default(
        "ampltd",
        DEFAULT_WIDTH,
        &ampltd.to_string(),
    ));
    anchor.append(&Input::new_default(
        "frqnz",
        DEFAULT_WIDTH,
        &frqnz.to_string(),
    ));
    anchor.append(&Input::new_default("phi", DEFAULT_WIDTH, &phi.to_string()));
}

pub const DRAW_DEFAULT_DIMS: (u32, u32) = (1600, 1200);
pub const DRAW_DEFAULT_DIMS_FRQNZ: (u32, u32) = (1600, 800);
pub const DRAW_YS_STEP: f64 = 10.0;

pub fn draw_generic(
    xs: Range<u64>,
    ys: Option<Range<f64>>,
    signal: Box<dyn Fn(u64) -> f64>,

    path: &str,
    path_frqnz: &str,
    path_phi: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let signal: Vec<f64> = xs.clone().map(signal).collect();
    let ys = match ys {
        Some(ys) => ys.clone(),
        None => {
            let max = signal.iter().fold(f64::NAN, |x, y| f64::max(x, *y));
            let min = signal.iter().fold(f64::NAN, |x, y| f64::min(x, *y));

            min..max
        }
    };
    let n_big = xs.end as usize;
    let xs_frqnz = xs.start..xs.end / 2;
    let ys_frqnz = (0. ..ys.end).step((ys.end - ys.start) / DRAW_YS_STEP);

    let ys = ys.clone().step((ys.end - ys.start) / DRAW_YS_STEP);

    let fourier = discrete_fourier(xs.end, &signal)
        .into_iter()
        .take(n_big / 2)
        .enumerate()
        .map(|(i, c)| Harmony {
            ampltd: c.norm(),
            phi: -f64::atan(c.im / c.re),
            frqnz: i as f64,
        })
        .collect::<Vec<_>>();

    let fourier_signal = inverse_discrete_fourier(n_big, &(discrete_fourier(xs.end, &signal)));
    for (sig, fsig) in signal.iter().zip(fourier_signal.iter()).take(5) {
        println!("{:?}, {:?}", sig, fsig);
    }

    let root_area = BitMapBackend::new(&path, DRAW_DEFAULT_DIMS).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d(xs.clone(), ys.clone())?;

    ctx.configure_mesh().draw()?;

    ctx.draw_series(LineSeries::new(xs.clone().zip(fourier_signal), &RED))?;
    ctx.draw_series(LineSeries::new(xs.clone().zip(signal), &BLUE))?;

    // FRQNZ
    let root_area = BitMapBackend::new(&path_frqnz, DRAW_DEFAULT_DIMS_FRQNZ).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d(xs_frqnz.clone(), ys_frqnz.clone())?;

    ctx.configure_mesh().draw()?;

    for harmony in &fourier {
        ctx.draw_series(LineSeries::new(
            vec![
                (harmony.frqnz as u64, 0.),
                (harmony.frqnz as u64, harmony.ampltd),
            ],
            &RED,
        ))?;
    }

    // PHI
    let root_area = BitMapBackend::new(&path_phi, DRAW_DEFAULT_DIMS_FRQNZ).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d(xs_frqnz, (-0.5 * PI..0.5 * PI).step(0.1))?;

    ctx.configure_mesh().draw()?;

    for harmony in &fourier {
        ctx.draw_series(LineSeries::new(
            vec![
                (harmony.frqnz as u64, 0.),
                (harmony.frqnz as u64, harmony.phi),
            ],
            &RED,
        ))?;
    }

    Ok(())
}
