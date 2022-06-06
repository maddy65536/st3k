use crate::math::Vector3;

// objects and their associated functions, like intersections
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub color: [u8; 3],
    pub specular: Option<f64>,
    pub reflective: f64,
}

impl Sphere {
    pub fn new(
        center: Vector3,
        radius: f64,
        color: [u8; 3],
        specular: Option<f64>,
        reflective: f64,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            specular,
            reflective,
        }
    }

    // calculate the intersection of a ray and a sphere
    pub fn ray_intersection(&self, origin: &Vector3, direction: &Vector3) -> f64 {
        let center_to_origin = origin.sub(&self.center);

        let a = direction.dot(direction);
        let b = 2.0 * center_to_origin.dot(direction);
        let c = center_to_origin.dot(&center_to_origin) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return f64::INFINITY;
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        t1.min(t2)
    }
}
