use cubiomes::{
    enums::{Dimension, MCVersion},
    generator::{Cache, Generator, GeneratorFlags, Range},
};
use rand::{rngs::SmallRng, Rng, SeedableRng};

const SEED: u64 = 937457292385;
const POINT_AMOUNT: u32 = 100;
const TEST_Y: i32 = 320;
const BASE_RANGE: Range = Range {
    scale: cubiomes::generator::Scale::Block,
    x: 0,
    z: 0,
    size_x: 32,
    size_z: 32,
    y: TEST_Y,
    size_y: 0,
};

fn get_random_point(rng: &mut (impl SeedableRng + Rng)) -> (i32, i32) {
    rng.random()
}

#[test]
fn test_random_points() {
    let mut rng = SmallRng::seed_from_u64(SEED);

    for _ in 0..POINT_AMOUNT {
        let generator = Generator::new(
            MCVersion::MC_1_21_WD,
            rng.random(),
            Dimension::DIM_OVERWORLD,
            GeneratorFlags::empty(),
        );

        let (x, z) = get_random_point(&mut rng);

        let biome_get_biome_at = generator.get_biome_at(x, 320, z).unwrap_or_else(|_| {
            panic!(
                "Failed to generate biome at x: {}, y: {} z: {}",
                x, TEST_Y, z
            )
        });

        let mut cache = Cache::new(&generator, Range { x, z, ..BASE_RANGE });
        cache
            .fill_cache()
            .unwrap_or_else(|_| panic!("Failed to generate cache for range: {:?}", cache.range()));

        let (scaled_x, scaled_z) = cache.range().global_to_local_coord(x, z).unwrap();

        dbg!(&scaled_x);
        dbg!(&scaled_z);

        let biome_get_from_cache = cache.biome_at(scaled_x, 0, scaled_z).unwrap_or_else(|_| {
            panic!(
                "Failed to get biome from cache at x: {}, y: {} z: {}",
                x, TEST_Y, z
            )
        });

        assert_eq!(biome_get_biome_at, biome_get_from_cache)
    }
}
