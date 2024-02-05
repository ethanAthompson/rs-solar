use crate::planets::mars::Martian;
use super::datetime::KeplerianTime;
use strum::AsRefStr;


/// Longitude representing +- 180 degrees
pub struct Longitude {
    pub east: i32,
    pub west: i32,
}

/// Timezones for keplerian bodies
pub trait Timezone {
    /// determines the timezone offset by hours
    fn track_left(&self, offset: f64) -> String;

    /// determines the number of hours to offset
    fn track_right(&self, offset: Self) -> f64;

    /// hour offset for timezone
    fn offset(&self, body: String);

    /// The prime meridian of the timezone
    fn prime(&self) -> Longitude;

    /// The timezones' code
    fn code(&self) -> String;

    /// The timezones' name
    fn named(&self) -> String;

    /// Converts the timezone into Military Time
    fn tz12(&self, hour: u8) -> String;

    /// Gets the current timezone: MTC, etc..
    fn tz_get(&self, offset: f64) -> KeplerianTime;

    fn tz_offset(&self, input: Martian) -> KeplerianTime;
}



#[derive(Default, Debug, Copy, Clone, AsRefStr)]
pub enum KepTzVariants {
    /* Martian Timezones */
    #[strum(serialize = "MTC-5")]
    MTCn5,
    #[strum(serialize = "MTC-4")]
    MTCn4,
    #[strum(serialize = "MTC-3")]
    MTCn3,
    #[strum(serialize = "MTC-2")]
    MTCn2,
    #[strum(serialize = "MTC-1")]
    MTCn1,
    #[default]
    #[strum(serialize = "MTC")]
    MTC,
    #[strum(serialize = "MTC+1")]
    MTCp1,
    #[strum(serialize = "MTC+2")]
    MTCp2,
    #[strum(serialize = "MTC+3")]
    MTCp3,
    #[strum(serialize = "MTC+4")]
    MTCp4,
    #[strum(serialize = "MTC+5")]
    MTCp5,
    /* Venerian Timezones */
}
