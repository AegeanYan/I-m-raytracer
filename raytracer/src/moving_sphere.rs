use crate::hit::HitRecord;
use crate::material;
use crate::Hittable;
use crate::Material;
use crate::Vec3;
use crate::{hit, Ray , AABB::Aabb};
use std::sync::Arc;
use std::cmp::{min, max};

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn material::Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0);
    }

    pub fn surrounding_box(box0:Aabb , box1:Aabb)->Aabb{
        let small:Vec3 = Vec3::new(box0.minimum.x.min(box1.minimum.x) , box0.minimum.y.min(box1.minimum.y) , box0.minimum.z.min(box1.minimum.z));
        let big:Vec3 = Vec3::new(box0.maximum.x.max(box1.maximum.x) , box0.maximum.y.max(box1.maximum.y) , box0.maximum.z.max(box1.maximum.z));
        let ab:Aabb = Aabb{
            minimum: small,
            maximum: big
        };
        return ab;
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig - MovingSphere::center(self, r.time);
        let a: f64 = r.dir.length_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - MovingSphere::center(self, r.time)) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        let mut box0:Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };
        let mut box1:Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };
        box0.minimum = MovingSphere::center(self ,time0) - Vec3::new(self.radius , self.radius , self.radius);
        box0.maximum = MovingSphere::center(self , time0) + Vec3::new(self.radius , self.radius , self.radius);
        box1.minimum = MovingSphere::center(self ,time1) - Vec3::new(self.radius , self.radius , self.radius);
        box1.maximum = MovingSphere::center(self , time1) + Vec3::new(self.radius , self.radius , self.radius);
        output_box.minimum = MovingSphere::surrounding_box(box0 , box1).minimum;
        output_box.maximum = MovingSphere::surrounding_box(box0 , box1).maximum;
        return true;
    }
}
