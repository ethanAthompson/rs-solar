use crate::shared::date::EarthDateTime;
use crate::sub_icu;
use displaydoc::Display;
use std::{cell::Cell, marker::PhantomData};

pub mod index;
pub mod julian;
pub mod julian_test;
pub mod macros;
pub mod macros2;
pub mod tests;

impl EarthDateTime {
    pub fn debug(&self) {
        println!("{:?}", self);
    }

    pub fn to_gregorian(&self, calendar_type: &str) -> EarthDateTime {
        match calendar_type {
            "iso" => sub_icu!(iso self),
            "chinese" => sub_icu!(chinese self),
            "indian" => sub_icu!(indian self),
            "julian" => sub_icu!(julian self),
            "gregorian" => sub_icu! {gregorian self},
            _ => EarthDateTime::default(),
        }
    }
}
