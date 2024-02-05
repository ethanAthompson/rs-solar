use strum::{AsRefStr, Display};

use crate::shared::math::cosmic::SolarLongitude;

/// The seasons in which all keplerian bodies follow
#[derive(AsRefStr, Debug, Display, Default)]
pub enum Seasons {
    /// March 19th
    #[strum(serialize = "Vernal Equinox")]
    VernalEquinox,

    /// July 6 
    #[strum(serialize = "Aphelion")]
    Aphelion,

    /// June 20th -> June 21st
    #[strum(serialize = "Summer Solstice")]
    SummerSolstice,

    /// September 22
    #[strum(serialize = "Autumn Equinox")]
    AutumnEquinox,

    /// January 3rd
    #[strum(serialize = "Perihelion")]
    Perihelion,

    /// December 21st -> December 22nd
    #[strum(serialize = "Winter Solstice")]
    WinterSolstice,
    #[strum(serialize = "N/A")]
    #[default]
    Unknown,
}

impl Seasons {
    pub fn in_season(&self, ls: SolarLongitude) -> String {
        match ls as u32 {
            71 => Self::Aphelion,
            251 => Self::Perihelion,
            0..=90 => Self::VernalEquinox,
            90..=180 => Self::SummerSolstice,
            180..=270 => Self::AutumnEquinox,
            270..=360 => Self::WinterSolstice,
            _ => Self::Unknown
        }
        .as_ref()
        .to_string()
    }
}

#[cfg(test)]
pub mod tests {
    use super::Seasons;

    #[test]
    pub fn season_work() {

        let hay = 0..=360;

        for needle in hay {
            let tmp = Seasons::default().in_season(needle as f64);
            println!("{needle}: {tmp}");
        } 
     
    }
}
