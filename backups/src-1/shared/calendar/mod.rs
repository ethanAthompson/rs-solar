/**
Formula from:
https://thecynster.home.blog/2019/11/08/calculating-the-julian-date-and-j2000/
https://scienceworld.wolfram.com/astronomy/JulianDate.html
https://www.militarytime.us/what-is-805-pm-military-time/
https://www.ontheclock.com/convert-hours-minutes-to-decimal-hours.aspx

*/


/// Common julian calculations for keplerian stuff..
pub mod julian;

/// The standard macro that contains abstracted Datetime W/ Calendars
pub mod standard;

/// The builder which abstracts the standard macro
pub mod builder;


