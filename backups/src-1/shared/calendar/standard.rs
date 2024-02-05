/**
A declarative macro wrapper which acts like a match statement for making datetimes

@DSL {gregorian | julian | ...} keywords
 these are the keywords which when specified,
 are given access to its special scope

@param {expr} $n
 This represents an ['EarthDateTime'] struct

*/
#[macro_export]
macro_rules! get_icu {
    (gregorian $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_gregorian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (julian $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_julian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (indian $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_indian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (iso $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_iso_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (chinese $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_chinese_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::chinese::Chinese::default(),
            )
            .unwrap(),
        )
    };
    (roc $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_roc_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (ethiopian $n: expr, $era: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_ethiopian_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (japanese $n: expr, $era: expr ) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_japanese_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::japanese::Japanese::default(),
            )
            .unwrap(),
        )
    };
    (persian $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_persian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (coptic $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_coptic_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (hebrew $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_hebrew_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::hebrew::Hebrew::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (buddhist $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_buddhist_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap(),
        )
    };
    (islamic_civil $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_islamic_civil_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicCivil::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (islamic_observational $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_observational_islamic_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicObservational::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (islamic_tabular $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_islamic_tabular_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicTabular::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (islamic_umm_al_qura $n: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_ummalqura_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicUmmAlQura::new_always_calculating(),
            )
            .unwrap(),
        )
    };
    (japanese_extended $n: expr, $era: expr) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_japanese_extended_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::japanese::JapaneseExtended::default(),
            )
            .unwrap(),
        )
    };
}

/** Calendar Transportation
 *
 * Common calendar transportation
 * AnyCalender -> Gregorian
 * AnyCalendar -> ...
 */

/**
A declarative macro wrapper which acts like a match statement,
for transporting datetimes to gregorian date

@DSL {gregorian | julian | ...} keywords
 these are the keywords which when specified,
 are given access to its special scope

@param {expr} $n
 This represents an ['EarthDateTime'] struct

@param {ty} $cal
 This represents a certain calendar to convert to.

@data
    Generic Calendar? $n calendar_type: AnyCalendar,
*/
#[macro_export]
macro_rules! transport_icu {
    (gregorian $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_gregorian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (julian $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_julian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (indian $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_indian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (iso $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_iso_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (chinese $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_chinese_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::chinese::Chinese::default(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (roc $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_roc_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (ethiopian $n: expr, $era: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_ethiopian_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (japanese $n: expr, $era: expr, $cal: ident ) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_japanese_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::japanese::Japanese::default(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (persian $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_persian_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (coptic $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_coptic_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (hebrew $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_hebrew_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::hebrew::Hebrew::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (buddhist $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_buddhist_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (islamic_civil $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_islamic_civil_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicCivil::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (islamic_observational $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_observational_islamic_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicObservational::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (islamic_tabular $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_islamic_tabular_datetime_with_calendar(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicTabular::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (islamic_umm_al_qura $n: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_ummalqura_datetime(
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::islamic::IslamicUmmAlQura::new_always_calculating(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
    (japanese_extended $n: expr, $era: expr, $cal: ident) => {
        crate::shared::calendar::builder::earth_maker(
            icu_calendar::DateTime::try_new_japanese_extended_datetime(
                $era,
                $n.date.year,
                $n.date.month as u8,
                $n.date.day as u8,
                $n.time.hour,
                $n.time.minute,
                $n.time.second,
                icu_calendar::japanese::JapaneseExtended::default(),
            )
            .unwrap()
            .to_calendar($cal),
        )
    };
}
