use crate::material::{Lambertian, Material, Metal};
use crate::onb::Onb;
use crate::{degrees_to_radians, random_int, MovingSphere, Ray, Vec3, AABB::Aabb};
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Vec3,      //交点
    pub normal: Vec3, //法向量
    pub mat_ptr: &'a dyn Material,
    pub t: f64, //距离
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        if self.front_face == true {
            self.normal = outward_normal;
        } else {
            self.normal = Vec3::new(0.0, 0.0, 0.0) - outward_normal;
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
    fn pdf_value(&self, _o: Vec3, _v: Vec3) -> f64 {
        return 0.0;
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        return Vec3::new(1.0, 0.0, 0.0);
    }
}

#[derive(Copy, Clone)]
pub struct Sphere<T: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: T,
}
impl<T: Material> Sphere<T> {
    #[warn(clippy::deref_addrof)]
    pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta: f64 = (-p.y).acos();
        let phi: f64 = (-p.z).atan2(p.x) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
    pub fn new(cen: Vec3, r: f64, m: T) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}
impl<T: Material> Hittable for Sphere<T> {
    #[allow(clippy::suspicious_operation_groupings)]
    #[warn(clippy::many_single_char_names)]
    fn hit(&self, rs: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = rs.orig - self.center;
        let ai: f64 = rs.dir.length_squared();
        let half_b = Vec3::dot(rs.dir, oc);
        let ci = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - ai * ci;
        if discriminant <= 0.0 {
            return None;
        }
        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();
            let mut ti: f64 = (-half_b - root) / ai;
            if ti > t_min && ti < t_max {
                let p = rs.at(ti);
                let outward_normal = p.sub(self.center.clone()).div(self.radius);
                let mut ui = 0.0;
                let mut vi = 0.0;
                Sphere::<Metal>::get_sphere_uv(outward_normal, &mut ui, &mut vi);
                let front_face = Vec3::dot(rs.dir, outward_normal) < 0.0;
                let mut flag = 1.0;
                if !front_face {
                    flag = -1.0;
                }
                return Option::from(HitRecord {
                    p,
                    normal: outward_normal.mul(flag),
                    mat_ptr: &(self.mat_ptr),
                    t: ti,
                    u: ui,
                    v: vi,
                    front_face,
                });
            }
            ti = (-half_b + root) / ai;
            if ti > t_min && ti < t_max {
                let pi = rs.at(ti);
                let outward_normal = pi.sub(self.center.clone()).div(self.radius);
                let mut ui = 0.0;
                let mut vi = 0.0;
                Sphere::<Metal>::get_sphere_uv(outward_normal, &mut ui, &mut vi);
                let front_face = Vec3::dot(rs.dir, outward_normal) < 0.0;
                let mut flag = 1.0;
                if !front_face {
                    flag = -1.0;
                }
                return Option::from(HitRecord {
                    p: pi,
                    normal: outward_normal.mul(flag),
                    mat_ptr: &(self.mat_ptr),
                    t: ti,
                    u: ui,
                    v: vi,
                    front_face,
                });
            }
        }
        return None;
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = self.center - Vec3::new(self.radius, self.radius, self.radius);
        output_box.maximum = self.center + Vec3::new(self.radius, self.radius, self.radius);
        return true;
    }
    #[warn(unused_assignments)]
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        // if !self.hit(Ray::new(o, v, 0.0), 0.001, INFINITY, &mut rec) {
        //     return 0.0;
        // };
        match self.hit(Ray::new(o, v, 0.0), 0.001, INFINITY) {
            Some(rec_) => {}
            None => {
                return 0.0;
            }
        }
        let cos_theta_max = 1.0 - self.radius * self.radius / (self.center - o).length_squared();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        return 1.0 / solid_angle;
    }
    fn random(&self, o: Vec3) -> Vec3 {
        //println!("{:?}",o);
        let mut direction = self.center - o;
        let distance_squared = direction.length_squared();
        let mut uvw: Onb = Onb {
            axis: [Vec3::new(0.0, 0.0, 0.0); 3],
        };
        uvw.build_from_w(&mut direction);
        if distance_squared == 0.0 {
            return Vec3::new(1.0, 1.0, 1.0);
        }
        return uvw.local0(Vec3::random_to_sphere(self.radius, distance_squared));
    }
}
#[allow(clippy::float_cmp)]
#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let mut hit_anything: bool = false;
        // let mut closest_so_far: f64 = t_max;
        // let mut temp_rec = HitRecord {
        //     p: Vec3::new(0.0, 0.0, 0.0),
        //     normal: Vec3::new(0.0, 0.0, 0.0),
        //     mat_ptr: &Lambertian::new(Vec3::new(0.0, 0.0, 0.0)),
        //     t: 0.0,
        //     u: 0.0,
        //     v: 0.0,
        //     front_face: true,
        // };
        // for object in self.objects.iter() {
        //     if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
        //         hit_anything = true;
        //         closest_so_far = temp_rec.t;
        //         // rec.p = temp_rec.p;
        //         // rec.t = temp_rec.t;
        //         // rec.mat_ptr = temp_rec.mat_ptr.clone();
        //         // rec.normal = temp_rec.normal;
        //         // rec.front_face = temp_rec.front_face;
        //         *rec = temp_rec.clone();
        //     }
        // }
        // return hit_anything;
        let mut rec_mid: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Some(tmp) = object.hit(r, t_min, closest_so_far) {
                rec_mid = Some(tmp.clone());
                closest_so_far = tmp.t;
            }
        }
        rec_mid
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
                output_box.minimum =
                    MovingSphere::<Lambertian>::surrounding_box(*output_box, temp_box).minimum;
                output_box.maximum =
                    MovingSphere::<Lambertian>::surrounding_box(*output_box, temp_box).maximum;
            }
            first_box = false;
        }
        return true;
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in self.objects.iter() {
            sum += weight * object.pdf_value(o, v);
        }
        return sum;
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        if int_size == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let ran = random_int(0, int_size - 1) as usize;
        let vv = (*self.objects[ran]).random(o);
        return vv;
    }
}

