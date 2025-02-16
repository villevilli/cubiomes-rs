//! Minecraft biome data generator
//!
//! This module is used to generate biomes from minecraft seeds
//! using the [Generator]
//!
//! # Usage
//!
//! For simple usage getting a biome at a specific place see [Generator::get_biome_at()]
//!
//! For more complicated usage, use a [Cache] generated with [Generator::new_cache()]
//!
//! # Detail
//!
//! This module follow closely to how the underlying cubiomes library works, but the
//! features have been wrapped by a safe rust api

use std::{
    alloc::{alloc, dealloc, Layout},
    fmt::Debug,
    mem::transmute,
};

use bitflags::bitflags;
use thiserror::Error;

use crate::enums;
use cubiomes_sys::{getMinCacheSize, num_traits::FromPrimitive};

///An error with the generator
///
/// This enum is produced as an error from the generator
#[derive(Error, Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    /// The underlying cubiomes library indicates an error with your biome request
    ///
    /// Cubiomes function getBiomeAt returned -1. This might indicate forgetting to
    /// initialize the seed.
    #[error(
        "Function getBiomeAt failed with error code -1, did you forgot to initialize the seed?"
    )]
    GetBiomeAtFailure,
    /// Failed to fill the cache
    ///
    /// This indicates, that cubiomes failed to fill the cache and returned a non 0
    /// exit code.
    #[error("Function genBiomes failed with error code {0}")]
    GenBiomeToCacheFailure(i32),
    /// Index out of bounds while getting from the cache
    ///
    /// This indicates that [`Cache::get_biome_at()`] tried to get an index
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
/// As cubiomes uses i32 for size, but states that it should be positive. (except for size_y)
/// I opted to use an unsized integer for abstraction. The conversion will
/// fail if either size_x or size_z is 0 or any size is bigger than [i32::MAX].
///
/// A size_y of 0 is equal to size_y of 1
#[derive(Error, Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

bitflags! {
    /// Flags for the cubiomes generator
    ///
    /// # Usage
    /// This indicates flags to pass to cubiomes. Unless you know what
    /// you are doing, you should probably leave these empty. Check the
    /// actual cubiomes library for documentation on what they do.
    pub struct GeneratorFlags: u32 {
        #[allow(missing_docs)]
        const LargeBiomes = 0x1;
        #[allow(missing_docs)]
        const NoBetaOcean = 0x2;
        #[allow(missing_docs)]
        const ForceOceanVariants = 0x4;
        //the source may set any bits
        #[allow(missing_docs)]
        const _ = !0;
    }
}

/// A scale for the [Range]
///
/// The scale reperesents valid options for giving to the generator
/// as a scale.
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Scale {
    /// A scale of 1:1, Block scale
    Block = 1,
    /// A scale of 1:4
    Quad = 4,
    /// A scale of 1:16, the size of a minecraft chunk
    Chunk = 16,
    /// A scale of 1:64, the size of 4 minecraft chunks
    QuadChunk = 64,
    /// A scale of 1:256, half of a minecraft region
    HalfRegion = 256,
}

impl Scale {
    pub fn scale_coord(&self, num: i32) -> i32 {
        num / *self as i32
    }

    pub fn unscale_coord(&self, num: i32) -> i32 {
        num * *self as i32
    }
}

/// Size and position for a [Cache]
///
/// The range represents a location and size of a cache.
///
/// The position and size of the range is scaled by its [Range::scale] attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Range {
    /// Scale used for the coordinates
    pub scale: Scale,
    /// Position of the top left corner of the range, scaled by the [Scale]
    ///
    /// to map this into minecraft coordinates the x is multiplied by the scale
    pub x: i32,
    /// Position of the top left corner of the range, scaled by the [Scale]
    ///
    /// to map this into minecraft coordinates the x is multiplied by the scale
    pub z: i32,
    /// Size of the range in x coordinates, scaled by the [Scale]
    ///
    pub size_x: u32,
    /// Size of the range in z coordinates, scaled by the [Scale]
    pub size_z: u32,
    /// The y coordinate of the range scaled either 1:1 or 1:4 depending on [Scale]
    ///
    /// Scale is 1:1 for [Scale::Block] other scales get a 1:4 mapping of the
    /// y coordinate
    pub y: i32,
    /// Veritcal size of the cube. 0 and 1 mean a 2d plane
    pub size_y: u32,
}

