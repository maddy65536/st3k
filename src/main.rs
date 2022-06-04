use image::{ImageBuffer, ImageResult, Rgb, RgbImage};

mod math;
use math::Vector3;

fn main() -> ImageResult<()> {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;

    // create image buffer to store results
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            img.put_pixel(x, y, Rgb([0, 255, 0]));
        }
    }

    img.save("test.png")
}
