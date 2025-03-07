use cubiomes::noise::SurfaceNoiseRelease;
use image::GrayImage;

fn main() {
    let noise = SurfaceNoiseRelease::new(
        cubiomes::enums::Dimension::DIM_OVERWORLD,
        -4804349823814383506,
    );

    let img = GrayImage::from_fn(256, 256, |x, y| {
        [noise.sample_between(x as i32, 0, y as i32, 0.0, 255.0) as u8].into()
    });

    img.save("noise.png").expect("failed to write image");
}
