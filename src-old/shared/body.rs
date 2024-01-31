/// Supports common functionality for related bodies
pub trait KeplerBody {
    pub fn semimajor() {}
    pub fn semiminor() {}
    pub fn minormajor() {}
    pub fn orbital_eccentricity() {}
    pub fn mean_anomaly() {}
    pub fn mean_motion() {}
    pub fn eccentric_anomaly() {}
    pub fn true_anomaly() {}
    pub fn perihelion_date() {}
    pub fn perihelion_time() {}
    pub fn what_shape() {}
    pub fn day_in_seconds() {}
    pub fn year_in_days() {}
}
