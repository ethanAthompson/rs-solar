//! Each planet, asteroid, moon, exo-planet, or comet date can be,
//! decided by its Ls and it's body information.
//!

use std::f64::consts::PI;

use strum::AsRefStr;

use crate::planets::mars::Martian;

/// Radians in a circle
pub const RIC: f64 = PI * 2.0;

/// Represents an orbiting keplerian body
///
/// Eccentricity + Major + Minor semi axises
///  https://en.wikipedia.org/wiki/Semi-major_and_semi-minor_axes
///  https://www.storyofmathematics.com/conic-sections/
///  https://oer.pressbooks.pub/lynnanegeorge/chapter/chapter-6-keplers-prediction-problem/
///
/// WARNING! Calcualtions in radians
///
/// (Non) means non fact, which is calculated
///
pub trait KeplerianBody {
    /// (Non) The angular position of a body in the elliptic kepler orbit
    // fn eccentric_anomaly(&mut self) -> Anomaly;
    /// (Non) The angle between the current position from perihelion
    fn true_anomaly(&mut self, day: f64) -> Anomaly;
    // / (Non) The fraction of an elliptical orbit period since the perihelion
    // fn mean_anomaly(&mut self) -> Anomaly;
    /// The size of the orbit
    fn semimajor(&self) -> AstronicalUnit;
    /// The length of the shortest radius of an ellipse / hyperbola / conic
    fn semiminor(&self) -> AstronicalUnit;
    /// The shape of the orbit
    fn orbital_eccentricity(&self) -> Eccentricity;
    /// The date that starts the perihelion
    fn perihelion_date(&self) -> f64;
    /// The time that the perihelion is started at
    fn perihelion_time(&self) -> f64;
    /// The shape can be elliptical, hyperbolic, parabolic, circular or straight
    fn what_shape(&self) -> OrbitType {
        match self.orbital_eccentricity() {
            0.0 => OrbitType::Circular,
            0.0..=1.0 => OrbitType::Elliptical,
            1.0 => OrbitType::Parabolic,
            e if e > 1.0 => OrbitType::Hyperbolic,
            f64::INFINITY => OrbitType::Straight,
            _ => OrbitType::Invalid,
        }
    }
    /// The amount of seconds in a day, calibrates when solor or sideral day
    fn day_in_seconds(&self, is_solar: bool) -> f64;
    /// The amount of days in a year, calibrates when leap year
    fn year_in_days(&self, is_leap: bool) -> f64;
    /// The determination of a leap year or not
    fn is_leapyear(&self) -> bool;
    /// The determination of the common equinoxes and solstices (Named version)
    fn seasons_named(&self, solar_longitude: SolarLongitude) -> String;
    /// The determination of the common equinoxes and solstices (Ls version)
    fn seasons_ls(&self, season: Seasons) -> Vec<u32>;
    /// Takes in a julian date to produce a keplerian date
    fn to_date(&mut self, jd: JulianDate) -> KeplerianDate;
    /// Gives you the time, when it works
    fn to_time(&mut self) -> KeplerianDateTime;
    /// Returns the epoch
    fn epoch(&self) -> JulianDate;
    /// computes ls from day given
    fn compute_ls(&mut self, day: f64) -> SolarLongitude;
    /// The average ls for each month
    fn average_ls(&self) -> SolarLongitude;
}

/// The keplerian date returned
#[derive(Default, Debug)]
pub struct KeplerianDate {
    pub year: f64,
    pub month: f64,
    pub day: f64,
    pub ls: f64,
}

#[derive(Default, Debug)]
pub struct KeplerianTime {
    pub hour: f64,
    pub minute: f64,
    pub second: f64,
    pub offset: String,
}

#[derive(Default, Debug)]
pub struct KeplerianDateTime {
    pub date: KeplerianDate,
    pub time: KeplerianTime,
}

pub type JulianDate = f64;

/// The average distance between Earth and the Sun
pub type AstronicalUnit = f64;

/// The calculated position of a keplerian body
pub type Anomaly = f64;

/// The measure of how much a body is from orbiting in a perfect conic circle
pub type Eccentricity = f64;

/// Represents a common standard for celestial bodies
/// Ranging from 0-360 degrees
pub type SolarLongitude = f64;

/// The orbit types generally found in outer space.
pub enum OrbitType {
    Circular,
    Elliptical,
    Parabolic,
    Hyperbolic,
    Straight,
    Invalid,
}

#[derive(AsRefStr, Debug)]
pub enum Seasons {
    #[strum(serialize = "Vernal Equinox")]
    VernalEquinox,
    #[strum(serialize = "Aphelion")]
    Aphelion,
    #[strum(serialize = "Summer Solstice")]
    SummerSolstice,
    #[strum(serialize = "Autumn Equinox")]
    AutumnEquinox,
    #[strum(serialize = "Perihelion")]
    Perihelion,
    #[strum(serialize = "Winter Solstice")]
    WinterSolstice,
    #[strum(serialize = "None")]
    None,
}

// Planets
#[derive(Default, Debug)]
pub struct Earth {}
#[derive(Default, Debug)]
pub struct Mars {}
#[derive(Default)]
pub struct Jupiter {}
#[derive(Default)]
pub struct Saturn {}
#[derive(Default)]
pub struct Mercury {}
#[derive(Default)]
pub struct Neptune {}
// Asteroids
#[derive(Default)]
pub struct Ceres {}
#[derive(Default)]
pub struct Vesta {}
#[derive(Default)]
pub struct Pallas {}
#[derive(Default)]
pub struct Juno {}
// Moons
#[derive(Default)]
pub struct Titan {}
#[derive(Default)]
pub struct Europa {}
#[derive(Default)]
pub struct Luna {}
// Exo Planets
#[derive(Default)]
pub struct Kepler442b {}
#[derive(Default)]
pub struct Kepler22b {}
#[derive(Default)]
pub struct Kepler186f {}
// Comets
#[derive(Default)]
pub struct HaleBopp {}
#[derive(Default)]
pub struct Hailey {}
