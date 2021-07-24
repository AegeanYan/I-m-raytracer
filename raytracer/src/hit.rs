use crate::material::{Material, Metal, Lambertian, NoMaterial};
use crate::{MovingSphere, Ray, Vec3, AABB::Aabb, degrees_to_radians, random_int};
use std::f64::consts::PI;
use std::sync::Arc;
use std::f64::INFINITY;
use image::imageops::overlay_bounds;
use crate::onb::Onb;
use crate::pdf::NoPdf;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,      //交点
    pub normal: Vec3, //法向量
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64, //距离
    pub u: f64,
    pub v: f64,
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
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
    fn pdf_value(&self , o:Vec3 , v:Vec3)->f64{
        return 0.0;
    }
    fn random(&self , o:Vec3)->Vec3{
        return Vec3::new(1.0,0.0,0.0);
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl Sphere {
    pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta: f64 = (-p.y).acos();
        let phi: f64 = (-p.z).atan2(p.x) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
    pub fn new(cen:Vec3 , r:f64 , m:Arc<dyn Material>)->Self{
        Self{
            center:cen,
            radius:r,
            mat_ptr:m,
        }
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
                Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            t = (-half_b + root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        return false;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = self.center - Vec3::new(self.radius, self.radius, self.radius);
        output_box.maximum = self.center + Vec3::new(self.radius, self.radius, self.radius);
        return true;
    }
    fn pdf_value(&self , o:Vec3 , v:Vec3)->f64{
        let mut rec:HitRecord = HitRecord{
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
            mat_ptr: Arc::new(NoMaterial{}),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: true
        };
        if !self.hit(Ray::new(o , v , 0.0) , 0.001 , INFINITY , &mut rec) {
            return 0.0;
        };
        let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - o).length_squared());
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        return 1.0 / solid_angle;
    }
    fn random(&self , o:Vec3)->Vec3{
        //println!("{:?}",o);
        let mut direction = self.center - o;
        let distance_squared = direction.length_squared();
        let mut uvw:Onb = Onb{
            axis: [Vec3::new(0.0 , 0.0 ,0.0);3]
        };
        uvw.build_from_w(&mut direction);

        return uvw.local0(Vec3::random_to_sphere(self.radius , distance_squared));
    }

}
#[derive(Default , Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new()->Self{
        Self{
            objects:vec![],
        }
    }
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
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.0,0.0,0.0))),
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

        let mut temp_box: Aabb = Aabb {
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
        let mut first_box: bool = true;

        for object in self.objects.iter() {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            if first_box {
                output_box.maximum = temp_box.maximum;
                output_box.minimum = temp_box.minimum;
            } else {
                output_box.minimum = MovingSphere::surrounding_box(*output_box, temp_box).minimum;
                output_box.maximum = MovingSphere::surrounding_box(*output_box, temp_box).maximum;
            }
            first_box = false;
        }
        return true;
    }
    fn pdf_value(&self , o:Vec3 , v:Vec3)->f64{
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in self.objects.iter() {
            sum += weight * object.pdf_value(o , v);
        };
        return sum;
    }
    fn random(&self , o:Vec3)->Vec3{
        let int_size = self.objects.len() as i32;
        return self.objects[random_int(0 , int_size - 1) as usize].random(o);
    }
}

pub struct Translate {
    pub ptr:Arc<dyn Hittable>,
    pub offset:Vec3,
}

impl Translate{
    pub fn new(p:Arc<dyn Hittable> , displacement:Vec3)->Self{
        Self{
            ptr: p.clone(),
            offset: displacement,
        }
    }
}

impl Hittable for Translate{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r:Ray = Ray{
            orig: r.orig - self.offset,
            dir: r.dir,
            time: r.time,
        };
        if !self.ptr.hit(moved_r.clone() , t_min ,t_max , rec) {
            return false;
        };
        rec.p += self.offset;
        rec.set_face_normal(moved_r.clone() , rec.normal);

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(time0 , time1 , output_box) {
            return false;
        };
        *output_box = Aabb::new(output_box.minimum + self.offset , output_box.maximum + self.offset);

        return true;
    }


}
pub struct RotateY {
    pub ptr:Arc<dyn Hittable>,
    pub sin_theta:f64,
    pub cos_theta:f64,
    pub hasbox:bool,
    pub bbox:Aabb,
}

impl RotateY{
    pub fn new(p:Arc<dyn Hittable> , angle:f64)->Self{
        let radians = degrees_to_radians(angle);
        let ptr = p.clone();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox:Aabb = Aabb {
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
        //let hass:bool = self.ptr.bounding_box(0.0, 1.0 , &mut self.bbox);
        let hasbox = ptr.bounding_box(0.0 , 1.0 ,&mut bbox);
        let mut min:Vec3 = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max:Vec3 = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = bbox.maximum.x * i as f64 + (1.0 - i as f64) * bbox.minimum.x;
                    let y = bbox.maximum.y * j as f64 + (1.0 - j as f64) * bbox.minimum.y;
                    let z = bbox.maximum.z * k as f64 + (1.0 - k as f64) * bbox.minimum.z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = - sin_theta * x + cos_theta * z;

                    let tester:Vec3 = Vec3::new(newx , y , newz);

                    for c in 0..3 {
                        match c {
                            0 => {
                                min.x = min.x.min(tester.x);
                                max.x = max.x.max(tester.x);
                            }
                            1 => {
                                min.y = min.y.min(tester.y);
                                max.y = max.y.max(tester.y);
                            }
                            2 => {
                                min.z = min.z.min(tester.z);
                                max.z = max.z.max(tester.z);
                            }
                            _ =>{

                            }
                        }
                    }
                }
            }
        }
        Self{
            ptr : ptr,
            sin_theta : sin_theta,
            cos_theta : cos_theta,
            hasbox : hasbox,
            bbox: Aabb::new(min , max),
        }
    }
}

impl Hittable for RotateY{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        origin.z = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;

        direction.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        direction.z = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;

        let rotated_r:Ray = Ray{
            orig: origin,
            dir: direction,
            time: r.time
        };
        if !self.ptr.hit(rotated_r , t_min , t_max , rec) {
            return false;
        };

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

        rec.p = p;
        rec.set_face_normal(rotated_r , normal);

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        return self.hasbox;
    }
}

pub struct FlipFace {
    pub ptr:Arc<dyn Hittable>,
}

impl FlipFace{
    pub fn new(p:Arc<dyn Hittable>)->Self{
        Self{
            ptr: p,
        }
    }
}

impl Hittable for FlipFace{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.ptr.hit(r , t_min , t_max , rec) {
            return false;
        };
        rec.front_face = !rec.front_face;
        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        return self.ptr.bounding_box(time0 , time1 , output_box);
    }
}