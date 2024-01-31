pub const JD2NOON: f64 = 2451545.0;

pub struct EarthDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}
// this includes the julian stuff too
// make folder for the other macros too

/// Returns a JD200 date
/// J2000 = JD - ['JD2NOON']
pub fn days_since_j2000(year: i32, month: i32, day: i32, offset: f64) -> f64 {
    let j2 = get_jd(year, month, day, offset) - JD2NOON;

    println!("{:?} Days since j2000", j2);

    j2
}

/// converts julian date to gregorian date
pub fn jd2greg(jd: f64) {
    if let Ok(date_time) = julian_day_converter::julian_day_to_datetime(jd) {
        println!("The date time is {}", date_time.format("%Y-%m-%d %H:%M:%S"));
    }
}

/// Formula from:
/// https://thecynster.home.blog/2019/11/08/calculating-the-julian-date-and-j2000/
/// https://scienceworld.wolfram.com/astronomy/JulianDate.html
/// https://www.militarytime.us/what-is-805-pm-military-time/
/// https://www.ontheclock.com/convert-hours-minutes-to-decimal-hours.aspx
///
/// @param {f32} offset
///  your offset is decimal hours in military time: ex; 20.5 is 20:05pm is 8:05pm
///
pub fn get_jd(year: i32, month: i32, day: i32, offset: f64) -> f64 {
    let jd = (367 as f64 * year as f64
        - (7 * (year + (month + 9) / 12) / 4) as f64
        - (((3 * (year + (month - 9) / 7) / 100) + 1) / 4) as f64
        + (275 * month / 9) as f64
        + day as f64
        + 1721028.5 as f64
        + offset as f64 / 24.0) as f64;

    println!("Julian date: {:?}", jd);

    jd
}

// pub fn convert_date(calender: icu_calendar)
