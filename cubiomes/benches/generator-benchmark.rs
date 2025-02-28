use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use cubiomes::{
    enums::{BiomeID, Dimension, MCVersion},
    generator::{
        colors::{new_biome_color_map, BiomeColorMapArr},
        BlockPosition, Cache, Generator, GeneratorFlags, Range,
    },
};
use cubiomes_sys::num_traits::FromPrimitive;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{hint::black_box, time::Duration};

const RNG_SEED: u64 = 90825401;
const RANGE: Range = Range {
    scale: cubiomes::generator::Scale::Block,
    x: 0,
    z: 0,
    size_x: 1024,
    size_z: 1024,
    y: 320,
    size_y: 0,
};

fn init_generator() -> Generator {
    Generator::new(
        MCVersion::MC_1_21_WD,
        0,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    )
}

pub fn generator_initialization(c: &mut Criterion) {
    let rng = SmallRng::seed_from_u64(RNG_SEED);

    let mut group = c.benchmark_group("generator_initialization");

    for seed in rng.random_iter::<i64>().take(10) {
        group.bench_with_input(BenchmarkId::from_parameter(seed), &seed, |bench, seed| {
            bench.iter(|| {
                Generator::new(
                    MCVersion::MC_1_21_WD,
                    *seed,
                    Dimension::DIM_OVERWORLD,
                    GeneratorFlags::empty(),
                )
            });
        });
    }
}

pub fn biome_generation_benchmark(c: &mut Criterion) {
    let mut generator = init_generator();
    let rng = SmallRng::seed_from_u64(RNG_SEED);

    let mut group = c.benchmark_group("biome_generation");

    group.sample_size(25);
    group.measurement_time(Duration::from_secs(30));

    for seed in rng.random_iter::<i64>().take(10) {
        group.bench_with_input(BenchmarkId::from_parameter(seed), &seed, |bench, seed| {
            generator.apply_seed(Dimension::DIM_OVERWORLD, *seed);
            bench.iter(|| {
                Cache::new(&generator, RANGE)
                    .fill_cache()
                    .expect("cubiomes failure");
            });
        });
    }
}

pub fn stronghold_generation(c: &mut Criterion) {
    let mut generator = init_generator();
    let rng = SmallRng::seed_from_u64(RNG_SEED);

    let mut group = c.benchmark_group("Stronghold Generation");

    group.sample_size(20);
    group.measurement_time(Duration::from_secs(30));

    for seed in rng.random_iter::<i64>().take(4) {
        group.bench_with_input(BenchmarkId::from_parameter(seed), &seed, |bench, seed| {
            generator.apply_seed(Dimension::DIM_OVERWORLD, *seed);
            bench.iter(|| generator.strongholds().collect::<Vec<BlockPosition>>());
        });
    }
}

pub fn compare_color_maps(c: &mut Criterion) {
    let mut generator = init_generator();
    let seed = 48527923439;
    generator.apply_seed(Dimension::DIM_OVERWORLD, seed);

    let mut group = c.benchmark_group("compare_color_maps");

    group.sample_size(500);

    for size in [512, 1024, 2048, 4096, 8196] {
        let mut area = Cache::new(
            &generator,
            Range {
                size_x: size,
                size_z: size,
                ..RANGE
            },
        );

        area.fill_cache().expect("Failed to fill cache");

        let area = area.as_vec();

        group.bench_function(BenchmarkId::new("btreemap", size.pow(2)), |bench| {
            let new_area = area.clone();
            let color_map = new_biome_color_map();

            bench.iter(|| {
                black_box::<Vec<[u8; 3]>>(
                    new_area
                        .iter()
                        .map(|pixel| color_map[&BiomeID::from_i32(*pixel).unwrap()])
                        .collect(),
                );
            });
        });

        group.bench_function(BenchmarkId::new("arraymap", size.pow(2)), |bench| {
            let new_area = area.clone();
            let color_map = BiomeColorMapArr::new();

            bench.iter(|| {
                black_box::<Vec<[u8; 3]>>(
                    new_area
                        .iter()
                        .map(|pixel| color_map[BiomeID::from_i32(*pixel).unwrap()])
                        .collect(),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    biome_generation_benchmark,
    generator_initialization,
    stronghold_generation,
    compare_color_maps
);
criterion_main!(benches);
