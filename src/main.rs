#![allow(dead_code)]
mod lights;
mod math;
mod objects;
mod scene;
mod trace;

use image::{ImageBuffer, ImageResult, Rgb, RgbImage};
use scene::Scene;

fn main() -> ImageResult<()> {
    const CANVAS_SIZE: [u32; 2] = [512, 512];

    // create image buffer to store results
    let mut img: RgbImage = ImageBuffer::new(CANVAS_SIZE[0], CANVAS_SIZE[1]);
    let scene = Scene::default_scene();

    for x in 0..CANVAS_SIZE[0] {
        for y in 0..CANVAS_SIZE[1] {
            let direction = math::canvas_to_viewport(
                -(CANVAS_SIZE[0] as i32) / 2 + (x as i32),
                (CANVAS_SIZE[1] as i32) / 2 - (y as i32),
                CANVAS_SIZE,
                scene.view_size,
                scene.proj_plane,
            );

            let color =
                trace::trace_ray(&scene.cam_pos, &direction, &scene, scene.view_frustum[0], 3);

            img.put_pixel(x, y, Rgb(color));
        }
    }

    img.save("test.png")
}
