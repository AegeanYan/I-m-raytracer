use crate::Ray;
use crate::Vec3;
use std::mem::swap;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        // for a in 0..3 {
        //     let t0:f64 = match a {
        //         0 => min((self.minimum.x - r.orig.x) / r.dir.x , (self.maximum.x - r.orig.x) / r.dir.x),
        //         1 => min((self.minimum.y - r.orig.y) / r.dir.y , (self.maximum.y - r.orig.y) / r.dir.y),
        //         2 => min((self.minimum.z - r.orig.z) / r.dir.z , (self.maximum.z - r.orig.z) / r.dir.z),
        //         _ => 0.0,
        //     };
        //     let t1:f64 = match a {
        //         0 => max((self.minimum.x - r.orig.x) / r.dir.x , (self.maximum.x - r.orig.x) / r.dir.x),
        //         1 => max((self.minimum.y - r.orig.y) / r.dir.y , (self.maximum.y - r.orig.y) / r.dir.y),
        //         2 => max((self.minimum.z - r.orig.z) / r.dir.z , (self.maximum.z - r.orig.z) / r.dir.z),
        //         _ => 0.0,
        //     };
        //     t_min = max(t0 , t_min);
        //     t_max = min(t1 , t_max);
        //     if t_max <= t_min {
        //         return false;
        //     }
        // }
        // return true;

        for a in 0..3 {
            let inv_d = match a {
                0 => 1.0 / r.dir.x,
                1 => 1.0 / r.dir.y,
                2 => 1.0 / r.dir.z,
                _ => 0.0,
            };
            let mut t0: f64 = match a {
                0 => (self.minimum.x - r.orig.x) * inv_d,
                1 => (self.minimum.y - r.orig.y) * inv_d,
                2 => (self.minimum.z - r.orig.z) * inv_d,
                _ => 0.0,
            };
            let mut t1: f64 = match a {
                0 => (self.maximum.x - r.orig.x) * inv_d,
                1 => (self.maximum.y - r.orig.y) * inv_d,
                2 => (self.maximum.z - r.orig.z) * inv_d,
                _ => 0.0,
            };
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }
}
