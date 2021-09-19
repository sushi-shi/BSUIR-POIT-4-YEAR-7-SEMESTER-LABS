use crate::signals::*;

pub const ERROR_PARSE_AMPLITUDE: &str = "Couldn't parse amplitude";
pub const ERROR_PARSE_PHI:       &str = "Couldn't parse phi";
pub const ERROR_PARSE_FREQUENCY: &str = "Couldn't parse frequency";
pub const ERROR_PARSE_DISCRETE:  &str = "Couldn't parse discrete number";

pub const DEFAULT_WIDTH: i32 = 6;

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

pub fn get_harmony(widget: Option<Widget>) -> OptionBox<(GString, GString, GString)> {
    let ampltd_input = widget?
        .downcast::<Input>().ok()?;
    let frqnz_input = ampltd_input
        .next_sibling()?
        .downcast::<Input>().ok()?;
    let phi_input = frqnz_input
        .next_sibling()?
        .downcast::<Input>().ok()?;
    let widget = phi_input.next_sibling();
    
    Some((widget, (ampltd_input.text(), frqnz_input.text(), phi_input.text())))
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
    anchor.append(&Input::new_default("ampltd", DEFAULT_WIDTH, &ampltd.to_string()));
    anchor.append(&Input::new_default("frqnz", DEFAULT_WIDTH, &frqnz.to_string()));
    anchor.append(&Input::new_default("phi", DEFAULT_WIDTH, &phi.to_string()));
}

pub fn parse_harmony(inputs: StringHarmony) -> ResultParse<(f64, f64, f64)> {
    let ampltd = parse_f64(&inputs.0, ERROR_PARSE_AMPLITUDE)?;
    let frqnz  = parse_f64(&inputs.1, ERROR_PARSE_FREQUENCY)?;
    let phi    = parse_f64(&inputs.2, ERROR_PARSE_PHI      )?;

    Ok((ampltd, frqnz, phi))
}

pub const DRAW_DEFAULT_DIMS: (u32, u32) = (800, 600);
pub const DRAW_YS_STEP: f64 = 10.0;

pub fn draw_generic(xs: Range<u64>, ys: Option<Range<f64>>, signal: Box<dyn Fn(u64) -> f64>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let signal: Vec<f64> = xs.clone().map(signal).collect();
        let ys = match ys {
            Some(ys) => ys.clone().step((ys.end - ys.start) / DRAW_YS_STEP),
            None => {
                let max = signal.iter().fold(f64::NAN, |x, y| f64::max(x, *y));
                let min = signal.iter().fold(f64::NAN, |x, y| f64::min(x, *y));

                (min .. max).step((max - min) / DRAW_YS_STEP)
            },
        };

        let root_area = BitMapBackend::new(&path, DRAW_DEFAULT_DIMS).into_drawing_area();
        root_area.fill(&WHITE)?;

        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 20)
            .build_cartesian_2d(xs.clone(), ys)?;

        ctx.configure_mesh().draw()?;

        ctx.draw_series(LineSeries::new(
            xs.zip(signal),
            &BLUE,
        ))?;

        // ctx.draw_series(PointSeries::of_element(
        //     xs.zip(signal),
        //     1 as u32,
        //     &BLUE,
        //     &|c, s, st| {
        //         return EmptyElement::at(c)
        //         + Circle::new((0,0),s,st.filled())
        //     },
        // ))?;

        Ok(())
}

