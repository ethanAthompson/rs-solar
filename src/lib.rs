#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    // trivial_casts,
    // trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    missing_docs,
    unused_import_braces,
    unused_qualifications
)]

//! # Rust Solar Information
//!
//!
//! Rust Solar is a full fledged library to help programmers, utilize calendar, date, and time for celestial bodies
//! in their application.
//!
//! # References Used
//! (Latex Editor)[https://latexeditor.lagrida.com/]
//! (Desmos Scientific Calculator)[https://www.desmos.com/scientific]
//! (Mars Html Calendar)[https://www-mars.lmd.jussieu.fr/mars/time/martian_time.html]
//! (Design of an Elliptical Transfer Orbit)[https://space.stackexchange.com/questions/23128/design-of-an-elliptical-transfer-orbit/23130#23130]
//! (Kepler Orbits)[https://www.bogan.ca/orbits/kepler/orbteqtn.html]
//! (Hyperbolic Kepler's Equation)[https://control.asu.edu/Classes/MAE462/462Lecture05.pdf]
//! (Semi Major Axises and Semi Minor Axises)[https://en.wikipedia.org/wiki/Semi-major_and_semi-minor_axes]
//! (Geometry of Orbits)[https://www.bogan.ca/orbits/geometry.html]
//! (Julian Date Converter)[https://aa.usno.navy.mil/data/JulianDate]
//! - (Semi Latus Rectum of Parabola or Hyperbola)[https://www.orbiter-forum.com/threads/how-calculate-semi-latus-rectum-of-parabola-or-hyperbola.40315/]
//!     > Semi-Latus Rectum equation by Kolodez 
//! (Orbital Periods)[https://upload.wikimedia.org/wikipedia/commons/thumb/b/be/Solar_system_orbital_period_vs_semimajor_axis.svg/800px-Solar_system_orbital_period_vs_semimajor_axis.svg.png]
//! (Astronomy Calculations)[https://docs.google.com/spreadsheets/d/1rwc2mVxyHuUEou_hxnG6kzl24XdqqmIAS5_1nJDpJ6o/edit#gid=1479831395]
//! 
//! 
//! (Use this, don't show to library)[https://github.com/ethanAthompson/rs-solar/blob/main/backups/src-1/planets/mars/body.rs#L88]
//! 
//! 
//! 
//! 
//! # Features
//!
//!  - `asteroids` : Brings asteroid support
//!  - `planets`: Brings planet support
//!  - `exo-planets`: Brings exo-planet support
//!  - `comets`: Brings comet support
//!  - `moons`: Brings moon support
//!
//!    To use features maybe?
//!    (https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it)
//! 
//! ** Warning! **, Please know this may not be 100% accurate as these data rely on 30% hypothetical data.
//!
//!

/// This module contains calculations for supported planets
pub mod planets;

/// This module contains common kepler data
pub mod kepler;

/// This module contains common conversion data
pub mod conversions;

/// This module contains common orbital data
pub mod orbit;

/// This module contains anomalic equations
pub mod anomaly;

/// This module contains julian operations
pub mod julian;

/// This module contains utilities that I wouldn't know where to put elsewhere.
pub mod utils;