use std::str::FromStr;

use super::Martian;


#[derive(Debug)]
 pub enum TzError {
    None,
    Found
 }

impl FromStr for Martian {
    type Err = TzError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MTC-5" => Ok(Self::MTCn5),
            "MTC-4" => Ok(Self::MTCn4),
            "MTC-3" => Ok(Self::MTCn3),
            "MTC-2" => Ok(Self::MTCn2),
            "MTC-1" => Ok(Self::MTCn1),
            "MTC" => Ok(Self::MTC),
            "MTC+1" => Ok(Self::MTCp1),
            "MTC+2" => Ok(Self::MTCp2),
            "MTC+3" => Ok(Self::MTCp3),
            "MTC+4" => Ok(Self::MTCp4),
            "MTC+5" => Ok(Self::MTCp5),
            _ => todo!()
        }
    }
}