use std::str::FromStr;

use chrono::prelude::*;
use icu::datetime::input::{DateInput, IsoTimeInput};
use icu_calendar::chinese::Chinese;
use icu_calendar::ethiopian::EthiopianEraStyle;
use icu_calendar::types::Era;
use icu_calendar::AsCalendar;

use crate::{get_icu, transport_icu};
use crate::shared::datetime::{EarthCodes, EarthDate, EarthDateTime, EarthTime};

/// Constructs an earth datetime from calendar type
///
/// @generic {AsCalendar} T
///  This generic represets a type that contains a calendar
///
/// @param {Date<T>} date
///  This represents a Date for a given calendar
///
/// @returns {EarthDateTime}
///  The earth date is returned with the given calendar year/month/day
///  methods into fields
pub fn earth_maker<T: AsCalendar>(date: icu_calendar::DateTime<T>) -> EarthDateTime {
    // reduce! {InputDate, icu_calendar::DateTime::try_new_gregorian_datetime}
    EarthDateTime {
        date: EarthDate {
            year: date.year().unwrap().number,
            month: date.month().unwrap().ordinal,
            day: date.day_of_month().unwrap().0,
        },
        codes: EarthCodes {
            year: date.year().unwrap().era.0.to_string(),
            month: date.month().unwrap().code.0.to_string(),
            day: format!("{:?}", date.iso_weekday().unwrap()),
        },
        time: EarthTime {
            hour: date.hour().unwrap().number(),
            minute: date.minute().unwrap().number(),
            second: date.second().unwrap().number(),
        },
    }
}

/// Converts an earth datetime to a specific calendar
///
/// @param {&self}
///  Allows daisy param chaining
///
/// @param {EarthDateTime} input
///  Represents an earthdate
///
/// @param {&'static str} calendar_type
///  Represents the type of calendar you want
///
/// @returns {EarthDateTime}
///  Returns an earth that which can be displayed seperately
///
pub fn construct_calendar(input: EarthDate, calendar_type: &'static str) -> EarthDateTime {
    let date: NaiveDate = NaiveDate::from_ymd_opt(input.year, input.month, input.day).unwrap();

    let date = EarthDateTime {
        date: EarthDate{year: date.year(), month: date.month(), day: date.day()},
        ..Default::default()
    };

    match calendar_type {
        "gregorian" => get_icu! {gregorian date},
        "julian" => get_icu!(julian date),
        "indian" => get_icu!(indian date),
        "iso" => get_icu!(iso date),
        /* China  */
        "chinese" => get_icu!(chinese date),
        "roc" => get_icu!(roc date),
        /* Ethiopian Eras */
        "ethiopian_alem" => get_icu!(ethiopian date, EthiopianEraStyle::AmeteAlem),
        "ethiopian_mihret" => get_icu!(ethiopian date, EthiopianEraStyle::AmeteMihret),
        /* The 5 common japanese eras */
        /* 40+ japanese eras: https://en.wikipedia.org/wiki/Japanese_era_name#:~:text=For%20example%2C%20S55%20means%20Sh%C5%8Dwa,era%20(%E5%B9%B3%E6%88%9031%E5%B9%B4). */
        "japanese_heisei" => get_icu!(japanese date, Era::from_str("heisei").unwrap()),
        "japanese_reiwa" => get_icu!(japanese date, Era::from_str("reiwa").unwrap()),
        "japanese_meiji" => get_icu!(japanese date, Era::from_str("meiji").unwrap()),
        "japanese_taisho" => get_icu!(japanese date, Era::from_str("taisho").unwrap()),
        "japanese_showa" => get_icu!(japanese date, Era::from_str("showa").unwrap()),
        "japanese_extended" => get_icu!(japanese_extended date, Era::from_str("kansei-1789").unwrap()),
        "persian" => get_icu!(persian date),
        "coptic" => get_icu!(coptic date),
        "hewbrew" => get_icu!(hebrew date),
        "buddhist" => get_icu!(buddhist date),
        /* Islamic  */ 
        "islamic_civil" => get_icu!(islamic_civil date),
        "islamic_observational" => get_icu!(islamic_observational date),
        "islamic_tabular" => get_icu!(islamic_tabular date),
        "islamic_umm_al_qura" => get_icu!(islamic_umm_al_qura date),
        _ => EarthDateTime::default(),

    }
}


