use displaydoc::Display;
use strum::AsRefStr;

use crate::orbit::{Perihelion, SemiAxis};

/// This trait acts as a common field for all planets, asteroids, moons, exo-planets, and comets
///
/// ## Limitations
/// `Only Solar`: Sidereal days is not supported
///
pub trait Body {    
    /// Calculates the reference point which the body was discovered
    fn epoch(&self) -> f64;
    /// Calculates the deviation of an orbit's path from a perfect circle.
    fn orbital_eccentricity(&self) -> f64;
    /// Calculates the days in time it takes a body to orbit a host body that's the sun or a planet.
    fn orbital_period(&self) -> f64;
    /// Calculates the seconds in time it takes a body to rotate on its' axis.
    fn rotational_period(&self) -> f64;
    /// Calculates the average distance of this body from the sun.
    fn semimajor(&self) -> f64;
    /// Calculates the shortest distance between the center of the body to the edge of the body.
    fn semiminor(&self) -> f64 {
        SemiAxis(self.semimajor()).minor(self.orbital_eccentricity())
    }
    /// A wrapper that's shared throughout the code
    fn perihelion(&self) -> Perihelion;
    /// Calculates the mean motion which is the perihelian elapse.
    /// * Mean Motion Equation
    /// > $$n={\frac {2\pi }{P}}$$
    /// 
    /// - `n` is the mean motion
    /// - `P` is the orbital period
    fn mean_motion(&mut self, day: f64) -> f64;
    /// Final Calculation into date
    fn to_date(&mut self, julian_date: f64) -> Date;
    /// Final Calculation into time
    fn to_time(&mut self) -> Time;
}

#[derive(Debug, Default, AsRefStr, Clone, Copy)]
/// This represents eras that the date is in
pub enum Eras {
    #[strum(serialize = "AD")]
    /// After Discovery, this must be a present or latter date.
    AD,
    #[strum(serialize = "BD")]
    /// Before Discovery, this must be an old date.
    BD,
    #[default]
    /// Unkown, you must've entered something wrong.
    Unknown,
}

#[derive(Display, Debug, Default, Clone)]
/// This is a collection of what a date should consist of
pub struct Date {
    /// This is the era of body
    pub era: Eras,
    /// This is the year of body
    pub year: f64,
    /// This is the month of body
    pub month: f64,
    /// This is the day of body
    pub day: f64,
    /// This is the ls of the body
    pub ls: f64,
    /// This is the season of the body (Optional)
    pub season: String,
}

#[derive(Display, Debug, Default)]
/// This is a collection of what a time should consist of
pub struct Time {
    /// This is the hour of the body
    pub hour: f64,
    /// This is the minute of the body
    pub minute: f64,
    /// This is the second of the body
    pub second: f64,
    /// This is the offset of the body
    pub offset: String,
    /// This is the hour type of body (Millitary Time or 12Hour)
    pub hour_type: String,
}
