use std::cmp;

// objects and their associated functions, like intersections
pub struct Sphere {
    center: Vector3,
    radius: f64,
    color: [u8; 3],
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, color: [u8; 3]) -> Sphere {
        Sphere {
            center,
            radius,
            color,
        }
    }

    // calculate the intersection of a ray and a sphere
    pub fn rayIntersection(&self, origin: Vector3, direction: Vector3) -> f64 {
        let centerToOrigin = origin.sub(self.center);

        let a = direction.dot(direction);
        let b = 2 * centerToOrigin.dot(direction);
        let c = centerToOrigin.dot(centerToOrigin) - self.radius * self.radius;

        let discriminant = b * b - 4 * a * c;
        if discriminant < 0 {
            return f64::INFINITY;
        }

        let t1 = (-b + discriminant.sqrt()) / (2 * a);
        let t2 = (-b - discriminant.sqrt()) / (2 * a);

        cmp::min(t1, t2)
    }
}
