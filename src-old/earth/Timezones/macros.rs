/// A macro that creates datetimes from a string
///
/// @param {$(expr), *} location
///  Represents a repeatable formula -> "", "", "", ...
///
/// @returns {String}
///  This string contains the datetime
///
#[macro_export]
macro_rules! set_datetimes {
    ($($location: expr), *) => {
        $(
            EarthSignal::set_datetime($location.clone());
        )*
    };
}
