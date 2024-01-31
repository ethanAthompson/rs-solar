use crate::shared::space::{get_eccent, get_kepler, get_peri, get_year};

pub fn martian_year2ls(sol: f64) -> f64 {
    // sols in a martian year
    let year = get_year("Mars");
    let perihelion_date = get_peri("Mars");
    let orbital_eccentricity = get_eccent("Mars");
    let perihelion_time = 2.0 * std::f64::consts::PI * (1.0 - 251.0 / 360.0);
    let semi_major_axis = get_axis("Mars");

    // Math
    let a = semi_major_axis;
    let e = orbital_eccentricity;

    return get_kepler("Mars");
}

#[cfg(test)]
mod tests {
    use super::martian_year2ls;

    #[test]
    pub fn my2ls() {
        martian_year2ls(200.0);
    }
}