impl TryFrom<Range> for cubiomes_sys::Range {
    type Error = GeneratorError;

    fn try_from(value: Range) -> Result<Self, Self::Error> {
        Ok(Self {
            scale: value.scale as i32,
            x: value.x,
            z: value.z,
            sx: value
                .size_x
                .try_into()
                .map_err(|_| TryFromRangeError::XSizeOutOfBounds)
                .and_then(|sx| err_if_zero(sx, TryFromRangeError::XSizeOutOfBounds))?,
            sz: value
                .size_z
                .try_into()
                .map_err(|_| TryFromRangeError::ZSizeOutOfBounds)
                .and_then(|sz| err_if_zero(sz, TryFromRangeError::YSizeOutOfBouns))?,

            y: value.y,
            sy: value
                .size_y
                .try_into()
                .map_err(|_| TryFromRangeError::YSizeOutOfBouns)?,
        })
    }
}

impl Range {
    pub fn is_inside_range(&self, x: i32, z: i32) -> bool {
        ((self.x <= self.scale.scale_coord(x))
            && (self.scale.scale_coord(x) < (self.x + self.size_x as i32)))
            && ((self.z <= self.scale.scale_coord(z))
                && (self.scale.scale_coord(z) < (self.z + self.size_z as i32)))
    }

    pub fn global_to_local_coord(&self, x: i32, z: i32) -> Option<(u32, u32)> {
        if !self.is_inside_range(x, z) {
            None
        } else {
            Some((
                (self.scale.scale_coord(x) - self.x)
                    .try_into()
                    .expect("x should always been in range since we tested it before"),
                (self.scale.scale_coord(z) - self.z)
                    .try_into()
                    .expect("z should always been in range since we tested it before"),
            ))
        }
    }
}

fn err_if_zero(num: i32, err: TryFromRangeError) -> Result<i32, TryFromRangeError> {
    if num == 0 {
        return Err(err);
    }
    Ok(num)
}

/// The cubioems generator
///
/// This is the struct which holds a cubiomes generator
/// and is used for most actions with the generator
///
/// A new instance of the generator can be created with [`Generator::new()`]
///
/// Biomes can be generated either with [`Self::get_biome_at()`] for single points
/// in conjuntion with a [`Cache`] generated by [`Self::new_cache()`]
#[derive(Debug)]
pub struct Generator {
    generator: *mut cubiomes_sys::Generator,
}

impl Drop for Generator {
    fn drop(&mut self) {
        // Safety:
        // The memory is safe to deallocate as its been allocated in new
        // and the pointer to it is dropped, so it is never referred to again
        unsafe {
            dealloc(self.generator as *mut u8, Layout::new::<Generator>());
        }
    }
}

impl Generator {
    /// Initializes a new generator for the given minecraft version and flags
    /// with a seed and dimension applied
    ///
    /// This function initializes a new cubiomes generator and then gives
    /// it a seed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cubiomes::generator::{Generator, GeneratorFlags};
    /// use cubiomes::enums::{MCVersion, Dimension};
    ///
    /// let seed: i64 = -4804349813814383506;
    /// let mc_version = MCVersion::MC_1_21_WD;
    ///
    /// let generator = Generator::new(mc_version, seed, Dimension::DIM_OVERWORLD, GeneratorFlags::empty());
    ///
    /// // Use the generator for something
    /// ```
    pub fn new(
        mc_version: enums::MCVersion,
        seed: i64,
        dimension: enums::Dimension,
        flags: GeneratorFlags,
    ) -> Self {
        // SAFETY:
        // the generator is immediatly given a seed
        unsafe {
            let mut generator = Generator::new_without_seed(mc_version, flags);
            generator.apply_seed(dimension, seed);

            generator
        }
    }

    /// Initializes a new generator for the given minecraft version and flags
    ///
    /// This function initializes a new cubiomes generator for the specified
    /// version of minecraft with the specified flags. To use the generator it
    /// must be given a seed with [`Self::apply_seed()`]
    ///
    /// # Safety
    /// Before using any generation functions one must use [`Self::apply_seed()`]
    /// to give the generator a seed, otherwise the generation will fail.
    ///
    /// # Examples
    /// ```
    ///    
    /// use cubiomes::generator::Generator;
    /// use cubiomes::enums::MCVersion;
    /// use cubiomes::generator::GeneratorFlags;
    ///
    /// // Version of minecraft to use with the generator
    /// let mc_version = MCVersion::MC_1_21_WD;
    /// let generator;
    /// unsafe{
    ///     generator = Generator::new_without_seed(mc_version, GeneratorFlags::empty());
    /// }
    /// ```
    pub unsafe fn new_without_seed(version: enums::MCVersion, flags: GeneratorFlags) -> Self {
        // SAFETY:
        // The function is safe since the generated pointer
        // points to memory that can fit a Generator and
        // the pointer is stored as a pointer
        unsafe {
            let generator =
                alloc(Layout::new::<cubiomes_sys::Generator>()) as *mut cubiomes_sys::Generator;

            cubiomes_sys::setupGenerator(generator, version as i32, flags.bits());
            Self { generator }
        }
    }

