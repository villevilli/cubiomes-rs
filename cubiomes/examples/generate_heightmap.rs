// Generate the heightmap as an image to a file called heightmap.png

// We need to import image for writing the image file to disk as some format
extern crate image;

// We write a 256x256 area from a seed into a file as an image

use std::{
    fs::File,
    io::{BufWriter, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use cubiomes::{
    enums::{Dimension, MCVersion},
    generator::{Generator, GeneratorFlags},
    noise::SurfaceNoiseRelease,
};

fn main() {
    // We initialize the generator
    let seed: i64 = -4804349823814383506;
    let mc_version = MCVersion::MC_1_21_WD;
    let path = "heightmap.png";

    let generator = Generator::new(
        mc_version,
        seed,
        Dimension::DIM_OVERWORLD,
        GeneratorFlags::empty(),
    );

    // Make an image buffer from the generator
    let now = Instant::now();

    let surface_noise = SurfaceNoiseRelease::new(Dimension::DIM_OVERWORLD, seed);

    let img = generator
        .generate_heightmap_image(256, 1024, 256, 256, 40.0, 100.0, surface_noise.into())
        .expect("Overworld should always exist");

    sleep(Duration::from_millis(100).saturating_sub(now.elapsed()));

    // Open a file
    let mut file = BufWriter::new(File::create(path).expect("Failed to open file"));

    // Write said image to the file
    img.write_to(&mut file, image::ImageFormat::Png)
        .expect("Failed to write file");

    // Flush the contents to disk explicitly
    file.flush().expect("Failed to flush file")
}
