#![allow(dead_code)]
mod math;
mod scene;
mod objects;
mod trace;
mod lights;

use scene::Scene;
use image::{ImageBuffer, ImageResult, Rgb, RgbImage};

fn main() -> ImageResult<()> {
    const CANVAS_SIZE: [u32; 2] = [512, 512];

    // create image buffer to store results
    let mut img: RgbImage = ImageBuffer::new(CANVAS_SIZE[0], CANVAS_SIZE[1]);
    let scene = Scene::default_scene();

    for x in 0..CANVAS_SIZE[0] {
        for y in 0..CANVAS_SIZE[1] {
            let direction = math::canvas_to_viewport(
                -(CANVAS_SIZE[0] as i32)/2 + (x as i32),
                (CANVAS_SIZE[1] as i32)/2 - (y as i32),
                CANVAS_SIZE,
                scene.view_size,
                scene.proj_plane,
            );
            
            let color = trace::trace_ray(&direction, &scene);

            img.put_pixel(x, y, Rgb(color));
        }
    }

    img.save("test.png")
}
