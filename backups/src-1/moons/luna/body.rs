use crate::shared::{
    calendar::julian::{get_jd, jd2greg, JulianDate},
    kepler::{
        body::KeplerianBody,
        datetime::{KeplerianDate, KeplerianTime},
        orbit::OrbitType,
        seasons::Seasons,
        trig::RIC,
    },
    math::cosmic::{Anomaly, AstronicalUnit, Eccentricity, SolarLongitude},
    sums::planets::Earth,
    sums::moons::Luna
};

use std::f64::consts::PI;

/// https://amaral.northwestern.edu/blog/lunar-standard-time#:~:text=The%20Moon%20experiences%20about%2014.77,29.53%2F30%20%3D%200.984%20seconds.
impl KeplerianBody for Luna {
    fn semimajor(&self) -> AstronicalUnit {
        0.3844
    }

    /// https://en.wikipedia.org/wiki/Semi-major_and_semi-minor_axes
    /// b = a * Sqrt(1-e^2): b = Semi Minor Axis, a = Semi Major Axis
    fn semiminor(&self) -> AstronicalUnit {
        self.semimajor() * (1.0 - self.orbital_eccentricity().powf(2.0))
    }

    fn orbital_eccentricity(&self) -> Eccentricity {
        0.0549
    }


    fn epoch(&self) -> JulianDate {

        // You usually see the moon better, since gallileo may of lived near Pisa
        // so 8pm CET is 7pm UTC. so 6pm UTC is 7pm CET
        let date = get_jd(1609, 11, 30, 6.0);

        // November 11 1609 6:00:00 UTC
        jd2greg(date);

        date
        
    }

    fn mean_motion(self, day: f64) -> Anomaly {
        todo!()
    }

    fn true_anomaly(&mut self, day: f64) -> Anomaly {
        todo!()
    }

    fn mean_anomaly(&mut self, day: f64) -> Anomaly {
        todo!()
    }

    fn orbital_shape(&self) -> OrbitType {
        OrbitType::default().shape(self.orbital_eccentricity())
    }

    fn day_in_seconds(&self, is_solar: bool) -> f64 {
        let is_synodic = is_solar;

        match is_synodic {
            // During synodic period: New Moon -> New Moon
            true => Earth::day_in_seconds(true) * 29.53,
            // During regular
            false => Earth::day_in_seconds(true) * 27.3
        }
    }

    fn year_in_days(&self, is_leap: bool) -> f64 {
        match is_leap {
            true => 29.53,
            false => 27.3,
        }
    }

    fn is_leapyear(&self) -> bool {
        false
    }

    fn compute_ls(&mut self, day: f64) -> SolarLongitude {
        todo!()
    }

    fn average_ls(&self) -> SolarLongitude {
        todo!()
    }

    fn perihelian_elapse(&mut self, day: f64) -> Anomaly {
        todo!()
    }

    fn perihelion_date(&self) -> f64 {
        todo!()
    }

    fn perihelion_time(&self) -> f64 {
        todo!()
    }

    fn to_date(&mut self, jd: JulianDate) -> KeplerianDate {
        todo!()
    }

    fn to_time(&mut self) -> KeplerianTime {
        todo!()
    }


}