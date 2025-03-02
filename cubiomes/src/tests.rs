use crate::colors::{self, BiomeColorMap};
use crate::enums::MCVersion;
use crate::generator::{error::GeneratorError, Cache, Generator, GeneratorFlags, Range, Scale};

use cubiomes_sys::enums::{self, BiomeID, Dimension};
use cubiomes_sys::num_traits::FromPrimitive;
use std::ffi::CStr;

fn init_generator() -> Generator {
    let seed: i64 = -4804349703814383506;
    let mc_version = MCVersion::MC_1_21_WD;

    // SAFETY: seed is immediatly applied
    unsafe {
        let mut generator = Generator::new_without_seed(mc_version, GeneratorFlags::empty());
        generator.apply_seed(Dimension::DIM_OVERWORLD, seed);

        generator
    }
}

#[test]
fn biome_to_str_sanity() {
    let biome = enums::BiomeID::badlands;
    let version = MCVersion::MC_1_21_WD;

    let _str;

    #[allow(clippy::undocumented_unsafe_blocks)]
    unsafe {
        _str = CStr::from_ptr(cubiomes_sys::biome2str(version as i32, biome as i32));
    }
    dbg!(_str);
}

#[test]
fn simple_biome_test() -> Result<(), GeneratorError> {
    let generator = init_generator();

    assert_eq!(
        generator.get_biome_at(700, 256, -2300)?,
        enums::BiomeID::mushroomIsland
    );
    Ok(())
}

#[test]
fn simple_biome_test_cached() -> Result<(), GeneratorError> {
    let mut generator = init_generator();
    generator.apply_seed(Dimension::DIM_OVERWORLD, -1693727681172482083);

    let cache = Cache::new(
        &generator,
        Range {
            scale: Scale::Block,
            x: -128,
            z: -128,
            size_x: 16,
            size_z: 16,
            y: 64,
            size_y: 0,
        },
    )
    .expect("Failed to fill the cache");

    dbg!(&cache);

    assert_eq!(cache.biome_at(5, 0, 6)?, enums::BiomeID::meadow);
    assert_eq!(cache.biome_at(15, 0, 15)?, enums::BiomeID::snowy_slopes);

    Ok(())
}

const SOME_RANGE: Range = Range {
    scale: Scale::Block,
    x: 0,
    z: 0,
    size_x: 32,
    size_z: 32,
    y: 320,
    size_y: 0,
};

#[test]
fn test_range_in_bounds() {
    let range = Range { ..SOME_RANGE };
    assert!(range.is_inside(23, 14));
}

#[test]
fn test_range_border_in_bounds() {
    let range = Range { ..SOME_RANGE };
    assert!(range.is_inside(31, 31));
}

#[test]
#[should_panic]
fn test_range_outside_bounds() {
    let range = Range { ..SOME_RANGE };
    assert!(range.is_inside(32, 32));
}

#[test]
fn init_biome_colors() {
    let colors = colors::new_biome_color_map();

    dbg!(&colors);

    assert_eq!(colors.len(), 94);
}

#[test]
fn set_color() {
    let mut colors = BiomeColorMap::default();

    colors[BiomeID::badlands] = [13, 13, 2];

    assert_eq!(colors[BiomeID::badlands], [13, 13, 2])
}

#[test]
fn get_all_colors() {
    let biome_color_map = BiomeColorMap::new();

    for idx in 0..500 {
        if let Some(biome) = BiomeID::from_u32(idx) {
            let _ = biome_color_map.get(biome);
        }
    }
}
