pub trait SpaceSolarTime {
    fn solar_day_in_seconds(&self) -> f32;
    fn solar_hour_in_seconds(&self) -> f32;
    fn solar_minute_in_seconds(&self) -> f32;
    fn solar_second_in_seconds(&self) -> f32;
}

pub trait SpaceSiderealTime {
    fn sidereal_day_in_seconds(&self) -> f32;
    fn sidereal_hour_in_seconds(&self) -> f32;
    fn sidereal_minute_in_seconds(&self) -> f32;
    fn sidereal_second_in_seconds(&self) -> f32;
}
