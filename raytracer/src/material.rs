use crate::hit;
use crate::hit::HitRecord;
use crate::random_double;
use crate::texture::SolidColor;
use crate::texture::Texture;
use crate::Ray;
use crate::Vec3;
use std::sync::Arc;
use std::f64::consts::PI;
use imageproc::math::l1_norm;
use crate::onb::Onb;
use crate::pdf::random_cosine_direction;

pub trait Material {
    fn scatter(
        &self,
        r_in:&mut Ray,
        rec:&mut hit::HitRecord,
        albedo: &mut Vec3,
        scattered: &mut Ray,
        pdf:&mut f64
    ) -> bool{
        return false;
    }
    fn scattering_pdf(&self , r_in:&mut Ray , rec:&mut hit::HitRecord , scattered:&mut Ray)->f64{
        return 0.0;
    }
    fn emitted(&self, r_in:&mut Ray , rec:&mut hit::HitRecord , u: f64, v: f64, p: &mut Vec3) -> Vec3 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn news(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
    pub fn new(a: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
}

impl Material for Lambertian {
    fn scattering_pdf(&self , r_in:&mut Ray , rec:&mut hit::HitRecord , scattered:&mut Ray)->f64{
        let cosine = Vec3::dot(rec.normal , Vec3::unit_vector(scattered.dir));
        if cosine < 0.0 {
            return 0.0;
        }else {
            return cosine / PI;
        }
    }
    fn scatter(
        &self,
        r_in:&mut Ray,
        mut rec:&mut HitRecord,
        mut albedo: &mut Vec3,
        scattered: &mut Ray,
        pdf:&mut f64,
    ) -> bool {
        // let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.normal;
        // }
        // let ra = Ray {
        //     orig: rec.p,
        //     dir: Vec3::unit_vector(scatter_direction),
        //     time: r_in.time,
        // };
        // scattered.dir = ra.dir;
        // scattered.orig = ra.orig;
        // scattered.time = ra.time;
        // albedo.x = self.albedo.value(rec.u , rec.v , &mut rec.p).x;
        // albedo.y = self.albedo.value(rec.u , rec.v , &mut rec.p).y;
        // albedo.z = self.albedo.value(rec.u , rec.v , &mut rec.p).z;
        // *pdf = (Vec3::dot(rec.normal, scattered.dir) / PI);
        //
        // return true;

        //another activision

        // let direction = Vec3::random_in_hemisphere(rec.normal);
        // let ra = Ray{
        //     orig: rec.p,
        //     dir: Vec3::unit_vector(direction),
        //     time: r_in.time,
        // };
        //
        // scattered.orig = ra.orig;
        // scattered.dir = ra.dir;
        // scattered.time = ra.time;
        //
        // albedo.x = self.albedo.value(rec.u , rec.v , &mut rec.p).x;
        // albedo.y = self.albedo.value(rec.u , rec.v , &mut rec.p).y;
        // albedo.z = self.albedo.value(rec.u , rec.v , &mut rec.p).z;
        //
        // *pdf = 0.5 / PI;
        // return true;

        //

        let mut uvw:Onb = Onb { axis: [Vec3::new(0.0,0.0,0.0) , Vec3::new(0.0,0.0,0.0) , Vec3::new(0.0,0.0,0.0)] };
        uvw.build_from_w(&mut rec.normal);
        let direction = uvw.local0(random_cosine_direction());
        let ra = Ray{
            orig: rec.p,
            dir: Vec3::unit_vector(direction),
            time: r_in.time,
        };
        scattered.orig = ra.orig;
        scattered.dir = ra.dir;
        scattered.time = ra.time;

        albedo.x = self.albedo.value(rec.u , rec.v , &mut rec.p).x;
        albedo.y = self.albedo.value(rec.u , rec.v , &mut rec.p).y;
        albedo.z = self.albedo.value(rec.u , rec.v , &mut rec.p).z;

        *pdf = Vec3::dot(uvw.axis[2] , scattered.dir) / PI;
        return true;
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in:&mut Ray,
        rec:&mut  HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        pdf:&mut f64
    ) -> bool {
        let reflected: Vec3 = Vec3::reflect(Vec3::unit_vector(r_in.dir), rec.normal);
        let ra = Ray {
            orig: rec.p,
            dir: reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            time: r_in.time,
        };
        scattered.dir = ra.dir;
        scattered.orig = ra.orig;
        scattered.time = ra.time;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        return Vec3::dot(scattered.dir, rec.normal) > 0.0;
    }
}

impl Metal {
    pub fn new() -> Self {
        Self {
            fuzz: 0.0,
            albedo: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn news(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(
        //我好像把几种方式重在一起了，之后可能会出问题
        &self,
        r_in:&mut Ray,
        rec:&mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        pdf:&mut f64
    ) -> bool {
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;
        scattered.time = r_in.time;
        let etai_over_etat: f64;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx;
        } else {
            etai_over_etat = self.ref_idx;
        }
        let unit_direction: Vec3 = Vec3::unit_vector(r_in.dir);
        let cos_theta: f64;
        if Vec3::dot(unit_direction * -1.0, rec.normal) < 1.0 {
            cos_theta = Vec3::dot(unit_direction * -1.0, rec.normal);
        } else {
            cos_theta = 1.0;
        }
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected: Vec3 = Vec3::reflect(unit_direction, rec.normal);
            scattered.orig = rec.p;
            scattered.dir = reflected;
            return true;
        }
        let reflect_prob = Dielectric::schlick(cos_theta, etai_over_etat);
        if random_double() < reflect_prob {
            let reflected: Vec3 = Vec3::reflect(unit_direction, rec.normal);
            scattered.orig = rec.p;
            scattered.dir = reflected;
            return true;
        }
        let refracted: Vec3 = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        scattered.orig = rec.p;
        scattered.dir = refracted;
        return true;
    }
}
impl Dielectric {
    pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }

    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx: ref_idx }
    }
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(a: Arc<dyn Texture>) -> Self {
        Self { emit: a }
    }

    pub fn new0(c: Vec3) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in:&mut Ray,
        rec:&mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        pdf:&mut f64
    ) -> bool {
        return false;
    }

    fn emitted(&self,r_in:&mut Ray , rec:&mut hit::HitRecord ,  u: f64, v: f64, p: &mut Vec3) -> Vec3 {
        if rec.front_face {
            return self.emit.value(u, v, p);
        }else {
            return Vec3::new(0.0,0.0,0.0);
        }
    }
}

pub struct Isotropiuc {
    pub albedo:Arc<dyn Texture>,
}

impl Isotropiuc{
    pub fn new0(a:Arc<dyn Texture>)->Self{
        Self{
            albedo:a,
        }
    }

    pub fn new(c:Vec3)->Self{
        Self{
            albedo:Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropiuc{
    fn scatter(&self, r_in:&mut Ray, mut rec:&mut HitRecord, mut attenuation: &mut Vec3, mut scattered: &mut Ray , pdf:&mut f64) -> bool {
        scattered.orig = rec.p;
        scattered.dir = Vec3::random_in_unit_sphere();
        scattered.time = r_in.time;

        attenuation.x = self.albedo.value(rec.u, rec.v, &mut rec.p).x;
        attenuation.y = self.albedo.value(rec.u, rec.v, &mut rec.p).y;
        attenuation.z = self.albedo.value(rec.u, rec.v, &mut rec.p).z;
        return true;
    }
}
