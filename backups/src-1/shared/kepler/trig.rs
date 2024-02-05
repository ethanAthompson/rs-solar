use std::f64::consts::PI;

/// Radians in a circle
pub const RIC: f64 = PI * 2.0;

#[cfg(test)]
pub mod tests {
    use super::RIC;
    use std::f64::consts::PI;

    #[test]
    pub fn ric_work() {
        assert_eq!(RIC, PI * 2.0);
    }
}
