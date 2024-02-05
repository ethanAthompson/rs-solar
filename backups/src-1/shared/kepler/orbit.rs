use crate::shared::math::cosmic::Eccentricity;

/**
 * The orbit types generally found in outer space.
 * Each orbit type has a specific equation
*/
#[derive(Debug, Default)]
pub enum OrbitType {
    Circular,
    Elliptical,
    Parabolic,
    Hyperbolic,
    Straight,
    #[default]
    Unknown,
}

impl OrbitType {
    /// Gives the shape of the keplerian body based of orbital shpae deviation
    pub fn shape(&self, obe: Eccentricity) -> Self {
        match obe {
            e if e == 0.0 => Self::Circular,
            e if e > 0.0 && e < 1.0 => Self::Elliptical,
            e if e == 1.0 => Self::Parabolic,
            e if e > 1.0 => Self::Hyperbolic,
            e if e == f64::INFINITY => Self::Straight,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::shared::{kepler::body::KeplerianBody, sums::planets::Earth};

    #[test]
    pub fn orbittype_work() {
        // println!("{:?}", Mars.orbital_shape());
        println!("{:?}", Earth.orbital_shape());
    }
}
