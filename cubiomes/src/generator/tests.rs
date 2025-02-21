use super::{position::BlockPosition, structures::StructureRegion, Generator};
use crate::enums::*;

#[test]
#[should_panic]
fn no_structure_found() {
    let seed = -834578276348761;
    let minecraft_version = MCVersion::MC_1_17_1;
    let structure_type = StructureType::Jungle_Temple;

    let pos = StructureRegion::from_block_position(
        BlockPosition::new(1923, 1020),
        minecraft_version,
        structure_type,
    )
    .expect("Failed to generate region position");

    let generator = Generator::new(
        minecraft_version,
        seed,
        Dimension::DIM_OVERWORLD,
        super::GeneratorFlags::empty(),
    );

    #[allow(clippy::unwrap_used)]
    generator.try_generate_structure_in_region(pos).unwrap();
}

#[test]
fn test_structure_generation() {
    let seed = -5134222539607995087;
    let minecraft_version = MCVersion::MC_1_21_WD;
    let structure_type = StructureType::Outpost;

    let pos = StructureRegion::from_block_position(
        BlockPosition::new(3888, 2656),
        minecraft_version,
        structure_type,
    )
    .expect("Failed to generate region position");

    dbg!(pos);

    let generator = Generator::new(
        minecraft_version,
        seed,
        Dimension::DIM_OVERWORLD,
        super::GeneratorFlags::empty(),
    );

    generator
        .try_generate_structure_in_region(pos)
        .expect("Couldn't find structure when there should be a structure");
}

#[test]
fn test_structure_generation_negative() {
    let seed = -5134222539607995087;
    let minecraft_version = MCVersion::MC_1_21_WD;
    let structure_type = StructureType::Igloo;

    let pos = StructureRegion::from_block_position(
        BlockPosition::new(-354, -1808),
        minecraft_version,
        structure_type,
    )
    .expect("Failed to generate region position");

    dbg!(pos);

    let generator = Generator::new(
        minecraft_version,
        seed,
        Dimension::DIM_OVERWORLD,
        super::GeneratorFlags::empty(),
    );

    generator
        .try_generate_structure_in_region(pos)
        .expect("Couldn't find structure when there should be a structure");
}
