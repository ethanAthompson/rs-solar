//! Each planet, asteroid, moon, exo-planet, or comet date can be,
//! decided by its Ls and it's body information.
use crate::shared::{calendar::julian::JulianDate, math::cosmic::*};
use super::{datetime::*, orbit::OrbitType};


/// Represents an orbiting keplerian body
pub trait KeplerianBody {
    // Utilities
    fn mean_motion(self, day: f64) -> Anomaly;
    fn true_anomaly(&mut self, day: f64) -> Anomaly;
    fn mean_anomaly(&mut self, day: f64) -> Anomaly;
    fn semimajor(&self) -> AstronicalUnit;
    fn semiminor(&self) -> AstronicalUnit;
    fn orbital_eccentricity(&self) -> Eccentricity;
    fn orbital_shape(&self) -> OrbitType;
    // how many seconds does it take to complete 1 day
    fn day_in_seconds(&self, is_solar: bool) -> f64;
    // How many days does to complete 1 revolution
    fn year_in_days(&self, is_leap: bool) -> f64;
    fn is_leapyear(&self) -> bool;
    fn epoch(&self) -> JulianDate;
    fn compute_ls(&mut self, day: f64) -> SolarLongitude;
    fn average_ls(&self) -> SolarLongitude;

    // Key Operations
    fn perihelian_elapse(&mut self, day: f64) -> Anomaly;
    fn perihelion_date(&self) -> f64;
    fn perihelion_time(&self) -> f64;
    fn to_date(&mut self, jd: JulianDate) -> KeplerianDate;
    fn to_time(&mut self) -> KeplerianTime;
}
