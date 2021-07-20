use crate::hit::HitRecord;
use crate::material::Material;
use crate::Hittable;
use crate::Vec3;
use crate::AABB::Aabb;
use crate::{rtweekend, Ray};
use std::sync::Arc;

pub struct XyRect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}
impl Hittable for XyRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        return true;
    }
}

pub struct XzRect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}
impl Hittable for XzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        );
        return true;
    }
}
pub struct YzRect {
    pub mp: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> Self {
        Self{
            mp: mat,
            y0,
            y1,
            z0,
            z1,
            k
        }
    }
}
impl Hittable for YzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        );
        return true;
    }
}