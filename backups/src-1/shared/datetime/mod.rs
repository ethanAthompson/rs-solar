use chrono::Datelike;
use icu_calendar::{
    buddhist::Buddhist,
    chinese::Chinese,
    ethiopian::Ethiopian,
    hebrew::Hebrew,
    islamic::{IslamicCivil, IslamicObservational, IslamicTabular, IslamicUmmAlQura},
    japanese::{Japanese, JapaneseExtended},
    persian::Persian,
    provider::JapaneseErasV1,
    roc::Roc,
    AnyCalendarKind, AsCalendar,
};
use strum::AsRefStr;

use super::{calendar::julian::{get_jd, jd2greg_named, JD2NOON}, kepler::datetime::KeplerianCalendar, sums::planets::Earth};

/// Mars Coordinated Time (MTC)
pub type CoordinatedMarsTime = f64;

/// Hailey's Coordinated Time (HTC)
pub type CoordinatedHaileyTime = f64;

#[derive(Debug, AsRefStr)]
pub enum MilitaryTime {
    #[strum(serialize = "AM")]
    /// 0000 -> 1100 AM
    AM,
    #[strum(serialize = "PM")]
    /// 1200 -> 2300 PM
    PM,
}

pub trait DateTime {
    fn now(calendar: AnyCalendarKind, input: String) -> KeplerianCalendar;
    fn nowstr(calendar: AnyCalendarKind, input: String) -> String;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EarthDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl EarthDate {
    /// Returns the current date now
    pub fn now() -> Self {
        let now = chrono::Utc::now();
        Self {
            year: now.year(),
            month: now.month(),
            day: now.day(),
        }
    }

    pub fn julian_epoch() -> Self {
        let epoch = jd2greg_named(JD2NOON);
        
        Self {
            year: epoch.date.year,
            month: epoch.date.month,
            day: epoch.date.day,
        }  
    }
}

#[derive(Debug, Default)]

pub struct EarthCodes {
    pub year: String,
    pub month: String,
    pub day: String,
}
#[derive(Debug, Default)]

pub struct EarthTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Debug, Default)]
pub struct EarthDateTime {
    pub date: EarthDate,
    pub codes: EarthCodes,
    pub time: EarthTime,
}

pub fn get_cc<T: AsCalendar>(d: T) -> T {
    d
}

#[macro_export]
macro_rules! getcal {
    ($cal: ty) => {};
}

fn example() {
    // icu_calendar::DateTime::try_new_japanese_extended_datetime(era, year, month, day, hour, minute, second, japanext_calendar)

    let fin = get_cc(Chinese::new_always_calculating());

    icu_calendar::DateTime::try_new_gregorian_datetime(2024, 5, 3, 12, 4, 30)
        .unwrap()
        .to_calendar(fin);

    getcal!(Chinese::new_always_calculating());
    getcal!(Gregorian::default());
}
