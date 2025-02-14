//! A safe wrapper for the cubiomes library
//!
//! This crate provides safe bindings for [`cubiomes`]: https://github.com/Cubitect/cubiomes by cubitect.
//!
//! For biome generation see [`crate::generator`]
//!

#![warn(clippy::undocumented_unsafe_blocks)]

pub mod generator;

pub use cubiomes_sys::{enums, Dimension, Range};

#[cfg(test)]
mod tests;
