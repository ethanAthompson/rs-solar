use crate::conversions::radians_in_circle;
use strum::AsRefStr;

#[derive(Debug, Default, Clone, Copy)]
/// This is the collection of orbital types a body would follow
pub enum Type {
    /// The orbit path is round, like a donut.
    Circular,
    /// The orbit path is oval, like an egg not a treadmill.
    Elliptical,
    /// The orbit path is boomerang shaped.
    Parabolic,
    /// The orbit path is like the sides of a guitar.
    Hyperbolic,
    /// The orbit path is like a pencil (how?)
    Straight,
    /// The orbit path is unknown, you probably entered incorrect data.
    #[default]
    Unknown,
}

impl Type {
    /// Gives the shape of the keplerian body based of orbital shpae deviation
    pub fn shape(&self, obe: f64) -> Self {
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

#[derive(Debug, Clone, Copy)]
/// This data structure contains perihelion data.
pub struct Perihelion {
    /// ### (Start, End)
    pub month: (f64, f64),
    /// ### (Start, End)
    pub ls: (f64, f64),
    /// ### The solar longitude of the perihelion
    pub perihelion: f64,
}

impl Perihelion {
    /// The days since the the perihelion by the orbital_period and day in planet
    pub fn elapse(&mut self, day: f64, orbital_period: f64) -> f64 {
        (day - self.date()) / orbital_period
    }

    /// The date of the perihelion by the orbital period
    pub fn date(&mut self) -> f64 {
        let avg_days = self.month.1 - self.month.0;
        let avg_ls = self.ls.1 - self.ls.0;
        let until_peri = self.perihelion - self.ls.0;
        let peri_day = avg_days / avg_ls;

        (peri_day * until_peri) + self.month.0
    }

    /// The time of the perihelion within the orbit
    pub fn time(&mut self) -> f64 {
        radians_in_circle() * (1.0 - self.perihelion / 360.0)
    }

    /// The average solar longitude between the start and end of the perihelion
    pub fn avg_ls(&mut self) -> f64 {
        self.ls.1 - self.ls.0
    }
}

#[derive(Debug, Copy, Clone)]
/// This is the data for calculating solar longitude among orbiting bodies.
pub struct SolarLongitude(pub f64);

impl SolarLongitude {
    /// This method computes the ls which should be given by [`kepler::Body`].
    /// * The final computation is in *degrees*
    ///
    pub fn compute(&mut self) -> f64 {
        if self.0 < 0.0 {
            self.0 += radians_in_circle();
        }

        if self.0 > radians_in_circle() {
            self.0 -= radians_in_circle();
        }

        self.0.to_degrees()
    }
}

#[derive(Debug, Copy, Clone)]
/// This structure is for the semi axises of an ellipse
pub struct SemiAxis(pub f64);

impl SemiAxis {
    /// This is just a wrapper to return the major axis.
    ///
    pub fn major(self) -> f64 {
        self.0
    }

    /// Calculates the shortest distance between the center of the body to the edge of the body.
    ///
    /// ```rust
    ///
    /// use crate::rust_solar::planets::mars::Mars;
    /// use crate::rust_solar::kepler::Body;
    /// use crate::rust_solar::orbit::SemiAxis;
    /// 
    /// let martian_semi_minor_axis = SemiAxis(Mars.semimajor()).minor(Mars.orbital_eccentricity());
    ///
    /// assert_eq!(1.5067401888, martian_semi_minor_axis)
    ///
    /// ```
    pub fn minor(self, orbital_eccentricity: f64) -> f64 {
        self.major() * (1.0 - orbital_eccentricity.powf(2.0))
    }
}



/// The colelction of seasons in which all keplerian bodies follow
#[derive(AsRefStr, Debug, Default, Copy, Clone)]
pub enum Season {
    /// March 19th
    #[strum(serialize = "Vernal Equinox")]
    VernalEquinox,

    /// July 6 
    #[strum(serialize = "Aphelion")]
    Aphelion,

    /// June 20th -> June 21st
    #[strum(serialize = "Summer Solstice")]
    SummerSolstice,

    /// September 22
    #[strum(serialize = "Autumn Equinox")]
    AutumnEquinox,

    /// January 3rd
    #[strum(serialize = "Perihelion")]
    Perihelion,

    /// December 21st -> December 22nd
    #[strum(serialize = "Winter Solstice")]
    WinterSolstice,

    /// This structure is not the problem, it must be the solar longitude,
    /// or maybe the planet you chose doesn't have seasons?
    #[strum(serialize = "N/A")]
    #[default]
    Unknown,
}

impl Season {
    /// This method creates a season given a solar longitude.
    pub fn from(&self, ls: u32) -> String {
        match ls  {
            71 => Self::Aphelion,
            251 => Self::Perihelion,
            0..=90 => Self::VernalEquinox,
            91..=180 => Self::SummerSolstice,
            181..=270 => Self::AutumnEquinox,
            271..=360 => Self::WinterSolstice,
            _ => Self::Unknown
        }
        .as_ref()
        .to_string()
    }
}
