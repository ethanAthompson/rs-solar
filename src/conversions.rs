/// This function converts astronomical units to kilometers
///
/// > $$1AU = 1.495978707 * 10^{11}
///
/// Numbers are according to https://en.wikipedia.org/wiki/Astronomical_unit
///
/// ### Example Of Venus orbiting average distance (0.723) AU
///
/// ```rust
///  use rust_solar::conversions::au2km;
///
///  // The AU of venus's' semi-major axis is 0.723
///  assert_eq!(108159260516.09999, au2km(0.723));
///
/// ```
///
pub fn au2km(value: f64) -> f64 {
    value * (1.495_978_707 * 100_000_000_000.0)
}

/// This function is a wrapper over calculating the radians in a circle
///
pub fn radians_in_circle() -> f64 {
    std::f64::consts::PI * 2.0
}
