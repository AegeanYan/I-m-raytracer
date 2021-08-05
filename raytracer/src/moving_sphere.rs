use crate::hit::{HitRecord, Sphere};
use crate::material::Lambertian;
use crate::Hittable;
use crate::Material;
use crate::Vec3;
use crate::{Ray, AABB::Aabb};
use std::ops::Mul;

pub struct MovingSphere<T: Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: T,
}

impl<T: Material> MovingSphere<T> {
    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0);
    }
    pub fn new(cen0: Vec3, cen1: Vec3, time0: f64, time1: f64, r: f64, m: T) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0,
            time1,
            radius: r,
            mat_ptr: m,
        }
    }
    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
        let small: Vec3 = Vec3::new(
            box0.minimum.x.min(box1.minimum.x),
            box0.minimum.y.min(box1.minimum.y),
            box0.minimum.z.min(box1.minimum.z),
        );
        let big: Vec3 = Vec3::new(
            box0.maximum.x.max(box1.maximum.x),
            box0.maximum.y.max(box1.maximum.y),
            box0.maximum.z.max(box1.maximum.z),
        );
        let ab: Aabb = Aabb {
            minimum: small,
            maximum: big,
        };
        return ab;
    }
}

impl<T: Material> Hittable for MovingSphere<T> {
    #[allow(clippy::suspicious_operation_groupings)]
    #[warn(clippy::many_single_char_names)]
    fn hit(&self, rs: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = rs.orig - MovingSphere::center(self, rs.time);
        let ass: f64 = rs.dir.length_squared();
        let half_b = Vec3::dot(oc, rs.dir);
        let cok = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - ass * cok;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root: f64 = (-half_b - sqrtd) / ass;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / ass;
            if root < t_min || t_max < root {
                return None;
            }
        }

        // rec.t = root;
        // rec.p = r.at(rec.t);
        // let outward_normal: Vec3 = (rec.p - MovingSphere::center(self, r.time)) / self.radius;
        // rec.set_face_normal(r, outward_normal);
        // rec.mat_ptr = self.mat_ptr.clone();
        //
        // return true;
        let ti = root;
        let pi = rs.at(ti);
        let outward_normal = (pi - MovingSphere::center(self, rs.time)) / self.radius;
        let front_face = Vec3::dot(rs.dir, outward_normal.clone()) < 0.0;
        let mut flag = 1.0;
        if !front_face {
            flag = -1.0;
        };
        let mut ui = 0.0;
        let mut vi = 0.0;
        Sphere::<Lambertian>::get_sphere_uv(outward_normal, &mut ui, &mut vi);
        return Some(HitRecord {
            p: pi,
            normal: outward_normal.mul(flag),
            mat_ptr: &self.mat_ptr,
            t: ti,
            u: ui,
            v: vi,
            front_face,
        });
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        let mut box0: Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let mut box1: Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
        box0.minimum =
            MovingSphere::center(self, time0) - Vec3::new(self.radius, self.radius, self.radius);
        box0.maximum =
            MovingSphere::center(self, time0) + Vec3::new(self.radius, self.radius, self.radius);
        box1.minimum =
            MovingSphere::center(self, time1) - Vec3::new(self.radius, self.radius, self.radius);
        box1.maximum =
            MovingSphere::center(self, time1) + Vec3::new(self.radius, self.radius, self.radius);
        output_box.minimum = MovingSphere::<Lambertian>::surrounding_box(box0, box1).minimum;
        output_box.maximum = MovingSphere::<Lambertian>::surrounding_box(box0, box1).maximum;
        return true;
    }
}