    /// Sets the seed for the generator
    ///
    /// Sets a new seed to the generator. This can either be used for
    /// initialization if the generator was generated with [`Self::new_without_seed()`]
    /// or changing the seed of the generator
    pub fn apply_seed(&mut self, dimension: enums::Dimension, seed: i64) {
        // SAFETY:
        // As the generator is correctly initialized and its fields are private
        // the applySeed function is only given valid instances of generator
        unsafe {
            cubiomes_sys::applySeed(
                self.generator,
                dimension as i32,
                transmute::<i64, u64>(seed),
            );
        }
    }

    /// Tries to get a biomeid at the specific location.
    pub fn get_biome_at(&self, x: i32, y: i32, z: i32) -> Result<enums::BiomeID, GeneratorError> {
        // SAFETY:
        // As the generator is correctly initialized and its fields are private
        // the applySeed function is only given valid instances of generator.
        //
        // The scale enum guarantees that getBiomeAt is only given a scale of 1 or 4
        // As specified in its documentation
        unsafe {
            match cubiomes_sys::getBiomeAt(self.generator, Scale::Block as i32, x, y, z) {
                -1 => Err(GeneratorError::GetBiomeAtFailure),
                n => FromPrimitive::from_i32(n).ok_or(GeneratorError::BiomeIDOutOfRange(n)),
            }
        }
    }

    fn min_cache_size_from_range(&self, range: Range) -> usize {
        #[allow(clippy::unwrap_used)]
        let raw_range: cubiomes_sys::Range = range.try_into().unwrap();

        // SAFETY:
        // The conversion from cubiomes_sys::Range provides a range that fits the requirements
        // of the function
        unsafe {
            self.unchecked_min_cache_size(raw_range.scale, raw_range.sx, raw_range.sy, raw_range.sz)
        }
    }

    /// Gets the minimum cache size for a specific sized range
    ///
    /// y can be either 0 or 1 for a plane
    ///
    /// # Panics
    /// Panics if scale, size_x, or size_z are 0 or less and if size_y is less than 0
    unsafe fn unchecked_min_cache_size(
        &self,
        scale: i32,
        size_x: i32,
        size_y: i32,
        size_z: i32,
    ) -> usize {
        // SAFETY:
        // The unsafety is documented for the function.
        //
        // The requirement for this is checked from the source code and not from documentation
        unsafe { getMinCacheSize(self.generator, scale, size_x, size_y, size_z) }
    }

    /// Fills the provided cache from the generator
    ///
    /// # Safety
    /// The caller must guarantee, that the cache is able to contain the generated data.
    /// The best way to guarantee this, is to use a cache generated from this generator
    /// using the [Self::new_cache()] function.
    unsafe fn generate_biomes_to_cache(&self, cache: &mut Cache) -> Result<(), GeneratorError> {
        let result_num = cubiomes_sys::genBiomes(
            self.generator,
            cache.cache.as_mut_ptr(),
            cache.range.try_into()?,
        );

        // If error is returned from genbiomes, dont resize the vec as it may contain garbage data
        if result_num != 0 {
            return Err(GeneratorError::GenBiomeToCacheFailure(result_num));
        }

        // We set the caches lenght to what an user would want to read from it as we
        // can't be sure if cubiomes has actually initialized all variables beyond
        // the readable area.
        cache.cache.set_len(cache.calculate_readable_cache_length());

        Ok(())
    }
}

impl<'a> Generator {
    /// Generates a new cache for the given generator
    ///
    /// This function creates a new [`Cache`] against this version of the generator
    pub fn new_cache(&'a self, range: Range) -> Cache<'a> {
        let cache_size = self.min_cache_size_from_range(range);

        let cache = Vec::with_capacity(cache_size);

        Cache {
            cache,
            range,
            generator: self,
        }
    }
}

