use crate::math::Vector3;
use crate::scene::Scene;

// function that traces the rays, it gets its own file because idk where else to put it
pub fn trace_ray(direction: &Vector3, scene: &Scene) -> [u8; 3] {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in scene.spheres.iter() {
        let t = sphere.ray_intersection(&scene.cam_pos, direction);

        if t > scene.view_frustum[0] && t < scene.view_frustum[1] && t < closest_t {
            closest_t = t;
            closest_sphere = Some(sphere);
        }
    }

    if let Some(sphere) = closest_sphere {
        sphere.color
    } else {
        scene.bg_color
    }
}

