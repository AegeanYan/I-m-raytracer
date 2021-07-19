use crate::material::{Material, Metal};
use crate::{Ray, Vec3 ,AABB::Aabb , MovingSphere};
use std::sync::Arc;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,      //交点
    pub normal: Vec3, //法向量
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64, //距离
    pub u:f64,
    pub v:f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        if self.front_face == true {
            self.normal = outward_normal;
        } else {
            self.normal = Vec3::new(0.0, 0.0, 0.0) - outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self , time0:f64 , time1:f64 , output_box:& mut Aabb) ->bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl Sphere{
    pub fn get_sphere_uv(p:Vec3, u:&mut f64, v:&mut f64){
        let theta:f64 = (-p.y).acos();
        let phi:f64 = (-p.z).atan2(p.x) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}
impl Hittable for Sphere {

    fn hit(&self, r: Ray, t_min: f64, t_max: f64, mut rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig - self.center;
        let a: f64 = r.dir.length_squared();
        let half_b = Vec3::dot(r.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();
            let mut t: f64 = (-half_b - root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                Sphere::get_sphere_uv(outward_normal , &mut rec.u , &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            t = (-half_b + root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                Sphere::get_sphere_uv(outward_normal , &mut rec.u , &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        return false;
    }

    fn bounding_box(&self , time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = self.center - Vec3::new(self.radius , self.radius , self.radius);
        output_box.maximum = self.center + Vec3::new(self.radius , self.radius , self.radius);
        return true;
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, mut rec: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        let mut temp_rec = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: Arc::new(Metal::new()),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: true,
        };
        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.mat_ptr = temp_rec.mat_ptr.clone();
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        
        let mut temp_box:Aabb = Aabb {
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
        let mut first_box:bool = true;

        for object in self.objects.iter() {
            if !object.bounding_box(time0 , time1 , &mut temp_box) {return false;}
            if first_box {output_box.maximum = temp_box.maximum;
            output_box.minimum = temp_box.minimum;
            }else {
                output_box.minimum = MovingSphere::surrounding_box(*output_box, temp_box).minimum;
                output_box.maximum = MovingSphere::surrounding_box(*output_box , temp_box).maximum;
            }
            first_box = false;
        }
        return true;
    }
}