/// A cache for generating and holding a chunk of biome data
///
/// The cache is usually generated with [`Generator::new_cache()`]
/// and holds a vector filled with biome data.
#[derive(Clone)]
pub struct Cache<'a> {
    cache: Vec<i32>,
    range: Range,
    generator: &'a Generator,
}

//Custom dbg implementation, so we get the cache formatted as a table
impl Debug for Cache<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ":")?;
        writeln!(f, "Range: {:?}", &self.range)?;
        writeln!(f, "Cache: ")?;

        for line in self.cache.chunks(self.range.size_x as usize) {
            writeln!(f, "{:?}", line)?
        }
        Ok(())
    }
}

impl Cache<'_> {
    /// Fills the cache so it can be read
    pub fn fill_cache(&mut self) -> Result<(), GeneratorError> {
        // Safety:
        // As the cache holds a reference to the generator, the generator
        // could not have been modified after the vec was allocated so the
        // vec inside this cache holds enough space for the generator
        unsafe { self.generator.generate_biomes_to_cache(self) }
    }

    /// Gets a reference to the internal representation of the cache.
    ///
    /// The cache is a linear array which can be accessed at the following
    /// index: ``y * self.range.sx * self.range.sz + z * self.range.sx + x``
    /// If the cache is a plane use y=0 for the index
    ///
    /// # Examples
    ///
    /// ```
    /// use cubiomes::generator::{Cache, Range, Scale};
    ///
    /// use cubiomes::generator::{Generator, GeneratorFlags};
    /// use cubiomes::enums::{MCVersion, Dimension, BiomeID};
    ///
    /// let mut generator = Generator::new(
    ///     MCVersion::MC_1_21_WD,
    ///     -380434930381432806,
    ///     Dimension::DIM_OVERWORLD,
    ///     GeneratorFlags::empty()
    /// );
    ///
    /// let mut cache = generator.new_cache(Range {
    ///     scale: Scale::Block,
    ///     x: 512,
    ///     z: -512,
    ///     size_x: 64,
    ///     size_z: 64,
    ///     y: 100,
    ///     size_y: 0,
    /// });
    ///
    /// cache.fill_cache().expect("failed to fill cache");
    ///
    /// // Read the cache at z=32, x=5
    ///
    /// assert_eq!(cache.get_cache()[(13 + cache.get_range().size_x + 5) as usize], BiomeID::plains as i32);
    ///
    pub fn get_cache(&self) -> &Vec<i32> {
        &self.cache
    }

    /// Gets a read-only reference to the range used by this cache
    ///
    /// Gets the range this cache was generated with. Useful for
    /// if you want to read from the caches.
    ///
    /// See example from [`Self::get_cache()`] for example usage
    pub fn get_range(&self) -> &Range {
        &self.range
    }

    /// This function gets a biome at the specified point in the cache
    ///
    /// The specified point is relative to the left upper corner of
    /// the caches range.
    ///
    /// The cache start from x:0 y:0 mapping to the 0,0 of the range it
    /// was generated with. This means that attempting to read x: 16 or y:16
    /// on a cache with a size of 16 will be out of bounds.
    pub fn get_biome_at(&self, x: u32, y: u32, z: u32) -> Result<enums::BiomeID, GeneratorError> {
        let raw_biomeid = *self
            .cache
            .get((y * self.range.size_x * self.range.size_z + z * self.range.size_x + x) as usize)
            .ok_or(GeneratorError::IndexOutOfBounds)?;

        enums::BiomeID::from_i32(raw_biomeid).ok_or(GeneratorError::BiomeIDOutOfRange(raw_biomeid))
    }

    /// Moves the cache without reallocating the space
    ///
    /// Moves the cache to the new position without allocation.
    /// Can be used to generate multiple positions without reallocation.
    pub fn move_cache(&mut self, x: i32, y: i32, z: i32) {
        let new_range = Range {
            x,
            y,
            z,
            ..self.range
        };

        // As the size of the cache is only affeted by scale and size, moving the location won't
        // change the excpected cache size

        self.range = new_range
    }

    /// Calculates the actual size of readable data within the cache
    fn calculate_readable_cache_length(&self) -> usize {
        let y_size = match self.range.size_y {
            0 => 1,
            n => n,
        };
        (self.range.size_x * self.range.size_z * y_size) as usize
    }
}
