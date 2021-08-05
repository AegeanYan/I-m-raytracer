use crate::hit::HitRecord;
use crate::material::Material;
use crate::Vec3;
use crate::AABB::Aabb;
use crate::{random_double_lim, Hittable};
use crate::{rtweekend::*, Ray};
use std::f64::INFINITY;
use std::ops::Mul;

pub struct XyRect<T: Material + Clone> {
    pub mp: T,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl<T: Material + Clone> XyRect<T> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: T) -> Self {
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
impl<T: Material + Clone> Hittable for XyRect<T> {
    #[warn(clippy::many_single_char_names)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ti = (self.k - r.orig.z) / r.dir.z;
        if ti < t_min || ti > t_max {
            return None;
        }
        let xi = r.orig.x + ti * r.dir.x;
        let yi = r.orig.y + ti * r.dir.y;
        if xi < self.x0 || xi > self.x1 || yi < self.y0 || yi > self.y1 {
            return None;
        }
        let ui = (xi - self.x0) / (self.x1 - self.x0);
        let vi = (yi - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let pi = r.at(ti);
        let front_face = Vec3::dot(r.dir, outward_normal.clone()) < 0.0;
        let mut flag = -1.0;
        if front_face {
            flag = 1.0;
        }
        Some(HitRecord {
            p: pi,
            normal: outward_normal.mul(flag),
            mat_ptr: &self.mp,
            t: ti,
            u: ui,
            v: vi,
            front_face,
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        return true;
    }
}

pub struct XzRect<T: Material + Clone> {
    pub mp: T,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl<T: Material + Clone> XzRect<T> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: T) -> Self {
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
impl<T: Material + Clone> Hittable for XzRect<T> {
    #[warn(clippy::many_single_char_names)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ti = (self.k - r.orig.y) / r.dir.y;
        if ti < t_min || ti > t_max {
            return None;
        }
        let xi = r.orig.x + ti * r.dir.x;
        let zi = r.orig.z + ti * r.dir.z;
        if xi < self.x0 || xi > self.x1 || zi < self.z0 || zi > self.z1 {
            return None;
        }
        let ui = (xi - self.x0) / (self.x1 - self.x0);
        let vi = (zi - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let pi = r.at(ti);
        let front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        let mut flag = -1.0;
        if front_face {
            flag = 1.0;
        }
        Some(HitRecord {
            p: pi,
            normal: outward_normal.mul(flag),
            mat_ptr: &self.mp,
            t: ti,
            u: ui,
            v: vi,
            front_face,
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        );
        return true;
    }
    #[warn(unused_assignments)]
    fn pdf_value(&self, origin: Vec3, v: Vec3) -> f64 {
        // let mut rec: HitRecord = HitRecord {
        //     p: Vec3 {
        //         x: 0.0,
        //         y: 0.0,
        //         z: 0.0,
        //     },
        //     normal: Vec3 {
        //         x: 0.0,
        //         y: 0.0,
        //         z: 0.0,
        //     },
        //     mat_ptr: &Metal::new(),
        //     t: 0.0,
        //     u: 0.0,
        //     v: 0.0,
        //     front_face: false,
        // };
        let mut rec: HitRecord;
        match self.hit(Ray::new(origin, v, 0.0), 0.001, INFINITY) {
            Some(rec_) => {
                rec = rec_;
            }
            None => {
                return 0.0;
            }
        }
        let area = (self.x1 - self.x0) * (self.z1 - self.x0);
        let distance_squared = rec.t * rec.t * v.length_squared();
        let cosine = (Vec3::dot(v, rec.normal) / v.length()).abs();

        return distance_squared / (cosine * area);
    }

    fn random(&self, origin: Vec3) -> Vec3 {
        let random_point = Vec3::new(
            random_double_lim(self.x0, self.x1),
            self.k,
            random_double_lim(self.z0, self.z1),
        );
        return random_point - origin;
    }
}
pub struct YzRect<T: Material + Clone> {
    pub mp: T,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl<T: Material + Clone> YzRect<T> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: T) -> Self {
        Self {
            mp: mat,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}
impl<T: Material + Clone> Hittable for YzRect<T> {
    #[warn(clippy::many_single_char_names)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ti = (self.k - r.orig.x) / r.dir.x;
        if ti < t_min || ti > t_max {
            return None;
        }
        let yi = r.orig.y + ti * r.dir.y;
        let zi = r.orig.z + ti * r.dir.z;
        if yi < self.y0 || yi > self.y1 || zi < self.z0 || zi > self.z1 {
            return None;
        }
        let ui = (yi - self.y0) / (self.y1 - self.y0);
        let vi = (zi - self.z0) / (self.z1 - self.z0);
        let pi = r.at(ti);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let front_face = Vec3::dot(r.dir, outward_normal.clone()) < 0.0;
        let mut flag = -1.0;
        if front_face {
            flag = 1.0;
        }
        Some(HitRecord {
            p: pi,
            normal: outward_normal.mul(flag),
            mat_ptr: &self.mp,
            t: ti,
            u: ui,
            v: vi,
            front_face,
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        );
        return true;
    }
}
#[derive(Copy, Clone)]
pub struct Triangle<T: Material> {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub mat_ptr: T,
}
impl<T: Material> Triangle<T> {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, mat_ptr: T) -> Self {
        Self {
            p0,
            p1,
            p2,
            mat_ptr,
        }
    }
}
impl<T: Material> Hittable for Triangle<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let dir1 = self.p1 - self.p0;
        let dir2 = self.p2 - self.p0;
        let normal = Vec3::unit_vector(Vec3::cross(dir1, dir2)); //法向量
        let tmp = self.p0 - r.orig;
        let t = Vec3::dot(normal, tmp) / Vec3::dot(normal, r.dir);
        if t < t_min || t > t_max {
            return None;
        };
        let r_ = r.at(t);
        if Vec3::same_side(self.p0, self.p1, self.p2, r_)
            && Vec3::same_side(self.p1, self.p2, self.p0, r_)
            && Vec3::same_side(self.p2, self.p0, self.p1, r_)
        {
            let outward_normal = normal;
            let front_face = Vec3::dot(outward_normal, r.dir) < 0.0;
            let mut flag = 1.0;
            if !front_face {
                flag = -1.0;
            };
            let v_ab = self.p1 - self.p0;
            let v_bc = self.p2 - self.p1;
            let v_ap = r_ - self.p0;
            let v_bp = r_ - self.p1;

            let mut u = 0.0;
            let mut v = 0.0;
            get_triangle_uv(v_ab, v_bc, v_ap, v_bp, &mut u, &mut v);
            Some(HitRecord {
                p: r_,
                normal: outward_normal * flag,
                mat_ptr: &(self.mat_ptr),
                t,
                u,
                v,
                front_face,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = Vec3::new(
            f_3_min(self.p0.x, self.p1.x, self.p2.x),
            f_3_min(self.p0.y, self.p1.y, self.p2.y),
            f_3_min(self.p0.z, self.p1.z, self.p2.z),
        ) + Vec3::new(0.01, 0.01, 0.01);
        output_box.maximum = Vec3::new(
            f_3_max(self.p0.x, self.p1.x, self.p2.x),
            f_3_max(self.p0.y, self.p1.y, self.p2.y),
            f_3_max(self.p0.z, self.p1.z, self.p2.z),
        ) - Vec3::new(0.01, 0.01, 0.01);
        return true;
    }
}
