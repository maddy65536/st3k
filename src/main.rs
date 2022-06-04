#![allow(dead_code)]
use image::{ImageBuffer, ImageResult, Rgb, RgbImage};

mod math;
mod scene;
mod objects;
mod trace;
use scene::Scene;

fn main() -> ImageResult<()> {
    const CANVAS_SIZE: [u32; 2] = [512, 512];

    // create image buffer to store results
    let mut img: RgbImage = ImageBuffer::new(CANVAS_SIZE[0], CANVAS_SIZE[1]);
    let scene = Scene::default_scene();

    for x in 0..CANVAS_SIZE[0] {
        for y in 0..CANVAS_SIZE[1] {
            let direction = math::canvas_to_viewport(
                CANVAS_SIZE[0]/2 + x,
                CANVAS_SIZE[1]/2 + y, 
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
