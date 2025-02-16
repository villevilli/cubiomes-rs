//! A safe wrapper for the cubiomes library
//!
//! This crate provides safe bindings for [`cubiomes`]: https://github.com/Cubitect/cubiomes by cubitect.
//!
//! The crate is organized into different modules which loosely correspond to features
//! available in cubiomes. The crate is still incomplete, as it doesn't provide all features
//! available in cubiomes.
//!
//! # Usage
//! See each module for usage of a specific feature of the library.
//!
//! For biome generation see [`crate::generator`]
//!

#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(missing_docs)]

pub mod generator;

pub use cubiomes_sys::{enums, Dimension};

#[cfg(test)]
mod tests;
