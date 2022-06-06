use crate::math::Vector3;
use crate::objects::Sphere;
use crate::scene::Scene;

// function that traces the rays, it gets its own file because idk where else to put it
pub fn trace_ray(
    pos: &Vector3,
    direction: &Vector3,
    scene: &Scene,
    t_min: f64,
    recursion_depth: u8,
) -> [u8; 3] {
    let (closest_sphere, closest_t) = closest_intersection(pos, direction, scene, t_min);

    if closest_sphere.is_none() {
        return scene.bg_color;
    }

    let sphere = closest_sphere.unwrap();

    let point = pos.add(&direction.scale(closest_t));
    let mut normal = point.sub(&sphere.center);
    normal = normal.scale(1.0 / normal.length());

    let base_color = Vector3::from_color(sphere.color).scale(compute_lighting(
        &point,
        &normal,
        &direction.scale(-1.0),
        scene,
        sphere.specular,
    ));

    if recursion_depth == 0 || sphere.reflective <= 0.0 {
        return base_color.to_color();
    }

    let r = reflect_ray(&direction.scale(-1.0), &normal);
    let reflected_color = trace_ray(&point, &r, scene, 0.001, recursion_depth - 1);

    base_color
        .scale(1.0 - sphere.reflective)
        .add(&Vector3::from_color(reflected_color).scale(sphere.reflective))
        .to_color()
}

fn reflect_ray(ray: &Vector3, normal: &Vector3) -> Vector3 {
    normal.scale(2.0 * normal.dot(ray)).sub(ray)
}

fn closest_intersection<'a>(
    point: &Vector3,
    direction: &Vector3,
    scene: &'a Scene,
    t_min: f64,
) -> (Option<&'a Sphere>, f64) {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in scene.spheres.iter() {
        let t = sphere.ray_intersection(point, direction);

        if t > t_min && t < scene.view_frustum[1] && t < closest_t {
            closest_t = t;
            closest_sphere = Some(sphere);
        }
    }

    (closest_sphere, closest_t)
}

fn compute_lighting(
    point: &Vector3,
    normal: &Vector3,
    obj_to_cam: &Vector3,
    scene: &Scene,
    specular: Option<f64>,
) -> f64 {
    let mut intensity: f64 = 0.0;

    for light in scene.lights.iter() {
        let light_dir = light.get_direction(point);
        let normal_dot_dir = normal.dot(&light_dir);

        // shadow check
        let (shadow_sphere, _) = closest_intersection(point, &light_dir, scene, 0.001);
        if shadow_sphere.is_some() {
            continue;
        }

        if normal_dot_dir > 0.0 {
            intensity += light.intensity * normal_dot_dir / (normal.length() * light_dir.length())
        }

        if let Some(spec) = specular {
            let r = normal.scale(2.0 * normal.dot(&light_dir)).sub(&light_dir);
            let r_dot_obj_to_cam = r.dot(obj_to_cam);
            if r_dot_obj_to_cam > 0.0 {
                intensity += light.intensity
                    * (r_dot_obj_to_cam / (r.length() * obj_to_cam.length())).powf(spec);
            }
        }
    }

    intensity + scene.amb_light
}
