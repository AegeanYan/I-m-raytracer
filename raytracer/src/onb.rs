use crate::Vec3;

pub struct Onb {
    pub axis: [Vec3; 3],
}

impl Onb {
    pub fn get(&self, i: usize) -> Vec3 {
        self.axis[i]
    }
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.axis[0] * a + self.axis[1] * b + self.axis[2] * c
    }
    pub fn local0(&self, a: Vec3) -> Vec3 {
        self.axis[0] * a.x + self.axis[1] * a.y + self.axis[2] * a.z
    }
    pub fn build_from_w(&mut self, n: &mut Vec3) {
        self.axis[2] = Vec3::unit_vector(*n);
        let a: Vec3;
        if self.axis[2].x.abs() > 0.9 {
            a = Vec3::new(0.0, 1.0, 0.0);
        } else {
            a = Vec3::new(1.0, 0.0, 0.0);
        };
        self.axis[1] = Vec3::unit_vector(Vec3::cross(self.axis[2], a));
        self.axis[0] = Vec3::cross(self.axis[2], self.axis[1]);
    }
}
