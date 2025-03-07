//! Module for functions dealing with raw noise
//!
//! Unless you are doing something advanced, this module is only used for
//! initializing noise used in heightmap generation.

use std::{
    alloc::{self, dealloc, Layout},
    mem::transmute,
};

use cubiomes_sys::{enums::Dimension, initSurfaceNoise, initSurfaceNoiseBeta};

/// This enum represents any surfacenoise.
///
/// See each surfance noise respectively for its usage:
/// - [SurfaceNoiseRelease]
/// - [SurfaceNoiseBeta]
#[derive(Debug)]
pub enum BiomeNoise {
    /// A release (post 1.0) biomenoise
    Release(SurfaceNoiseRelease),
    /// A beta minecraft biomenoise
    Beta(SurfaceNoiseBeta),
}

impl From<SurfaceNoiseRelease> for BiomeNoise {
    fn from(value: SurfaceNoiseRelease) -> Self {
        Self::Release(value)
    }
}

impl From<SurfaceNoiseBeta> for BiomeNoise {
    fn from(value: SurfaceNoiseBeta) -> Self {
        Self::Beta(value)
    }
}

/// Represents surfacenoise from release (post 1.0) minecraft.
#[derive(Debug)]
pub struct SurfaceNoiseRelease(*mut cubiomes_sys::SurfaceNoise);

/// Represents surfacenoise from beta minecraft.
#[derive(Debug)]
pub struct SurfaceNoiseBeta(*mut cubiomes_sys::SurfaceNoiseBeta);

impl SurfaceNoiseRelease {
    /// Initializes a new [self] for the given seed and [Dimension]
    pub fn new(dimension: Dimension, seed: i64) -> Self {
        // SAFETY: Layout is not zero sized.
        let noise: *mut cubiomes_sys::SurfaceNoise = unsafe {
            alloc::alloc(Layout::new::<cubiomes_sys::SurfaceNoise>())
                as *mut cubiomes_sys::SurfaceNoise
        };

        // SAFETY: Arguments to foregin function are correct.
        unsafe {
            initSurfaceNoise(noise, dimension as i32, transmute::<i64, u64>(seed));
        }
        Self(noise)
    }

    /// Gets the underlying pointer inside [self].
    ///
    /// This function is mostly provided for use with cubiomes_sys.
    ///
    /// # Safety
    /// Mutating the underlying data using this pointer is unsafe. Instead use
    /// [`Self::as_mut_ptr()`] if you need mutable access.
    ///
    /// The pointer becomes dangling if [self] is droppped.
    pub unsafe fn as_ptr(&self) -> *const cubiomes_sys::SurfaceNoise {
        self.0
    }

    /// Gets the underlying pointer inside [self].
    ///
    /// This function is mostly provided for use with cubiomes_sys.
    ///
    /// # Safety
    /// The pointer becomes dangling if [self] is droppped.
    pub unsafe fn as_mut_ptr(&mut self) -> *mut cubiomes_sys::SurfaceNoise {
        self.0
    }
}

impl Drop for SurfaceNoiseRelease {
    fn drop(&mut self) {
        // SAFETY: Memory was initialized in the constructor
        unsafe {
            dealloc(
                self.0 as *mut u8,
                Layout::new::<cubiomes_sys::SurfaceNoise>(),
            )
        };
    }
}

// SAFETY: As interior mutation should not happen, the type is both send and
// sync
unsafe impl Send for SurfaceNoiseRelease {}
// SAFETY: As interior mutation should not happen, the type is both send and
// sync
unsafe impl Sync for SurfaceNoiseRelease {}

impl SurfaceNoiseBeta {
    /// Initializes a new surface noise.
    pub fn new(seed: i64) -> Self {
        // SAFETY: Layout is not zero sized.
        let noise: *mut cubiomes_sys::SurfaceNoiseBeta = unsafe {
            alloc::alloc(Layout::new::<cubiomes_sys::SurfaceNoiseBeta>())
                as *mut cubiomes_sys::SurfaceNoiseBeta
        };

        // SAFETY: Arguments to foreign function are correct.
        unsafe {
            initSurfaceNoiseBeta(noise, transmute::<i64, u64>(seed));
        }
        Self(noise)
    }

    /// Gets the underlying pointer inside [self].
    ///
    /// This function is mostly provided for use with cubiomes_sys.
    ///
    /// # Safety
    /// Mutating the underlying data using this pointer is unsafe. Instead use
    /// [`Self::as_mut_ptr()`] if you need mutable access.
    ///
    /// The pointer becomes dangling if [self] is droppped.
    pub unsafe fn as_ptr(&self) -> *const cubiomes_sys::SurfaceNoiseBeta {
        self.0
    }

    /// Gets the underlying pointer inside [self].
    ///
    /// This function is mostly provided for use with cubiomes_sys.
    ///
    /// # Safety
    /// The pointer becomes dangling if [self] is droppped.
    pub unsafe fn as_mut_ptr(&mut self) -> *mut cubiomes_sys::SurfaceNoiseBeta {
        self.0
    }
}

// SAFETY: As interior mutation does not happen, the type is both send and
// sync
unsafe impl Send for SurfaceNoiseBeta {}
// SAFETY: As interior mutation does not happen, the type is both send and
// sync
unsafe impl Sync for SurfaceNoiseBeta {}

impl Drop for SurfaceNoiseBeta {
    fn drop(&mut self) {
        // SAFETY: The memory was initialized in the constructor.
        unsafe {
            alloc::dealloc(
                self.0 as *mut u8,
                Layout::new::<cubiomes_sys::SurfaceNoiseBeta>(),
            )
        };
    }
}
