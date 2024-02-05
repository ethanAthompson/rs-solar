use std::{str::FromStr, time::UNIX_EPOCH};
use strum::AsRefStr;
use crate::shared::{
    datetime::MilitaryTime, kepler::{body::KeplerianBody, datetime::KeplerianTime, timezone::{Longitude, Timezone}}, sums::planets::Earth
};

use super::Martian;

impl Timezone for Martian {
    fn code(&self) -> String {
        match self {
            Self::MTCn5 => "AMT",
            Self::MTCn4 => "OT",
            Self::MTCn3 => "TT",
            Self::MTCn2 => "MT",
            Self::MTCn1 => "AGT",
            Self::MTC => "NT",
            Self::MTCp1 => "ABT",
            Self::MTCp2 => "HT",
            Self::MTCp3 => "UT",
            Self::MTCp4 => "ET",
            Self::MTCp5 => "ACT",
        }
        .to_string()
    }

    fn named(&self) -> String {
        match self {
            Self::MTCn5 => "Amazonis Time",
            Self::MTCn4 => "Olympus Time",
            Self::MTCn3 => "Tharsis Time",
            Self::MTCn2 => "Marineris Time",
            Self::MTCn1 => "Argyre Time",
            Self::MTC => "Noachis Time",
            Self::MTCp1 => "Arabia Time",
            Self::MTCp2 => "Hellas Time",
            Self::MTCp3 => "Utopia Time",
            Self::MTCp4 => "Elysium Time",
            Self::MTCp5 => "Arcadia Time",
        }
        .to_string()
    }

    fn offset(&self, body: String) {
        todo!()
    }

    fn prime(&self) -> Longitude {
        match self {
            Self::MTCn5 => Longitude {
                east: -180,
                west: -162,
            },
            Self::MTCn4 => Longitude {
                east: -162,
                west: -126,
            },
            Self::MTCn3 => Longitude {
                east: -126,
                west: -90,
            },
            Self::MTCn2 => Longitude {
                east: -90,
                west: -54,
            },
            Self::MTCn1 => Longitude {
                east: -54,
                west: -18,
            },
            Self::MTC => Longitude {
                east: -18,
                west: 18,
            },
            Self::MTCp1 => Longitude { east: 18, west: 54 },
            Self::MTCp2 => Longitude { east: 54, west: 90 },
            Self::MTCp3 => Longitude {
                east: 90,
                west: 126,
            },
            Self::MTCp4 => Longitude {
                east: 126,
                west: 162,
            },
            Self::MTCp5 => Longitude {
                east: 162,
                west: 180,
            },
        }
    }

    fn track_left(&self, offset: f64) -> String {
        match offset {
            -12.5 => Self::MTCn5,
            -10.0 => Self::MTCn4,
            -7.5 => Self::MTCn3,
            -5.0 => Self::MTCn2,
            -2.5 => Self::MTCn1,
            0.0 => Self::MTC,
            2.5 => Self::MTCp1,
            5.0 => Self::MTCp2,
            7.5 => Self::MTCp3,
            10.0 => Self::MTCp4,
            12.5 => Self::MTCp5,
            _ => todo!(),
        }
        .as_ref()
        .to_string()
    }    
    
    fn track_right(&self, offset: Self) -> f64 {
        match offset {
               // West
               Self::MTCn5 => -2.5 * 5.0,
               Self::MTCn4 => -2.5 * 4.0,
               Self::MTCn3 => -2.5 * 3.0,
               Self::MTCn2 => -2.5 * 2.0,
               Self::MTCn1 => -2.5 * 1.0,
               // Center
               Self::MTC => 0.0,
               // East
               Self::MTCp1 => 2.5,
               Self::MTCp2 => 2.5 * 2.0,
               Self::MTCp3 => 2.5 * 3.0,
               Self::MTCp4 => 2.5 * 4.0,
               Self::MTCp5 => 2.5 * 5.0,
            _ => todo!(),
        }

    }

    fn tz12(&self, hour: u8) -> String {
        match hour {
            0..=11 => MilitaryTime::AM,
            12..=24 => MilitaryTime::PM,
            _ => todo!(),
        }
        .as_ref()
        .to_string()
    }

    fn tz_get(&self, offset: f64) -> KeplerianTime {
        let millis = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("error")
            .as_millis() as f64;
        let jd_ut = 2_440_587.5 + (millis / 86_400_000.0);
        let jd_tt = jd_ut + (37.0 + 32.184) / Earth.day_in_seconds(true);
        let jd2000_t = jd_tt - 2_451_545.0;

        // INFO! maybe adjust for other plants
        // keplerian_body_length_of_day (in seconds) / earth_julian_length_of_day (in seconds)
        let mars_earth_ratio = 1.027491252;
        let midday = 44_796.0;
        let alignment = 0.00096;
        let earth_day = Earth.day_in_seconds(true);

        // 9000 seconds = 2.5 hours
        let msd = ((jd2000_t - 4.5) / mars_earth_ratio) + midday - alignment;
        let mtc = (24.0 * msd) % 24.0;
        // println!("MSD: {:?}", msd);
        // println!("MTC: {:?}", mtc);

        let fh = msd.fract(); // Fractional hour
        let mut hour = (24.0 * fh).floor();
        let fm = (24.0 * fh).fract(); // Fractional minute
        let minute = (60.0 * fm).floor();
        let second = 60.0 * (60.0 * fm).fract();
        let hour12 = (hour + offset) as u8;
        let offset = self.track_left(offset);
        let hour_type = self.tz12(hour12);

        // calibrates hour to mt
        match hour as u8 > 24 {
            true => {
                hour = 0.0;
            }
            false => (),
        }

        return KeplerianTime {
            hour: hour12 as f64,
            minute,
            second,
            offset,
            hour_type,
        };
    }

        fn tz_offset(&self, input: Martian) -> KeplerianTime {
            self.tz_get(self.track_right(input))
        }
}
