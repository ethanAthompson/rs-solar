use displaydoc::Display;

#[derive(Display, Default, Debug, Clone, PartialEq, Eq)]
pub struct EarthDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Display, Default, Debug, Clone, PartialEq, Eq)]
pub struct EarthDateTime {
    pub date: (i32, u32, u32),
    pub codes: (String, String, String),
    pub time: (u8, u8, u8),
}

#[derive(Display, Default, Debug, Clone, PartialEq, Eq)]
pub struct SpaceDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub ls: u32,
}

#[derive(Display, Debug, Clone, PartialEq, Eq)]
pub enum SupportedPlanets {
    Mars,
    Earth,
    Venus,
    Saturn,
    Jupiter,
    Neptune,
    Ceros,
    Titan,
    Pluto,
    Mercury,
}