pub struct Translate<T: Hittable> {
    pub ptr: T,
    pub offset: Vec3,
}

impl<T: Hittable> Translate<T> {
    pub fn new(p: T, displacement: Vec3) -> Self {
        Self {
            ptr: p,
            offset: displacement,
        }
    }
}

impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let moved_r: Ray = Ray {
        //     orig: r.orig - self.offset,
        //     dir: r.dir,
        //     time: r.time,
        // };
        // if !self.ptr.hit(moved_r.clone(), t_min, t_max, rec) {
        //     return false;
        // };
        // rec.p += self.offset;
        // rec.set_face_normal(moved_r.clone(), rec.normal);
        //
        // return true;
        let moved_r = Ray::new(r.orig.sub(self.offset.clone()), r.dir.clone(), r.time);
        match self.ptr.hit(moved_r, t_min, t_max) {
            Some(rec) => {
                let front_face = Vec3::dot(r.dir, rec.normal) < 0.0;
                let mut flag = 1.0;
                if !front_face {
                    flag = -1.0;
                }
                Some(HitRecord {
                    p: rec.p.add(self.offset.clone()),
                    normal: rec.normal.mul(flag),
                    front_face,
                    ..rec
                })
            }
            None => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        };
        *output_box = Aabb::new(
            output_box.minimum + self.offset,
            output_box.maximum + self.offset,
        );

        return true;
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        self.ptr.pdf_value(o - self.offset, v)
    }
    fn random(&self, o: Vec3) -> Vec3 {
        self.ptr.random(o - self.offset)
    }
}
pub struct RotateY<T: Hittable> {
    pub ptr: T,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: Aabb,
}

impl<T: Hittable> RotateY<T> {
    pub fn new(p: T, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox: Aabb = Aabb {
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
        //let hass:bool = self.ptr.bounding_box(0.0, 1.0 , &mut self.bbox);
        let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);
        let mut min: Vec3 = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max: Vec3 = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = bbox.maximum.x * i as f64 + (1.0 - i as f64) * bbox.minimum.x;
                    let y = bbox.maximum.y * j as f64 + (1.0 - j as f64) * bbox.minimum.y;
                    let z = bbox.maximum.z * k as f64 + (1.0 - k as f64) * bbox.minimum.z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester: Vec3 = Vec3::new(newx, y, newz);

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
                            _ => {}
                        }
                    }
                }
            }
        }
        Self {
            ptr: p,
            sin_theta,
            cos_theta,
            hasbox,
            bbox: Aabb::new(min, max),
        }
    }
}

impl<T: Hittable> Hittable for RotateY<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        origin.z = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;

        direction.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        direction.z = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;

        let rotated_r: Ray = Ray {
            orig: origin,
            dir: direction,
            time: r.time,
        };
        match self.ptr.hit(rotated_r, t_min, t_max) {
            Some(rec) => {
                let mut p = rec.p.clone();
                let mut normal = rec.normal.clone();
                p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
                p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
                normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
                normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
                let front_face = Vec3::dot(rotated_r.dir, normal.clone()) < 0.0;
                let mut flag = -1.0;
                if front_face {
                    flag = 1.0;
                }
                Some(HitRecord {
                    p,
                    normal: normal.mul(flag),
                    front_face,
                    ..rec
                })
            }
            None => None,
        }
        // if !self.ptr.hit(rotated_r, t_min, t_max, rec) {
        //     return false;
        // };
        //
        // let mut p = rec.p;
        // let mut normal = rec.normal;
        //
        // p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        // p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
        //
        // normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        // normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
        //
        // rec.p = p;
        // rec.set_face_normal(rotated_r, normal);
        //
        // return true;
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        return self.hasbox;
    }
}

pub struct FlipFace<T: Hittable> {
    pub ptr: T,
}

impl<T: Hittable> FlipFace<T> {
    pub fn new(p: T) -> Self {
        Self { ptr: p }
    }
}

impl<T: Hittable> Hittable for FlipFace<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // if !self.ptr.hit(r, t_min, t_max) {
        //     return None;
        // };
        // rec.front_face = !rec.front_face;
        // return true;
        match self.ptr.hit(r, t_min, t_max) {
            Some(mut rec) => {
                rec.front_face = !rec.front_face;
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        return self.ptr.bounding_box(time0, time1, output_box);
    }
}
