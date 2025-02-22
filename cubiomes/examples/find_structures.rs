use cubiomes::enums::*;
use cubiomes::generator::{BlockPosition, Generator, GeneratorFlags};
use cubiomes::structures::StructureRegion;

const STRUCTURE_TYPE: StructureType = StructureType::Mansion;
const MINECRAFT_VERSION: MCVersion = MCVersion::MC_1_21_WD;
const SEED: i64 = 4239805798134;

fn main() {
    let mut generator = Generator::new(
        MINECRAFT_VERSION,
        SEED,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    );

    let mut strucutre_region =
        StructureRegion::new(0, 0, generator.minecraft_version(), STRUCTURE_TYPE)
            .expect("we passed a valid structure for the version");

    let mut mansion_positions: Vec<BlockPosition> = Vec::new();

    // Iterates over a 20x20 area of regions, checking for mansions in each one
    for x in -10..10 {
        strucutre_region.x = x;
        for z in -10..10 {
            strucutre_region.z = z;

            if let Some(pos) = generator.try_generate_structure_in_region(strucutre_region) {
                mansion_positions.push(pos);
            }
        }
    }

    println!("Found mansions at: {:?}", mansion_positions)
}
