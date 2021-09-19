use plotters::prelude::*;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::combinators::Linspace;

pub struct GraphInfo {
    pub cols: Vec<f64>,
    pub min_x: f64,
    pub max_x: f64,
    pub max_y: f64,
}

pub fn draw_graphs(ginfo: &GraphInfo, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let delta_x = (ginfo.max_x - ginfo.min_x) / ginfo.cols.len() as f64;
    let delta_y = ginfo.max_y / 7.0;

    let root_area = BitMapBackend::new(path, (440, 300)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx: ChartContext<_, Cartesian2d<Linspace<RangedCoordf64, f64, _>, _>> = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d((ginfo.min_x .. ginfo.max_x).step(delta_x), (0.0 .. ginfo.max_y * 1.2).step(delta_y))?;

    ctx.configure_mesh().draw()?;

    ctx.draw_series((0 ..).zip(ginfo.cols.iter()).map(|(x, y)| {
        let x0 = ginfo.min_x + x as f64 * delta_x;
        let x1 = x0 + delta_x;
        let mut bar = Rectangle::new([(x0, 0.0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))?;

    Ok(())
}
