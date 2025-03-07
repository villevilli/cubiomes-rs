//! A safe wrapper for the cubiomes library, which mimics
//! minecraft worldgen for seed finding and other purposes.
//!
//! This crate provides safe bindings for [cubiomes](https://github.com/Cubitect/cubiomes) by cubitect.
//! Cubiomes is intended for use in seed finding and biome map generation.
//!
//! The crate is organized into different modules which loosely correspond to
//! features available in cubiomes. The crate is still incomplete, as it doesn't
//! provide all features available in cubiomes.
//!
//! # Usage
//! See each module for usage of a specific feature of the library.
//!
//! - For biome generation see [`crate::generator`]
//! - For biome colormaps see [`crate::colors`]
//! - For structure geneartion see [`crate::structures`]
//!
//! # Features
//! This create provides the following features, which are all enabled by
//! default:
//! - `image`: Provides the option to generate images from biome areas using the
//!   [image](https://docs.rs/image/latest/image/) crate. Also see
//!   [generator::Cache::to_image()] for generating the images.
//!
//! - `cc_build`: Build cubiomes using the [cc](https://docs.rs/cc/latest/cc/)
//!   crate instead of make. It's benchamrked to be faster and might support
//!   more platforms than using make.

#![deny(clippy::ptr_cast_constness)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::todo)]
#![warn(clippy::missing_panics_doc)]

pub use cubiomes_sys::enums;

pub mod colors;
pub mod generator;
pub mod noise;
pub mod structures;

#[cfg(test)]
mod tests;
