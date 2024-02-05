pub mod datetime;
pub mod fromstr;
pub mod mtc;
pub mod body;

use strum::AsRefStr;

/// The 11 different coordinated timezones
#[derive(Default, Debug, Copy, Clone, AsRefStr)]
pub enum Martian {
    #[strum(serialize = "MTC-5")]
    MTCn5,
    #[strum(serialize = "MTC-4")]
    MTCn4,
    #[strum(serialize = "MTC-3")]
    MTCn3,
    #[strum(serialize = "MTC-2")]
    MTCn2,
    #[strum(serialize = "MTC-1")]
    MTCn1,
    #[default]
    #[strum(serialize = "MTC")]
    MTC,
    #[strum(serialize = "MTC+1")]
    MTCp1,
    #[strum(serialize = "MTC+2")]
    MTCp2,
    #[strum(serialize = "MTC+3")]
    MTCp3,
    #[strum(serialize = "MTC+4")]
    MTCp4,
    #[strum(serialize = "MTC+5")]
    MTCp5,
}
