//! This module contains generated rust enums representing different cubiomes
//! enums. It also implements display and fromstr for them.
//!
//! The enums in this module are automatically generated from the version of
//! cubiomes this crate links against. It also adds methods to display the
//! structures.
//!
//! The display methods will panic if cubiomes generates invalid data. Cubiomes
//! should never generate invalid data, so the functions should never panic
use core::str;
use std::{
    error::Error,
    ffi::{CStr, CString},
    fmt::Display,
    str::FromStr,
};

use num_traits::{FromPrimitive, ToPrimitive};

use crate::{biome2str, mc2str, str2mc, struct2str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    NonAsciiStr,
    NotMCVersion,
    InvalidCString(std::ffi::NulError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NonAsciiStr => write!(f, "Value is not an ascii sequence"),
            ParseError::NotMCVersion => write!(f, "Value does not represent a minecraft version"),
            ParseError::InvalidCString(_) => {
                write!(f, "Could not format self as a c string")
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Self::InvalidCString(e) = self {
            Some(e)
        } else {
            None
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl From<std::ffi::NulError> for ParseError {
    fn from(value: std::ffi::NulError) -> Self {
        Self::InvalidCString(value)
    }
}

include!(concat!(env!("OUT_DIR"), "/biome_enums.rs"));

impl BiomeID {
    /// Converts the enum to its string representation in the specified version.
    ///
    /// We can't implement display or fromstr for this, since some biomeids just
    /// go renamed in 1.18, and as such, formatting differs per minecraft
    /// version.
    pub fn to_mc_biome_str(&self, version: MCVersion) -> &'static str {
        let chars = unsafe { biome2str(version as i32, self.to_i32().unwrap()) };

        // Assert that chars is not null, note that it should never be, but the api
        // could theoretically return null if somehow the MCVersion was invalid.
        assert!(!chars.is_null());

        let slice = unsafe { CStr::from_ptr(chars) };

        match slice.to_str() {
            Ok(str) => str,
            Err(_) => panic!(),
        }
    }
}

/// Formats the structure as its name
impl Display for StructureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = unsafe { struct2str(*self as i32) };

        assert!(!chars.is_null());
        let slice = unsafe { CStr::from_ptr(chars) };

        write!(f, "{}", slice.to_str().unwrap())
    }
}

impl Display for MCVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = unsafe { mc2str(*self as i32) };

        assert!(!chars.is_null());
        let slice = unsafe { CStr::from_ptr(chars) };

        write!(f, "{}", slice.to_str().unwrap())
    }
}

impl FromStr for MCVersion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseError::NonAsciiStr);
        }

        let s: CString = CString::from_str(s)?;

        let version = unsafe { str2mc(s.as_ptr()) };

        if version == 0 {
            Err(ParseError::NotMCVersion)
        } else {
            Ok(MCVersion::from_i32(version).unwrap())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::enums::{BiomeID, MCVersion, StructureType};

    #[test]
    fn test_biome_conversion() {
        assert_eq!(
            BiomeID::badlands.to_mc_biome_str(crate::enums::MCVersion::MC_1_6_4),
            "badlands"
        );

        assert_eq!(
            BiomeID::stony_shore.to_mc_biome_str(crate::enums::MCVersion::MC_1_18),
            "stony_shore"
        );

        assert_eq!(
            BiomeID::stone_shore.to_mc_biome_str(crate::enums::MCVersion::MC_1_6_4),
            "stone_shore"
        );
    }

    #[test]
    fn test_structure_conversion() {
        assert_eq!(StructureType::Bastion.to_string(), "bastion_remnant");
        assert_eq!(StructureType::Shipwreck.to_string(), "shipwreck");
        assert_eq!(StructureType::End_City.to_string(), "end_city");
    }

    #[test]
    fn test_mc_version_conversion() {
        assert_eq!(MCVersion::MC_1_15.to_string(), "1.15");
    }

    #[test]
    fn test_mc_version_parsing() {
        assert_eq!(
            MCVersion::MC_B1_7,
            "Beta 1.7".parse().expect("parsing erro")
        );
        assert_eq!(
            MCVersion::MC_1_15_2,
            "1.15.2".parse().expect("parsing erro")
        );
        assert_eq!(
            MCVersion::MC_1_10_2,
            "1.10.2".parse().expect("parsing erro")
        );
    }
}
