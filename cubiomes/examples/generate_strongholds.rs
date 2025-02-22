use cubiomes::{
    enums::*,
    generator::{BlockPosition, Generator, GeneratorFlags},
};

const MINECRAFT_VERSION: MCVersion = MCVersion::MC_1_21_WD;
const SEED: i64 = 4239805798134;

fn main() {
    let generator = Generator::new(
        MINECRAFT_VERSION,
        SEED,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    );

    let strongholds: Vec<BlockPosition> = generator.strongholds().collect();

    println!("Found strongholds at: {:?}", strongholds);
}
