use crate::hit::HitRecord;
use crate::material::Isotropiuc;
use crate::texture::{SolidColor, Texture};
use crate::Hittable;
use crate::Material;
use crate::AABB::Aabb;
use crate::{random_double, Ray, Vec3};
use std::f64::INFINITY;

pub struct ConstantMedium<T0: Hittable, T1: Material> {
    pub boundary: T0,
    pub phase_function: T1,
    pub neg_inv_density: f64,
}

impl<T0: Hittable, T1: Material> ConstantMedium<T0, T1> {
    #[warn(dead_code)]
    pub fn new0<T2: Texture>(b: T0, d: f64, a: T2) -> ConstantMedium<T0, Isotropiuc<T2>> {
        ConstantMedium {
            boundary: b,
            neg_inv_density: (-1.0 / d),
            phase_function: (Isotropiuc::new0(a)),
        }
    }
}
impl<T0: Hittable> ConstantMedium<T0, Isotropiuc<SolidColor>> {
    pub fn new(b: T0, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            phase_function: Isotropiuc::<SolidColor>::new(c),
            neg_inv_density: (-1.0 / d),
        }
    }
}
#[warn(clippy::question_mark)]
impl<T0: Hittable, T1: Material> Hittable for ConstantMedium<T0, T1> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rec1 = self.boundary.hit(r, -INFINITY, INFINITY);
        if rec1.is_none() {
            return None;
        }
        let mut rec1 = rec1.unwrap();
        let rec2 = self.boundary.hit(r, rec1.t + 0.0001, INFINITY);
        if rec2.is_none() {
            return None;
        };
        let mut rec2 = rec2.unwrap();
        if rec1.t < t_min {
            rec1.t = t_min;
        };
        if rec2.t > t_max {
            rec2.t = t_max;
        };
        if rec1.t >= rec2.t {
            return None;
        };
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        };
        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (random_double().ln());

        if hit_distance > distance_inside_boundary {
            return None;
        };
        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);
        Some(HitRecord {
            p,
            normal: Vec3::new(1.0, 0.0, 0.0),
            mat_ptr: &self.phase_function,
            t,
            u: 0.0,
            v: 0.0,
            front_face: true,
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }

    // fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
    //     //return random_double();
    //     return 0.1;
    // }
    // fn random(&self, o: Vec3) -> Vec3 {
    //     return Vec3::new(random_double(), random_double(), random_double());
    // }
}
