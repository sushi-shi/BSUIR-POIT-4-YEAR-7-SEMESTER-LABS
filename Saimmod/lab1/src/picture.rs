use plotters::prelude::*;

use crate::seq::*;

pub fn draw_image(meta: &LemerSequenceMeta, path: &str) -> Result<(), Box<dyn std::error::Error>> {

    let root_area = BitMapBackend::new(path, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Lemer Distribution", ("sans-serif", 40))
        .build_cartesian_2d(0.0 .. 1.03, 0.0..0.07)?;

    ctx.configure_mesh().draw()?;

    ctx.draw_series((0..).zip(meta.cols.iter()).map(|(x, y)| {
        let delta = 0.05;
        let x0 = x as f64 * delta;
        let x1 = x0 + delta;
        let mut bar = Rectangle::new([(x0, 0.0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))?;

    Ok(())
}
