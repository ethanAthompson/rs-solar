use crate::{
    anomaly::Anomaly,
    conversions::radians_in_circle,
    kepler::{self, Body},
    orbit::{self, Perihelion, Season, SemiAxis, SolarLongitude},
    planets::EARTH_ROTATIONAL_PERIOD,
};

#[derive(Debug, Copy, Clone)]
/// This structure represents the fourth planet from the sun
pub struct Mars;

impl Body for Mars {
    /// A.D 1975 December 19, 04:00:00.3
    fn epoch(&self) -> f64 {
        2.442765667e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0934
    }

    fn orbital_period(&self) -> f64 {
        689.6
    }

    fn rotational_period(&self) -> f64 {
        88_775.245
    }

    fn semimajor(&self) -> f64 {
        1.52
    }

    fn semiminor(&self) -> f64 {
        SemiAxis(self.semimajor()).minor(self.orbital_eccentricity())
    }

    fn perihelion(&self) -> Perihelion {
        Perihelion {
            month: (468.5, 514.6),
            ls: (240.0, 270.0),
            perihelion: 251.0,
        }
    }

    fn mean_motion(&mut self, day: f64) -> f64 {
        let elapse = Perihelion::elapse(&mut self.perihelion(), day, self.orbital_period());

        radians_in_circle() * (elapse - elapse.round())
    }

    fn to_date(&mut self, julian_date: f64) -> crate::kepler::Date {
        let mut tmp_year = 12.0;
        let mut tmp_day =
            (julian_date - self.epoch()) * EARTH_ROTATIONAL_PERIOD / self.rotational_period();

        let periapis = Perihelion::time(&mut self.perihelion());

        let theta = Anomaly.truly(
            self.mean_motion(tmp_day),
            orbit::Type::default().shape(self.orbital_eccentricity()),
            self.orbital_eccentricity(),
            self.semimajor(),
        );

        while tmp_day >= self.orbital_period() {
            tmp_day -= self.orbital_period();
            tmp_year += 1.0;
        }

        while tmp_day < 0.0 {
            tmp_day += self.orbital_period();
            tmp_year -= 1.0;
        }

        let ls = SolarLongitude(theta - periapis).compute();
        let year = tmp_year;
        let month = 1.0 + (ls / self.perihelion().avg_ls()).floor();
        let day = 1.0 + tmp_day.floor();
        let season = Season::default().from(ls as u32);
        let era = match year as i32 > 0 {
            true => kepler::Eras::AD,
            false => kepler::Eras::BD,
        };

        kepler::Date {
            era,
            year,
            month,
            day,
            ls,
            season,
        }
    }

    fn to_time(&mut self) -> crate::kepler::Time {
        todo!()
    }
}
