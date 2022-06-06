// struct to hold the scene's data
use crate::lights::PointLight;
use crate::math::Vector3;
use crate::objects::Sphere;

pub struct Scene {
    pub bg_color: [u8; 3],
    pub cam_pos: Vector3,
    pub proj_plane: f64,
    pub view_size: [f64; 2],
    pub view_frustum: [f64; 2],
    pub amb_light: f64,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl Scene {
    pub fn default_scene() -> Scene {
        Scene {
            bg_color: [131, 205, 212],
            cam_pos: Vector3::new(0.0, 0.0, 0.0),
            proj_plane: 1.0,
            view_size: [1.0, 1.0],
            view_frustum: [1.0, f64::INFINITY],
            amb_light: 0.2,
            spheres: vec![Sphere::new(
                Vector3::new(0.0, 0.0, 3.0),
                1.0,
                [164, 116, 212],
                Some(10.0),
            )],
            lights: vec![PointLight::new(0.8, Vector3::new(2.0, 1.0, 0.0))],
        }
    }
}
