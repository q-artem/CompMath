pub const MIN_POINTS: usize = 4;
pub const MAX_POINTS: usize = 12;

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn variant_function(x: f64) -> f64 {
    (12.0 * x) / (x.powi(4) + 6.0)
}
