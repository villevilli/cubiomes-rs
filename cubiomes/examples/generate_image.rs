// Generate image to a file called biome.png

// We need to import image for writing the image file to disk as some format
extern crate image;

// We write a 256x256 area from a seed into a file as an image

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use cubiomes::{
    colors::BiomeColorMap,
    enums::{Dimension, MCVersion},
    generator::{
        Cache, Generator, GeneratorFlags, Range,
        Scale::{self},
    },
};

fn main() {
    // We initialize the generator
    let seed: i64 = -4804349823814383506;
    let mc_version = MCVersion::MC_1_21_WD;
    let path = "biome.png";

    let generator = Generator::new(
        mc_version,
        seed,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    );

    // and cache
    let mut cache = Cache::new(
        &generator,
        Range {
            scale: Scale::Chunk,
            x: 1024,
            z: -256,
            size_x: 256,
            size_z: 256,
            y: 320,
            size_y: 0,
        },
    )
    .expect("Failed to generate cache");

    // Make an image buffer from the cache
    let img = cache.to_image(BiomeColorMap::new());

    // Open a file
    let mut file = BufWriter::new(File::create(path).expect("Failed to open file"));

    // Write said image to the file
    img.write_to(&mut file, image::ImageFormat::Png)
        .expect("Failed to write file");

    // Flush the contents to disk explicitly
    file.flush().expect("Failed to flush file")
}
