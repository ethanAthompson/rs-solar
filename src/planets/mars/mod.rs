use chrono::DateTime;
use icu::datetime::input::DateInput;
use icu_calendar::{gregorian::GregorianDateInner, Gregorian};

use crate::shared::{
    earth_calendar::{get_jd, EarthDate},
    keplerian_body::*,
};
use std::f64::consts::PI;

/// n = -, p = +
pub enum Martian {
    MTCn5,
    MTCn4,
    MTCn3,
    MTCn2,
    MTCn1,
    MTC,
    MTCp1,
    MTCp2,
    MTCp3,
    MTCp4,
    MTCp5,
}
// https://www-mars.lmd.jussieu.fr/mars/time/martian_time.html
impl KeplerianBody for Mars {
    fn semimajor(&self) -> AstronicalUnit {
        1.00000
    }

    fn semiminor(&self) -> AstronicalUnit {
        0.99986
    }
    fn orbital_eccentricity(&self) -> Eccentricity {
        0.0934
    }

    fn day_in_seconds(&self, is_solar: bool) -> f64 {
        match is_solar {
            true => 88_775.245,
            false => 88_642.663,
        }
    }

    fn year_in_days(&self, is_leap: bool) -> f64 {
        match is_leap {
            true => 669.6,
            false => 668.6,
        }
    }

    fn is_leapyear(&self) -> bool {
        false
    }

    /// 8th month of mars is the date of the perihelion
    fn perihelion_date(&self) -> f64 {
        let month_start = 468.5;
        let month_end = 514.6;
        let ls_start = 240.0;
        let ls_end = 270.0;
        let perihelion_ls = 251.0;

        let avg_days = month_end - month_start;
        let avg_ls = ls_end - ls_start;
        let until_peri = perihelion_ls - ls_start;
        let peri_day = avg_days / avg_ls;

        let date = (peri_day * until_peri) + month_start;

        // println!("{date}");

        date
    }

    fn perihelion_time(&self) -> f64 {
        // 2*Pi*(1-Ls(perihelion)/360); Ls(perihelion)=250.99
        println!(
            "Perihelion Time: {}",
            2.0 * PI * (1.0 - self.seasons_ls(Seasons::Perihelion)[0] as f64 / 360.0)
        );

        2.0 * PI * (1.0 - self.seasons_ls(Seasons::Perihelion)[0] as f64 / 360.0)
    }

    fn epoch(&self) -> JulianDate {
        2.442765667e6
        // 2442765.66667
    }

    fn seasons_ls(&self, season: Seasons) -> Vec<u32> {
        match season {
            Seasons::VernalEquinox => (0..90).collect(),
            Seasons::Aphelion => vec![71],
            Seasons::SummerSolstice => (90..180).collect(),
            Seasons::AutumnEquinox => (180..270).collect(),
            Seasons::Perihelion => vec![251],
            Seasons::WinterSolstice => (270..360).collect(),
            Seasons::None => (1000..1001).collect(),
        }
    }

    fn seasons_named(&self, solar_longitude: SolarLongitude) -> String {
        match solar_longitude as i32 {
            0..=90 => Seasons::VernalEquinox,
            71 => Seasons::Aphelion,
            90..=180 => Seasons::SummerSolstice,
            180..=270 => Seasons::AutumnEquinox,
            251 => Seasons::Perihelion,
            270..=360 => Seasons::WinterSolstice,
            _ => Seasons::None,
        }
        .as_ref()
        .to_string()
    }

    fn average_ls(&self) -> SolarLongitude {
        30.0
    }

