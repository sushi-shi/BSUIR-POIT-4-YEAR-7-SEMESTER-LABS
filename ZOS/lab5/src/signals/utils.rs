use crate::signals::*;

pub const ERROR_PARSE_AMPLITUDE: &str = "Couldn't parse amplitude";
pub const ERROR_PARSE_PHI: &str = "Couldn't parse phi";
pub const ERROR_PARSE_FREQUENCY: &str = "Couldn't parse frequency";
pub const ERROR_PARSE_DISCRETE: &str = "Couldn't parse discrete number";

pub const DEFAULT_WIDTH: i32 = 8;

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
pub const DRAW_YS_STEP: f64 = 10.0;

pub fn draw_generic(
    signal: Vec<f64>,
    ys: Range<f64>,
    colour: RGBColor,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let xs = 0..signal.len();

    let ys = ys.clone().step((ys.end - ys.start) / DRAW_YS_STEP);

    let root_area = BitMapBackend::new(&path, DRAW_DEFAULT_DIMS).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d(xs.clone(), ys.clone())?;

    ctx.configure_mesh().draw()?;

    ctx.draw_series(LineSeries::new(xs.clone().zip(signal), &colour))?;

    Ok(())
}
