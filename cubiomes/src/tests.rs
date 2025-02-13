use std::ffi::CStr;

use cubiomes_sys::{
    enums::{self, Dimension},
    Range,
};

use crate::enums::MCVersion;
use crate::generator::{Generator, GeneratorError, GeneratorFlags, Scale};

fn init_generator() -> Generator {
    let seed: i64 = -4804349703814383506;
    let mc_version = MCVersion::MC_1_21_WD;

    // SAFETY:
    // seed is immediatly applied
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
        generator.get_biome_at(Scale::Block, 700, 256, -2300)?,
        enums::BiomeID::mushroomIsland
    );
    Ok(())
}

#[test]
fn simple_biome_test_cached() -> Result<(), GeneratorError> {
    let generator = init_generator();

    let mut cache = generator.new_cache(Range {
        scale: 1,
        x: -128,
        z: -384,
        sx: 64,
        sz: 64,
        y: 100,
        sy: 0,
    });

    cache.fill_cache().expect("Failed to fill the cache");

    assert_eq!(cache.get_biome_at(5, 0, 6)?, enums::BiomeID::grove);
    assert_eq!(cache.get_biome_at(63, 0, 63)?, enums::BiomeID::frozen_peaks);

    Ok(())
}
