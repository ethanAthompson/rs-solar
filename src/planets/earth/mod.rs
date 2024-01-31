use crate::shared::keplerian_body::*;
use std::f64::consts::PI;

impl KeplerianBody for Earth {
    fn semimajor(&self) -> AstronicalUnit {
        1.00000
    }

    fn semiminor(&self) -> AstronicalUnit {
        0.99986
    }
    fn orbital_eccentricity(&self) -> Eccentricity {
        0.017
    }

    fn day_in_seconds(&self, is_solar: bool) -> f64 {
        match is_solar {
            true => 86400.0,
            false => 86164.1,
        }
    }

    fn year_in_days(&self, is_leap: bool) -> f64 {
        match is_leap {
            true => 366.24255,
            false => 365.24225,
        }
    }

    fn is_leapyear(&self) -> bool {
        false
    }

    fn seasons_ls(&self, season: Seasons) -> Vec<u32> {
        match season {
            Seasons::VernalEquinox => (0..90).collect(),
            Seasons::Aphelion => vec![71],
            Seasons::SummerSolstice => (90..180).collect(),
            Seasons::AutumnEquinox => (180..270).collect(),
            Seasons::Perihelion => vec![251],
            Seasons::WinterSolstice => (270..360).collect(),
            Seasons::None => (1000..1001).collect(),
        }
    }
    fn true_anomaly(&mut self, day: f64) -> Anomaly {
        0.0
    }
    fn perihelion_date(&self) -> f64 {
        0.0
    }
    fn perihelion_time(&self) -> f64 {
        0.0
    }
    fn seasons_named(&self, solar_longitude: SolarLongitude) -> String {
        String::new()
    }

    fn to_date(&mut self, jd: JulianDate) -> KeplerianDate {
        KeplerianDate::default()
    }

    fn epoch(&self) -> JulianDate {
        0.0
    }

    fn compute_ls(&mut self, day: f64) -> SolarLongitude {
        0.0
    }
    fn average_ls(&self) -> SolarLongitude {
        0.0
    }
}