pub fn to_calendar<T: AsCalendar>(input: EarthDate, calendar_type: &'static str, conversion_type: T) -> EarthDateTime {
    let date: NaiveDate = NaiveDate::from_ymd_opt(input.year, input.month, input.day).unwrap();

    let date = EarthDateTime {
        date: EarthDate{year: date.year(), month: date.month(), day: date.day()},
        ..Default::default()
    };

    match calendar_type {
        "gregorian" => transport_icu! {gregorian date, conversion_type},
        "julian" => transport_icu!(julian date, conversion_type),
        "indian" => transport_icu!(indian date, conversion_type),
        "iso" => transport_icu!(iso date, conversion_type),
        /* China  */
        "chinese" => transport_icu!(chinese date, conversion_type),
        "roc" => transport_icu!(roc date, conversion_type),
        /* Ethiopian Eras */
        "ethiopian_alem" => transport_icu!(ethiopian date, EthiopianEraStyle::AmeteAlem, conversion_type),
        "ethiopian_mihret" => transport_icu!(ethiopian date, EthiopianEraStyle::AmeteMihret, conversion_type),
        /* The 5 common japanese eras */
        "japanese_heisei" => transport_icu!(japanese date, Era::from_str("heisei").unwrap(), conversion_type),
        "japanese_reiwa" => transport_icu!(japanese date, Era::from_str("reiwa").unwrap(), conversion_type),
        "japanese_meiji" => transport_icu!(japanese date, Era::from_str("meiji").unwrap(), conversion_type),
        "japanese_taisho" => transport_icu!(japanese date, Era::from_str("taisho").unwrap(), conversion_type),
        "japanese_showa" => transport_icu!(japanese date, Era::from_str("showa").unwrap(), conversion_type),
        "japanese_extended" => transport_icu!(japanese_extended date, Era::from_str("kansei-1789").unwrap(),conversion_type),
        "persian" => transport_icu!(persian date, conversion_type),
        "coptic" => transport_icu!(coptic date, conversion_type),
        "hewbrew" => transport_icu!(hebrew date, conversion_type),
        "buddhist" => transport_icu!(buddhist date, conversion_type),
        /* Islamic  */ 
        "islamic_civil" => transport_icu!(islamic_civil date, conversion_type),
        "islamic_observational" => transport_icu!(islamic_observational date, conversion_type),
        "islamic_tabular" => transport_icu!(islamic_tabular date, conversion_type),
        "islamic_umm_al_qura" => transport_icu!(islamic_umm_al_qura date, conversion_type),
        _ => EarthDateTime::default()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use icu_calendar::{chinese::Chinese, islamic::IslamicCivil, julian::Julian, Gregorian};

    use crate::shared::datetime::EarthDate;

    use super::{construct_calendar, to_calendar};

    #[test]
    fn julian2chinese_jd_epoch() {
        let input = EarthDate::julian_epoch();
        let calendar_type = "julian";
        let conversion_type = Chinese::default();
        let cal = to_calendar(input, calendar_type, conversion_type);
        println!("{:?}", cal);
    }

    #[test]
    fn gregorian2chinese_jd_epoch() {
        let input = EarthDate::julian_epoch();
        let calendar_type = "gregorian";
        let conversion_type = Chinese::default();
        let cal = to_calendar(input, calendar_type, conversion_type);
        println!("{:?}", cal);
    }

    #[test]
    fn julian2chinese_now() {
        let input = EarthDate::now();
        let calendar_type = "julian";
        let conversion_type = Chinese::default();
        let cal = to_calendar(input, calendar_type, conversion_type);
        println!("{:?}", cal);
    }

    #[test]
    fn julian_now() {
        let input = EarthDate::now();
        let calendar_type = "julian";
        let cal = construct_calendar(input, calendar_type);
        println!("{:?}", cal);
    }

    #[test]
    fn gregorian_now() {
        let input = EarthDate::now();
        let calendar_type: &str = "gregorian";
        let cal = construct_calendar(input, calendar_type);
        println!("{:?}", cal);
    }

    #[test]
    fn chinese_now() {
        let input = EarthDate::now();
        let calendar_type = "chinese";
        let cal = construct_calendar(input, calendar_type);
        println!("{:?}", cal);
    }

    #[test]
    fn julian2chinese2gregorian() {
        let input = EarthDate::now();
        let cal = construct_calendar(input, "julian");
        let cal_1 = to_calendar(cal.date, "julian", Chinese::default());
        let cal_2 = to_calendar(cal_1.date, "chinese", Gregorian::default());
        println!("{:?}", cal_2);
    }

    #[test]
    fn gregorian2chinese2julian() {
        let input = EarthDate::now();
        let cal = construct_calendar(input, "gregorian");
        let cal_1 = to_calendar(cal.date, "gregorian", Chinese::default());
        let cal_2 = to_calendar(cal_1.date, "chinese", Julian::default());
        println!("{:?}", cal_2);
    }


    #[test]
    fn chinese2julian2gregorian() {
        let input = EarthDate::now();
        let cal = construct_calendar(input, "chinese");
        let cal_1 = to_calendar(cal.date, "chinese", Julian::default());
        let cal_2 = to_calendar(cal_1.date, "julian", Gregorian::default());
        println!("{:?}", cal_2);
    }

}
