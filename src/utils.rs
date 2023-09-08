pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-8
}
