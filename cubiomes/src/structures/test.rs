use crate::enums::*;
use crate::generator::{Generator, GeneratorFlags};

use super::Strongholds;

#[test]
fn iterate_over_limit() {
    let generator = Generator::new(
        MCVersion::MC_1_21_WD,
        2103012030,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    );

    let iter: Strongholds = generator.strongholds();

    let strongholds: Vec<crate::generator::BlockPosition> = iter.collect();

    dbg!(strongholds);
}
