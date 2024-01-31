#[cfg(test)]
mod tests {
    use crate::earth::Timezones::index::EarthSignal;
    use crate::set_datetimes;
    /// cargo test does_all_timezones -- --nocapture
    #[test]
    fn does_all_timezones() {
        let list = EarthSignal::all_timezones(Vec::new());
        println!("{:?}", list);
    }

    /// cargo test does_set_datetime -- --nocapture
    #[test]
    fn does_set_datetime() {
        let dt = EarthSignal::set_datetime("America/New_York".to_string());
        println!("DT : {:?}, {:?}", dt[0].0, dt[0].1);
    }

    /// cargo test does_set_datetimes -- --nocapture
    #[test]
    fn does_set_datetimes() {
        set_datetimes! {
            "Japan".to_string(), "EST".to_string(), "APPLE".to_string()
        };
    }
}
