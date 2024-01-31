/// A declarative macro wrapper which acts like a match statement for making datetimes
///
/// @DSL {gregorian | julian | ...} keywords
///  these are the keywords which when specified,
///  are given access to its special scope
///
/// @param {expr} $n
///  This represents an ['EarthDateTime'] struct
///
#[macro_export]
macro_rules! main_icu {
    (gregorian $n: expr) => {
        crate::earth::Calendar::index::EarthCalendar::Signal::default().earth_maker(
            icu_calendar::DateTime::try_new_gregorian_datetime(
                $n.date.0,
                $n.date.1 as u8,
                $n.date.2 as u8,
                $n.time.0,
                $n.time.1,
                $n.time.2,
            )
            .unwrap(),
        )
    };
    (julian $n: expr) => {
        crate::earth::Calendar::index::EarthCalendar::Signal::default().earth_maker(
            icu_calendar::DateTime::try_new_julian_datetime(
                $n.date.0,
                $n.date.1 as u8,
                $n.date.2 as u8,
                $n.time.0,
                $n.time.1,
                $n.time.2,
            )
            .unwrap(),
        )
    };
    (indian $n: expr) => {
        crate::earth::Calendar::index::EarthCalendar::Signal::default().earth_maker(
            icu_calendar::DateTime::try_new_indian_datetime(
                $n.date.0,
                $n.date.1 as u8,
                $n.date.2 as u8,
                $n.time.0,
                $n.time.1,
                $n.time.2,
            )
            .unwrap(),
        )
    };
    (iso $n: expr) => {
        crate::earth::Calendar::index::EarthCalendar::Signal::default().earth_maker(
            icu_calendar::DateTime::try_new_iso_datetime(
                $n.date.0,
                $n.date.1 as u8,
                $n.date.2 as u8,
                $n.time.0,
                $n.time.1,
                $n.time.2,
            )
            .unwrap(),
        )
    };

    (chinese $n: expr) => {
        crate::earth::Calendar::index::EarthCalendar::Signal::default().earth_maker(
            icu_calendar::DateTime::try_new_chinese_datetime_with_calendar(
                $n.date.0,
                $n.date.1 as u8,
                $n.date.2 as u8,
                $n.time.0,
                $n.time.1,
                $n.time.2,
                icu_calendar::chinese::Chinese::default(),
            )
            .unwrap(),
        )
    };
}