    /// https://www-mars.lmd.jussieu.fr/mars/time/martian_time.html
    ///
    /// Solving Kepler's Equation
    /// https://en.wikipedia.org/wiki/Kepler%27s_equation
    /// En+1 = En - ( (En - e * En.sin() - M(t) ) / (1 - e * En.cos()) );
    /// = En + En+1
    ///
    /// zdz > 10.0 for more precision
    /// En = zx0
    /// M(t) = xref
    /// En+1 = zdx
    /// zanom = Initial Mean Motion
    ///
    /// Solving Eccentric Anomaly
    /// https://en.wikipedia.org/wiki/True_anomaly
    /// v = 2 * ( ((1 + e) / (1 - e)).sqrt() * (E / 2).tan() ).atan()
    /// v = theta
    /// e = Orbital Eccentricty
    ///
    fn true_anomaly(&mut self, day: f64) -> Anomaly {
        let mut zdx = 10.0;

        // since perihelion date
        let zz = (day - self.perihelion_date()) / self.year_in_days(false);
        println!("ZZ: {zz}");

        let zanom = 2.0 * PI * (zz - zz.round());
        println!("Zanom: {zanom}");

        let xref = zanom.abs();
        println!("Xref 1: {xref}");

        let mut zx0 = xref + self.orbital_eccentricity() * xref.sin();
        println!("Zx0 +: {zx0}");

        while zdx > 1.0e-7 {
            // En = - ((En - e * En.sin() - M(t)) / 1 - e * En.cos() )
            zdx = -(zx0 - self.orbital_eccentricity() * zx0.sin() - xref)
                / (1.0 - self.orbital_eccentricity() * zx0.cos());

            // En = En + En+1
            zx0 = zx0 + zdx;
        }

        if zanom < 0.0 {
            zx0 = -zx0;
        };

        println!("Zx0 -: {zx0}");

        let mean_motion =
            ((1.0 + self.orbital_eccentricity()) / (1.0 - self.orbital_eccentricity())).sqrt();

        // Eccentric Anomaly
        // v = 2 * ( ((1 + e) / (1 - e)).sqrt() * (E / 2).tan() ).atan()
        let zteta = 2.0 * (mean_motion * (zx0 / 2.0).tan()).atan();

        println!("Zteta: {zteta}");

        return zteta;
    }

    /// https://squarewidget.com/keplers-equation/
    /// https://squarewidget.com/solar-coordinates/
    /// Find the true position of the planet by it's anomaly
    /// M = l - u
    /// l = Mean Longitude
    /// u = Longitude of the pericenter
    /// https://en.wikipedia.org/wiki/Mean_anomaly
    fn compute_ls(&mut self, day: f64) -> SolarLongitude {
        let theta = self.true_anomaly(day);

        // Mean anomaly; position of body in orbit since the perihelian time
        let mut ls = theta - self.perihelion_time();

        // if its less than the perihelion time
        if ls < 0.0 {
            ls += RIC;
        }

        // if its more than the perihelion time
        if ls > RIC {
            ls -= RIC;
        }

        println!("Degrees {}", ls.to_degrees());
        return ls.to_degrees();
    }

    /// https://www-mars.lmd.jussieu.fr/mars/time/martian_time.html
    fn to_date(&mut self, jd: JulianDate) -> KeplerianDate {
        let mut tmp_year = 12.0;
        let mut tmp_day =
            (jd - self.epoch()) * Earth::default().day_in_seconds(true) / self.day_in_seconds(true);

        // After Discovery
        while tmp_day >= self.year_in_days(false) {
            tmp_day -= self.year_in_days(false);
            tmp_year += 1.0;
        }

        // Before Discovery
        while tmp_day < 0.0 {
            tmp_day += self.year_in_days(false);
            tmp_year -= 1.0;
        }

        let ls = self.compute_ls(tmp_day);

        let year = tmp_year;
        let month = 1.0 + (ls / self.average_ls()).floor();
        let day = 1.0 + tmp_day.floor();

        // AD vs BD
        return KeplerianDate {
            year,
            month,
            day,
            ls,
        };
    }

    /// https://en.wikipedia.org/wiki/Timekeeping_on_Mars
    /// https://en.wikipedia.org/wiki/Universal_Time#Versions
    /// https://en.wikipedia.org/wiki/Timekeeping_on_Mars#Coordinated_Mars_Time
    fn to_time(&mut self) -> KeplerianDateTime {}
}

impl Mars {
    pub fn to_mars(date: EarthDate, mtc: Martian) -> String {
        // converts gregorian date to julian date to mars date with timezone

        let greg =
            icu_calendar::Date::try_new_gregorian_date(date.year, date.month, date.day).unwrap();

        let jd = get_jd(
            greg.year().number as i32,
            greg.month().ordinal as i32,
            greg.day_of_month().0 as i32,
            0.0,
        );

        // let mars_date = Mars::default().to_date(jd, mtc);
        String::new()
    }
}

