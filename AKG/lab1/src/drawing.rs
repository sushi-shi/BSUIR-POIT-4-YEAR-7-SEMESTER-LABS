
type Point = (i64, i64);

// pub fn dda_line<T>(x: Point<T>, y: Point<T>) -> Box<dyn Iterator<Item = T>> {
pub fn dda_line(from: Point, to: Point) -> Vec<Point> {
    let dx = (from.0 - to.0).abs();
    let dy = (from.1 - to.1).abs();

    let l = i64::max(dx, dy);

    (0..l).map(|i| (
            (from.0 as f64 + i as f64 * dx as f64 / l as f64).round() as i64,
            (from.1 as f64 + i as f64 * dy as f64 / l as f64).round() as i64
        )).collect()
}
