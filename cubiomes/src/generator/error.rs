//! Errors related to [`super::Generator`] and [`super::Range`]

use thiserror::Error;

/// An error with the generator
///
/// This enum is produced as an error from the generator
#[derive(Error, Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum GeneratorError {
    /// Biomeid produced by cubiomes is not a valid biome
    ///
    /// This error indicates that the underlying cubiomes library produced
    /// a biome that does not map to any valid biomeid. The out of range
    /// biomeid is given as a parameter
    ///
    /// This error probably indicates a bug, so feel free to report
    /// how you got it.
    #[error("Biome id {0} is out of range and is not a valid biomeid")]
    BiomeIDOutOfRange(i32),
    /// The underlying cubiomes library indicates an error with your biome
    /// request
    ///
    /// Cubiomes function getBiomeAt returned -1. This might indicate forgetting
    /// to initialize the seed.
    #[error(
        "Function getBiomeAt failed with error code -1, did you forgot to initialize the seed?"
    )]
    GetBiomeAtFailure,
    /// Failed to fill the cache
    ///
    /// This indicates, that cubiomes failed to fill the cache and returned a
    /// non 0 exit code.
    #[error("Function genBiomes failed with error code {0}")]
    GenBiomeToCacheFailure(i32),
    /// Index out of bounds while getting from the cache
    ///
    /// This indicates that [`super::Cache::biome_at()`] tried to get an index
    /// outside the bounds of the internal vector. This either means that
    /// the cache has not yet been filled, or you tried to get something outside
    /// of the lenght of the vector
    #[error("Index out of bounds")]
    IndexOutOfBounds,
    #[error("Failed to convert range")]
    /// An error happened converting the range for use with cubiomes.
    TryFromRangeError(TryFromRangeError),
}

impl From<TryFromRangeError> for GeneratorError {
    fn from(value: TryFromRangeError) -> Self {
        Self::TryFromRangeError(value)
    }
}

/// The given size x y or z is too big to fit an i32 or x or z are zero.
///
/// As cubiomes uses i32 for size, but states that it should be positive.
/// (except for `size_y`) I opted to use an unsized integer for abstraction. The
/// conversion will fail if either `size_x` or `size_z` is 0 or any size is
/// bigger than [`i32::MAX`].
///
/// A `size_y` of 0 is equal to `size_y` of 1
#[derive(Error, Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum TryFromRangeError {
    #[error("x sixe is out of bounds for range")]
    #[allow(missing_docs)]
    XSizeOutOfBounds,
    #[error("z size is out of bounds for range")]
    #[allow(missing_docs)]
    ZSizeOutOfBounds,
    #[error("y size is out of bounds for range")]
    #[allow(missing_docs)]
    YSizeOutOfBouns,
}
