use chrono::{NaiveDate, Offset, TimeZone, Utc};
use icu::datetime::input::DateInput;
use icu_calendar::DateTime;

pub mod formats;
pub mod index;
pub mod pages;
pub mod tests;

/// 0 Ls
const NORTHWARD_EQUINOX: [u8; 3] = [1, 2, 3];

/// 90 Ls
const NORTHERN_SOLSTICE: [u8; 3] = [4, 5, 6];

/// 180 Ls
const SOUTHWARD_EQUINOX: [u8; 3] = [7, 8, 9];

/// 270 Ls
const SOUTHERN_SOLSTICE: [u8; 3] = [10, 11, 12];
