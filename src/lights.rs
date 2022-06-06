use crate::math::Vector3;

pub struct PointLight {
    pub intensity: f64,
    pos: Vector3,
}

impl PointLight {
    pub fn new(intensity: f64, pos: Vector3) -> PointLight {
        PointLight { intensity, pos }
    }

    pub fn get_direction(&self, point: &Vector3) -> Vector3 {
        self.pos.sub(point)
    }
}
