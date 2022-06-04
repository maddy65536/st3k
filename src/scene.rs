// struct to hold the scene's data
mod math;
use math::Vector3;

mod objects;
use objects::Sphere;

pub struct Scene {
    bgColor: [u8; 3],
    camPos: Vector3,
    projPlane: f64,
    viewSize: [f64; 2],
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn defaultScene() -> Scene {
        Scene {
            bgColor: [131, 205, 212],
            camPos: Vector3::new(0, 0, 0),
            projPlane: 1.0,
            viewSize: [1, 1],
            spheres: vec![
                Sphere::new(
                    center: Vector3(0, 0, 3),
                    radius: 1.0,
                    color: [0, 255, 0],
                )
            ]
        }
    }
}
