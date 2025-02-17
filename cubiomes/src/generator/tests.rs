use super::{
    position::{MinecraftPosition, StructureRegionPosition},
    Generator,
};
use crate::enums::*;

#[test]
fn test_structure_generation() {
    let seed = -5134222539607995087;
    let minecraft_version = MCVersion::MC_1_21_WD;
    let structure_type = todo!();

    let pos = StructureRegionPosition::new(
        MinecraftPosition::new(1712, -1840),
        minecraft_version,
        structure_type,
    );

    let generator = Generator::new(
        minecraft_version,
        seed,
        Dimension::DIM_OVERWORLD,
        super::GeneratorFlags::empty(),
    );
}
