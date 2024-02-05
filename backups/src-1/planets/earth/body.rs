use crate::shared::{calendar::julian::JulianDate, kepler::{body::KeplerianBody, datetime::{KeplerianDate, KeplerianTime}, orbit::OrbitType}, math::cosmic::{Anomaly, AstronicalUnit, Eccentricity, SolarLongitude}, sums::planets::Earth};


impl KeplerianBody for Earth  {
    fn mean_motion(self, day: f64) -> Anomaly {
        0.0
    }
    fn perihelian_elapse(&mut self, day: f64) -> Anomaly {
        0.0
    }
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

    fn true_anomaly(&mut self, day: f64) -> Anomaly {
        0.0
    }
    fn perihelion_date(&self) -> f64 {
        0.0
    }
    fn perihelion_time(&self) -> f64 {
        0.0
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

    fn to_time(&mut self) -> KeplerianTime {
        KeplerianTime::default() 
    }

    fn mean_anomaly(&mut self, day: f64) -> Anomaly {
        todo!()
    }

    fn orbital_shape(&self) -> OrbitType {
        OrbitType::default().shape(self.orbital_eccentricity())
    }
}
