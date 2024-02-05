use icu_calendar::AnyCalendarKind;
use strum::AsRefStr;


#[derive(Default, Debug, Copy, Clone, AsRefStr)]

pub enum KeplerianEras {   
    /// After Discovery 
    #[strum(serialize = "AD")]
    AD,

    // Before Discovery
    #[strum(serialize = "BD")]
    BD,

    // Invalid Target
    #[default]
    Unknown
}

/// The keplerian date returned
#[derive(Default, Debug)]
pub struct KeplerianDate {
    pub era: KeplerianEras,
    pub year: f64,
    pub month: f64,
    pub day: f64,
    pub ls: f64,
    pub season: String
}

#[derive(Default, Debug)]
pub struct KeplerianTime {
    pub hour: f64,
    pub minute: f64,
    pub second: f64,
    pub offset: String,
    pub hour_type: String,
}

#[derive(Debug)]
pub struct KeplerianDateTime {
    pub date: KeplerianDate,
    pub time: KeplerianTime,
}

#[derive(Debug)]
pub struct KeplerianCalendar {
    pub date: KeplerianDate,
    pub time: KeplerianTime,
    pub cal: AnyCalendarKind
}
