use std::str::FromStr;

use chrono::{Datelike, Timelike};
use icu_calendar::AnyCalendarKind;

use crate::{planets::mars::Martian, shared::{calendar::julian::get_jd, kepler::{body::KeplerianBody, datetime::KeplerianCalendar, timezone::Timezone}, sums::planets::Mars}};
use crate::shared::datetime::DateTime;

impl DateTime for Mars {
    fn now(calendar: AnyCalendarKind, input: String) -> KeplerianCalendar {
        let now = chrono::Utc::now();
        let greg = get_jd(
            now.year(),
            now.month() as i32,
            now.day() as i32,
            (now.hour() + now.minute() / 100) as f64,
        );
      
        let date = Mars.to_date(greg);
        let time = Martian::default().tz_offset(Martian::from_str(&input).unwrap());


        KeplerianCalendar { date: date, time: time, cal: calendar }
    }

    fn nowstr(calendar: AnyCalendarKind, input: String) -> String {
        let now = chrono::Utc::now();
        let greg = get_jd(
            now.year(),
            now.month() as i32,
            now.day() as i32,
            (now.hour() + now.minute() / 100) as f64,
        );
      
        let date = Mars.to_date(greg);
        let time = Martian::default().tz_offset(Martian::from_str(&input).unwrap());

        format!(
            "{:?} {:?}/{:?}/{:?}, Ls {:?}, {:?}:{:?}:{:?} {:?}, {:?}",
            calendar,
            date.year as u32,
            date.month as u32,
            date.day as u32,
            date.ls as u32,
            time.hour as u32,
            time.minute as u32,
            time.second as u32,
            time.offset,
            time.hour_type 
        )
    }
}


#[cfg(test)] 
mod tests {
    use icu_calendar::AnyCalendarKind;

    use crate::shared::{datetime::DateTime, sums::planets::Mars};

    #[test]
    fn mars_nowstr() {
        let str = Mars::nowstr(AnyCalendarKind::Gregorian, "MTC+5".to_string());
        println!("{str}");
    }
    
    #[test]
    fn mars_now() {
        let now = Mars::now(AnyCalendarKind::Chinese, "MTC".to_string());
        println!("{:?}", now);
    }
}