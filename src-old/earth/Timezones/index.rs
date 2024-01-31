#[allow(non_snake_case)]
pub mod EarthSignal {
    use chrono::{DateTime, Local, Utc};
    use chrono_tz::TZ_VARIANTS;
    use std::str::FromStr;

    /// Returns all timezones from chrono database
    ///
    /// @param {Vec<String>} mut list
    ///  A mutable list
    ///
    /// @returns {Vec<String>} list
    ///   returns a vector of strings
    ///
    pub fn all_timezones(mut list: Vec<String>) -> Vec<String> {
        for variant in TZ_VARIANTS.iter() {
            list.push(variant.to_string())
        }

        list
    }

    /// Make DateTime
    ///
    /// @param {String} input
    ///   A string that is matched throughout the chrono database
    ///
    /// @returns {String} time.to_string()
    ///  The datetime after chrono conversion
    ///
    pub fn set_datetime(input: String) -> Vec<(String, String)> {
        if let Ok(time) = chrono_tz::Tz::from_str(input.as_str()) {
            let time = DateTime::with_timezone(&Utc::now(), &time).format("%Y/%m/%d %r %Z");
            vec![(input, time.to_string())]
        } else {
            let name = Local::now()
                .with_timezone(&Local::now().timezone())
                .to_string();
            let time = Local::now().format("%Y/%m/%d %r %Z");

            vec![(name, time.to_string())]
        }
    }
}