impl Martian {
    /// https://marscalendar.com/time-zones
    /// Adds 2.5 hours when entering new timezone
    /// 2.5 hours = 1 decisol
    pub fn set_tz(&self, body: Martian) -> f64 {
        let tz = match body {
            // West
            Martian::MTCn5 => -2.5 * 5.0,
            Martian::MTCn4 => -2.5 * 4.0,
            Martian::MTCn3 => -2.5 * 3.0,
            Martian::MTCn2 => -2.5 * 2.0,
            Martian::MTCn1 => -2.5 * 1.0,
            // Center
            Martian::MTC => 0.0,
            // East
            Martian::MTCp1 => 2.5,
            Martian::MTCp2 => 2.5 * 2.0,
            Martian::MTCp3 => 2.5 * 3.0,
            Martian::MTCp4 => 2.5 * 4.0,
            Martian::MTCp5 => 2.5 * 5.0,
        };

        // MTCn5 to MTCp5
        // if 2.5 * 5.0 == tz {
        // -2.5 * 10.0;
        // }

        return tz;
    }
    /// The known Martian MTC time line
    pub fn prime_meridian(&self, body: Martian) -> (i32, i32) {
        match body {
            Martian::MTCn5 => (-180, -162),
            Martian::MTCn4 => (-162, -126),
            Martian::MTCn3 => (-126, -90),
            Martian::MTCn2 => (-90, -54),
            Martian::MTCn1 => (-54, -18),
            Martian::MTC => (-18, 18),
            Martian::MTCp1 => (18, 54),
            Martian::MTCp2 => (54, 90),
            Martian::MTCp3 => (90, 126),
            Martian::MTCp4 => (126, 162),
            Martian::MTCp5 => (162, 180),
        }
    }

    pub fn named_prime_meridian(&self, body: Martian) -> String {
        match body {
            Martian::MTCn5 => "Amazonis Time".to_string(),
            Martian::MTCn4 => "Olympus Time".to_string(),
            Martian::MTCn3 => "Tharsis Time".to_string(),
            Martian::MTCn2 => "Marineris Time".to_string(),
            Martian::MTCn1 => "Argyre Time".to_string(),
            Martian::MTC => "Noachis Time".to_string(),
            Martian::MTCp1 => "Arabia Time".to_string(),
            Martian::MTCp2 => "Hellas Time".to_string(),
            Martian::MTCp3 => "Utopia Time".to_string(),
            Martian::MTCp4 => "Elysium Time".to_string(),
            Martian::MTCp5 => "Arcadia Time".to_string(),
        }
    }

    pub fn code_prime_meridian(&self, body: Martian) -> String {
        match body {
            Martian::MTCn5 => "AMT".to_string(),
            Martian::MTCn4 => "OT".to_string(),
            Martian::MTCn3 => "TT".to_string(),
            Martian::MTCn2 => "MT".to_string(),
            Martian::MTCn1 => "AGT".to_string(),
            Martian::MTC => "NT".to_string(),
            Martian::MTCp1 => "ABT".to_string(),
            Martian::MTCp2 => "HT".to_string(),
            Martian::MTCp3 => "UT".to_string(),
            Martian::MTCp4 => "ET".to_string(),
            Martian::MTCp5 => "ACT".to_string(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::{Datelike, Timelike};

    use crate::shared::{
        earth_calendar::{get_jd, jd2greg},
        keplerian_body::{KeplerianBody, Mars},
    };

    #[test]
    /// 2024 Super Bowl Date
    pub fn mars_super_bowl() {
        let greg = get_jd(2024, 2, 11, 11.5);
        let date = Mars::default().to_date(greg);
        jd2greg(greg);
        println!("SuperBowl: {:?}", date);
    }

    /// Current Date
    #[test]
    pub fn mars_today() {
        let now = chrono::Utc::now();
        let greg = get_jd(
            now.year(),
            now.month() as i32,
            now.day() as i32,
            (now.hour() + now.minute() / 100) as f64,
        );
        let date = Mars::default().to_date(greg);
        jd2greg(greg);
        println!("Today: {:?}", date);
    }

    #[test]
    pub fn mars_china_today() {
        let now = chrono::Utc::now();
        let greg = get_jd(
            now.year(),
            now.month() as i32,
            now.day() as i32,
            now.hour() as f64,
        );
        // let china = convert_date(greg, icu_calendar::chinese::Chinese);
        // let date = Mars::default().to_date(china);
        // jd2greg(greg);
        // println!("Today: {:?}", date);
    }
}
