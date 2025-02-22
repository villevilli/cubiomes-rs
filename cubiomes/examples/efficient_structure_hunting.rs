extern crate cubiomes;

use cubiomes::{
    enums::{Dimension, MCVersion, StructureType},
    generator::{position::BlockPosition, structures::StructureRegion, Generator, GeneratorFlags},
};

// We attempt to find the specified structure on
// the specified version within the first chunk

// (this example is quite closely copied from cubiomes)

const STRUCTURE_TYPE: StructureType = StructureType::Igloo;
const MINECRAFT_VERSION: MCVersion = MCVersion::MC_1_21_WD;

fn main() {
    let mut lower_48: i64 = 0b00000000_00000000_00000000_00000000;
    let mut pos = BlockPosition { x: 20, z: 20 };
    let structure_region = StructureRegion::new(0, 0, MINECRAFT_VERSION, STRUCTURE_TYPE)
        .expect("This structure type exsists on this version of minecraft");

    while pos.x >= 16 || pos.z >= 16 {
        lower_48 += 1;

        if let Some(new_pos) = structure_region.get_structure_generation_attempt(lower_48) {
            pos = new_pos
        };
    }

    let mut generator = Generator::new(
        MINECRAFT_VERSION,
        lower_48,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    );

    let mut seed = lower_48;
    let mut upper_16: i64 = 0;

    while !generator
        .verify_structure_generation_attempt(pos, STRUCTURE_TYPE)
        .expect("The structure type is valid for this generator")
    {
        upper_16 += 1;
        seed = lower_48 | upper_16 << 48;

        generator.apply_seed(Dimension::DIM_OVERWORLD, seed);
    }

    println!("Found {:?} at {:?} on seed: {}", STRUCTURE_TYPE, pos, seed)
}
