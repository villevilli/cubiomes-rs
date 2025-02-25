//! Raw ffi bindings to the cubiomes library.
//!
//! This crate also contains module [enums] for building enums for conviniance
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(clippy::all)]
#![allow(rustdoc::all)]

pub use num_traits;

#[macro_use]
extern crate num_derive;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// This module contains generated rust enums representing different cubiomes enums
///
/// The enums in this module are automatically generated from the version of cubiomes
/// this crate links against.
pub mod enums {
    include!(concat!(env!("OUT_DIR"), "/biome_enums.rs"));
}
