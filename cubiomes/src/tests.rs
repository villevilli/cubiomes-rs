use std::ffi::CStr;

use cubiomes_sys::{
    enums::{self, MCVersion},
    Dimension,
};

use crate::generator::{Flags, Generator, GeneratorError, Scale};

#[test]
fn biome_to_str_sanity() {
    let biome = enums::BiomeID::badlands;
    let version = MCVersion::MC_1_21_WD;

    let str;
    unsafe {
        str = CStr::from_ptr(cubiomes_sys::biome2str(version as i32, biome as i32));
    }
}

#[test]
fn simple_biome_test() -> Result<(), GeneratorError> {
    let seed: i64 = -4804349703814383506;
    let mc_version = MCVersion::MC_1_21_WD;

    let mut generator = Generator::new(mc_version, Flags::empty());

    generator.apply_seed(Dimension::DIM_OVERWORLD, seed);

    assert_eq!(
        generator.get_biome_at(Scale::Block, 700, 100, -2300)?,
        enums::BiomeID::mushroomIsland
    );
    Ok(())
}
