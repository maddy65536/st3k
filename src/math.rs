// basic implementations of math stuff for doing math
#[derive(Debug)]
pub struct Vector3 {
    els: Vec<f64>,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { els: vec![x, y, z] }
    }

    pub fn from_color(color: [u8; 3]) -> Vector3 {
        Vector3::new(color[0] as f64, color[1] as f64, color[2] as f64)
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.els[0] * other.els[0] + self.els[1] * other.els[1] + self.els[2] * other.els[2]
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn scale(&self, scalar: f64) -> Vector3 {
        Vector3::new(
            self.els[0] * scalar,
            self.els[1] * scalar,
            self.els[2] * scalar,
        )
    }

    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.els[0] + other.els[0],
            self.els[1] + other.els[1],
            self.els[2] + other.els[2],
        )
    }

    pub fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.els[0] - other.els[0],
            self.els[1] - other.els[1],
            self.els[2] - other.els[2],
        )
    }

    pub fn to_color(&self) -> [u8; 3] {
        [self.els[0] as u8, self.els[1] as u8, self.els[2] as u8]
    }
}

pub fn canvas_to_viewport(
    x: i32,
    y: i32,
    canvas: [u32; 2],
    view: [f64; 2],
    proj_plane: f64,
) -> Vector3 {
    Vector3::new(
        (x as f64) * view[0] / (canvas[0] as f64),
        (y as f64) * view[0] / (canvas[1] as f64),
        proj_plane,
    )
}
