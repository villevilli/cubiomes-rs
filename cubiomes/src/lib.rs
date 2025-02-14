#![warn(clippy::undocumented_unsafe_blocks)]

pub mod generator;

pub use cubiomes_sys::{enums, Dimension, Range};

#[cfg(test)]
mod tests;
