use crate::{rtweekend, random_double};
use crate::Ray;
use crate::hit;
use crate::Vec3;
use crate::hit::HitRecord;

pub trait Material{
    fn scatter(&self , r_in:Ray , rec:hit::HitRecord , attenuation:&mut Vec3 , scattered:&mut Ray)->bool;
}

pub struct Lambertian {
    pub albedo:Vec3,
}

impl Lambertian{
    pub fn new(a:Vec3)->Self{
        Self{
            albedo:a,
        }
    }
}

impl Material for Lambertian{
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation:&mut Vec3, scattered:&mut Ray) -> bool {
        let scatter_direction:Vec3 = rec.normal + Vec3::random_unit_vector();
        let ra = Ray{
            orig: rec.p,
            dir: scatter_direction
        };
        scattered.dir = ra.dir;
        scattered.orig = ra.orig;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        return true;
    }
}



pub struct Metal{
    pub albedo:Vec3,
    pub fuzz:f64,
}

impl Material for Metal{
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected:Vec3 = Vec3::reflect(Vec3::unit_vector(r_in.dir) , rec.normal);
        let ra = Ray{
            orig: rec.p,
            dir: reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        };
        scattered.dir = ra.dir;
        scattered.orig = ra.orig;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        return (Vec3::dot(scattered.dir , rec.normal) > 0.0);
    }
}

impl Metal{
    pub fn new()->Self{
        Self{
            fuzz:0.0,
            albedo:Vec3::new(0.0 , 0.0 , 0.0),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ref_idx:f64,
}

impl Material for Dielectric{
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;

        let etai_over_etat:f64;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx;
        }else {
            etai_over_etat = self.ref_idx;
        }
        let unit_direction:Vec3 = Vec3::unit_vector(r_in.dir);
        let cos_theta:f64;
        if Vec3::dot(unit_direction * -1.0 , rec.normal) < 1.0{
            cos_theta = Vec3::dot(unit_direction * -1.0 , rec.normal);
        }else {
            cos_theta = 1.0;
        }
        let sin_theta:f64 = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected:Vec3 = Vec3::reflect(unit_direction , rec.normal);
            scattered.orig = rec.p;
            scattered.dir = reflected;
            return true;
        }
        let reflect_prob = Dielectric::schlick(cos_theta , etai_over_etat);
        if random_double() < reflect_prob{
            let reflected:Vec3 = Vec3::reflect(unit_direction , rec.normal);
            scattered.orig = rec.p;
            scattered.dir = reflected;
            return true;
        }
        let refracted:Vec3 = Vec3::refract(unit_direction , rec.normal , etai_over_etat);
        scattered.orig = rec.p;
        scattered.dir = refracted;
        return true;
    }
}
impl Dielectric{
    pub fn schlick(cosine:f64 , ref_idx:f64)->f64{
        let mut r0:f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}


