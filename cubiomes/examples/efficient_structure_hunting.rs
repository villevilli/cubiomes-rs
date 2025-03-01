use cubiomes::{
    enums::{Dimension, MCVersion, StructureType},
    generator::{BlockPosition, Generator, GeneratorFlags},
    structures::StructureRegion,
};

// We attempt to find the specified structure on
// the specified version within the first chunk

// (this example is quite closely copied from cubiomes)

const STRUCTURE_TYPE: StructureType = StructureType::Igloo;
const MINECRAFT_VERSION: MCVersion = MCVersion::MC_1_21_WD;

fn main() {
    let mut lower_48: i64 = 0;

    // We initialize the position outside the first chunk so that we enter the while
    // loop
    let mut pos = BlockPosition { x: 20, z: 20 };

    let structure_region = StructureRegion::new(0, 0, MINECRAFT_VERSION, STRUCTURE_TYPE)
        .expect("This structure type exsists on this version of minecraft");

    // Loop until the structure generation attempt is inside the first chunk
    while pos.x >= 16 || pos.z >= 16 {
        // Increment the lover bits of the seed
        lower_48 += 1;

        if let Some(new_pos) = structure_region.get_structure_generation_attempt(lower_48) {
            pos = new_pos
        };
    }

    // Now that we have a seed with a generation attempt in the correct place
    // we initialize a generator that we can use to verify said generation attempt
    let mut generator = Generator::new(
        MINECRAFT_VERSION,
        lower_48,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    );

    let mut seed = lower_48;
    let mut upper_16: i64 = 0;

    // Loop while the biome is incorrect for the structure
    while !generator
        .verify_structure_generation_attempt(pos, STRUCTURE_TYPE)
        .expect("The structure type is valid for this generator")
    {
        // We modify the upper 16 bits
        upper_16 += 1;
        // We or the 16 bits to the lower 48 bits to get the full seed with
        // the structure at the specified coordinates
        seed = lower_48 | upper_16 << 48;

        // We then update the seed of the generator for the next round of
        // the while loop
        generator.apply_seed(Dimension::DIM_OVERWORLD, seed);
    }

    // Print the found valid instance of the seed and position
    println!("Found {:?} at {:?} on seed: {}", STRUCTURE_TYPE, pos, seed)
}
