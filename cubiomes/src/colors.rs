//! This module is used for biome color mappings.
//!
//! The module contains [BiomeColorMap] which uses an array backend to map
//! biomes into colors. There is also a functino to generate a [BTreeMap] which
//! maps biomes into colors. Its benchmarked to be slower than using
//! [BiomeColorMap]

use crate::enums::BiomeID;
use cubiomes_sys::num_traits::FromPrimitive;
use std::{
    collections::BTreeMap,
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

/// Function returns a map of biomeids to colors
///
/// Using a [`BiomeColorMap`] is probably faster, unless you desire a map type
/// for some reason.
///
/// This function is useful for generating pictures of biome maps as a binary
/// tree.
///
/// The colors are mapped as an array, the elements corresponding
/// to the Red Green And blue value respectively. eg \[RED, GREEN, BLUE]
///
/// The color scheme comes from cubiomes. The color scheme in cubiomes
/// is strongly inspired by the color scheme used in
/// [AMIDST](https://github.com/toolbox4minecraft/amidst/wiki/Biome-Color-Table)
#[must_use]
pub fn new_biome_color_map() -> BTreeMap<BiomeID, [u8; 3]> {
    let mut colors: MaybeUninit<[[u8; 3]; 256]> = MaybeUninit::uninit();

    // SAFETY: colors is the correct length of array for cubiomes.
    unsafe {
        cubiomes_sys::initBiomeColors(colors.as_mut_ptr() as *mut [u8; 3]);
    }

    // SAFETY: Colors was correctly initialized by ffi
    let colors = unsafe { colors.assume_init() };

    colors
        .into_iter()
        .enumerate()
        .filter_map(|(index, color)| BiomeID::from_usize(index).map(|biome_id| (biome_id, color)))
        .collect()
}

/// A map of biomeids to colors
///
/// This function is useful for generating pictures of biome maps.
///
/// The colors are mapped as an array, the elements corresponding to the Red
/// Green And blue value respectively. eg `[RED, GREEN, BLUE]` The map should
/// contain all biomes available in minecraft.
///
/// The color scheme comes from cubiomes. The color scheme in cubiomes is
/// strongly inspired by the color scheme used in
/// [AMIDST](https://github.com/toolbox4minecraft/amidst/wiki/Biome-Color-Table)
///
/// It's implemented as its own type which uses an array as its backend. The
/// array maps each biomeid number to a color. The id's which dont map to a id
/// are 0.
///
/// This approach was benchmarked to be faster than a map type. If you still
/// desire a seperate map type you can use [`new_biome_color_map()`] instead.
#[derive(Debug, Clone, Copy)]
pub struct BiomeColorMap([[u8; 3]; 256]);

impl Default for BiomeColorMap {
    fn default() -> Self {
        Self::new()
    }
}

impl BiomeColorMap {
    /// Constructs a new [BiomeColorMap] with colours from cubiomes.
    #[must_use]
    pub fn new() -> Self {
        let mut colors: MaybeUninit<[[u8; 3]; 256]> = MaybeUninit::uninit();

        // SAFETY: colors is the correct length of array for cubiomes.
        unsafe {
            cubiomes_sys::initBiomeColors(colors.as_mut_ptr() as *mut [u8; 3]);
        }

        // SAFETY: colors was initialized by ffi
        Self(unsafe { colors.assume_init() })
    }

    /// Gets a specific [BiomeID]'s color from the map. If not found
    /// (for some reason) returns [None]. The map should contain all biomes.
    #[must_use]
    pub fn get(&self, idx: BiomeID) -> &[u8; 3] {
        self.0.get(idx as usize).expect("All values should exists")
    }

    /// Gets a mutable reference to a specific [BiomeID]'s color.
    ///
    /// The function can be used to mutate the color map
    #[must_use]
    pub fn get_mut(&mut self, idx: BiomeID) -> &mut [u8; 3] {
        self.0
            .get_mut(idx as usize)
            .expect("All values should exist")
    }

    /// Returns a reference to the underlying array of the map.
    #[must_use]
    #[inline]
    pub fn as_arr(&self) -> &[[u8; 3]; 256] {
        &self.0
    }

    /// Returns a copy of the underlying array in the map.
    #[must_use]
    #[inline]
    pub fn to_arr(&self) -> [[u8; 3]; 256] {
        self.0
    }
}

impl Index<BiomeID> for BiomeColorMap {
    type Output = [u8; 3];

    fn index(&self, index: BiomeID) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<BiomeID> for BiomeColorMap {
    fn index_mut(&mut self, index: BiomeID) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
