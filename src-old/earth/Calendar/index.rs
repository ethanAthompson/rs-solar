#[allow(non_snake_case)]
pub mod EarthCalendar {
    use chrono::prelude::*;
    use icu::datetime::input::{DateInput, IsoTimeInput};
    use icu_calendar::chinese::Chinese;
    use icu_calendar::indian::Indian;
    use icu_calendar::julian::Julian;
    use icu_calendar::{AsCalendar, Gregorian, Iso};

    use crate::main_icu;
    use crate::shared::date::{EarthDate, EarthDateTime};

    /// A signal that represents all supported calendar operations
    ///
    #[derive(Debug)]
    pub struct Signal {}

    impl Default for Signal {
        fn default() -> Self {
            Self {}
        }
    }

    impl Signal {
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
        pub fn earth_maker<T: AsCalendar>(&self, date: icu_calendar::DateTime<T>) -> EarthDateTime {
            // reduce! {InputDate, icu_calendar::DateTime::try_new_gregorian_datetime}
            EarthDateTime {
                date: (
                    date.year().unwrap().number,
                    date.month().unwrap().ordinal,
                    date.day_of_month().unwrap().0,
                ),

                codes: (
                    date.year().unwrap().era.0.to_string(),
                    date.month().unwrap().code.0.to_string(),
                    format!("{:?}", date.iso_weekday().unwrap()),
                ),
                time: (
                    date.hour().unwrap().number(),
                    date.minute().unwrap().number(),
                    date.second().unwrap().number(),
                ),
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
        pub fn to_calendar(&self, input: EarthDate, calendar_type: &'static str) -> EarthDateTime {
            let date: NaiveDate =
                NaiveDate::from_ymd_opt(input.year, input.month, input.day).unwrap();

            let date = EarthDateTime {
                date: (date.year(), date.month(), date.day()),
                ..Default::default()
            };

            match calendar_type {
                "iso" => main_icu!(iso date),
                "chinese" => main_icu!(chinese date),
                "indian" => main_icu!(indian date),
                "julian" => main_icu!(julian date),
                "gregorian" => main_icu! {gregorian date},
                _ => EarthDateTime::default(),
            }
        }

        pub fn debug(&self) {
            println!("{:?}", self);
        }
    }
}
