//! This module is used for biome color mappings

use cubiomes_sys::num_traits::FromPrimitive;

use crate::enums::BiomeID;
use std::{collections::BTreeMap, mem::MaybeUninit};

/// Function returns a map of biomeids to colors
///
/// This function is useful for generating pictures of biome maps.
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
        cubiomes_sys::initBiomeColors(colors.as_mut_ptr().cast::<[u8; 3]>());
    }

    // SAFETY: Colors was correctly initialized by ffi
    let colors = unsafe { colors.assume_init() };

    colors
        .into_iter()
        .enumerate()
        .filter_map(|(index, color)| BiomeID::from_usize(index).map(|biome_id| (biome_id, color)))
        .collect()
}
