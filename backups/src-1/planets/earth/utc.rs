use std::str::FromStr;

use chrono::{DateTime, Local, Utc};
use chrono_tz::TZ_VARIANTS;

/// Returns all timezones from chrono database
pub fn all_timezones(mut list: Vec<String>) -> Vec<String> {
    for variant in TZ_VARIANTS.iter() {
        list.push(variant.to_string())
    }

    list
}

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

#[macro_export]
macro_rules! set_datetimes {
    ($($location: expr), *) => {
        $(
            super::set_datetime($location.clone());
        )*
    };
}

#[cfg(test)]
mod tests {
    use crate::planets::earth::utc::{all_timezones, set_datetime};

    #[test]
    fn does_all_timezones() {
        let list = all_timezones(Vec::new());
        println!("{:?}", list);
    }

    #[test]
    fn does_set_datetime() {
        let dt = set_datetime("America/New_York".to_string());
        println!("DT : {:?}, {:?}", dt[0].0, dt[0].1);
    }

    #[test]
    fn does_set_datetimes() {
        set_datetimes! {
            "Japan".to_string(), "EST".to_string(), "APPLE".to_string()
        };
    }
}
