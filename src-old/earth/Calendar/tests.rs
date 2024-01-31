#[cfg(test)]
mod tests {
    use crate::earth::Calendar::index::EarthCalendar;
    use crate::shared::date::EarthDate;

    /// cargo test does_to_calendar -- --nocapture
    #[test]
    fn does_to_calendar() {
        let date = EarthDate {
            year: 2024,
            month: 1,
            day: 13,
        };

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "julian")
            .debug();

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "gregorian")
            .debug();

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "chinese")
            .debug();

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "iso")
            .debug();

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "indian")
            .debug();
    }

    /// cargo test does_prove_to_calendar -- --nocapture
    #[test]
    fn does_prove_to_calendar() {
        let date = EarthDate {
            year: 2024,
            month: 1,
            day: 13,
        };

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "julian")
            .to_gregorian("julian")
            .debug();

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "gregorian")
            .to_gregorian("gregorian")
            .debug();

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "chinese")
            .to_gregorian("chinese")
            .debug();

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "iso")
            .to_gregorian("iso")
            .debug();

        EarthCalendar::Signal::default()
            .to_calendar(date.clone(), "indian")
            .to_gregorian("indian")
            .debug();
    }
}
