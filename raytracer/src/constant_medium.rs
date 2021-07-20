use crate::{rtweekend, Vec3, Ray, random_double};
use crate::Hittable;
use crate::Material;
use crate::texture::Texture;
use std::sync::Arc;
use crate::material::{Isotropiuc, Metal};
use crate::hit::HitRecord;
use crate::AABB::Aabb;
use std::f64::INFINITY;

pub struct ConstantMedium {
    pub boundary:Arc<dyn Hittable>,
    pub phase_function:Arc<dyn Material>,
    pub neg_inv_density:f64,
}

impl ConstantMedium{
    pub fn new0(b:Arc<dyn Hittable> , d:f64 , a:Arc<dyn Texture>)->Self{
        Self{
            boundary:b,
            neg_inv_density:(-1.0 / d),
            phase_function:Arc::new(Isotropiuc::new0(a)),
        }
    }
    pub fn new(b:Arc<dyn Hittable> , d:f64 , c:Vec3)->Self{
        Self{
            boundary:b,
            neg_inv_density:(-1.0 / d),
            phase_function:Arc::new(Isotropiuc::new(c)),
        }
    }
}

impl Hittable for ConstantMedium{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let enableDebug:bool = false;
        let debugging:bool = enableDebug && random_double() < 0.00001;

        let mut rec1:HitRecord = HitRecord {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            mat_ptr: Arc::new(Metal::new()),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false
        };
        let mut rec2:HitRecord= HitRecord {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            mat_ptr: Arc::new(Metal::new()),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false
        };

        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
          return false;
        };

        if !self.boundary.hit(r , rec1.t + 0.0001 , INFINITY , &mut rec2) {
            return false;
        };

        if debugging { std::println!("az")};

        if rec1.t < t_min {
            rec1.t = t_min;
        };
        if rec2.t > t_max {
            rec2.t = t_max;
        };

        if rec1.t >= rec2.t {
            return false;
        };

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        };

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().log(2.0);

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0 , 0.0 ,0.0);
        rec.front_face = true;
        rec.mat_ptr = self.phase_function.clone();

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        return self.boundary.bounding_box(time0 , time1 , output_box);
    }
}