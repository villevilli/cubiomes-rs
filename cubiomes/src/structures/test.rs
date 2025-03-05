use crate::enums::*;
use crate::generator::{Generator, GeneratorFlags};
use crate::structures::strongholds::StrongholdIter;

#[test]
fn iterate_over_limit() {
    let generator = Generator::new(
        MCVersion::MC_1_21_WD,
        2103012030,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    );

    let mut iter: StrongholdIter = generator.strongholds();

    #[allow(clippy::unwrap_used)]
    let len = iter.size_hint().1.unwrap();

    for _ in 0..(len + 50) {
        iter.next();
    }
}
