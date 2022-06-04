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
        let point = scene.cam_pos.add(&direction.scale(closest_t));
        let mut normal = point.sub(&sphere.center);
        normal = normal.scale(1.0 / normal.length());
        
        Vector3::from_color(sphere.color).scale(compute_lighting(&point, &normal, scene)).to_color()
    } else {
        scene.bg_color
    }
}

fn compute_lighting(point: &Vector3, normal: &Vector3, scene: &Scene) -> f64 {
    let mut intensity: f64 = 0.0;

    for light in scene.lights.iter() {
        let light_dir = light.get_direction(point);
        let normal_dot_dir = normal.dot(&light_dir);

        if normal_dot_dir > 0.0 {
            intensity += light.intensity * normal_dot_dir / (normal.length() * light_dir.length())
        }
    }

    intensity + scene.amb_light
}
