use crate::{Vec3, random_double};
use std::f64::consts::PI;
use crate::onb::Onb;
use std::sync::Arc;
use crate::hit::Hittable;

pub trait Pdf{
    fn value(&self , direction:&mut Vec3)->f64;
    fn generate(&self)->Vec3;
}

pub fn random_cosine_direction()->Vec3{
    let r1 = random_double();
    let r2 = random_double();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    return Vec3::new(x , y ,z);
}

pub struct CosinePdf {
    pub uvw:Onb,
}

impl CosinePdf{
    pub fn new(mut w:Vec3) ->Self{
        let mut abc:Onb = Onb { axis: [Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,0.0,0.0),Vec3::new(0.0,0.0,0.0)] };
        abc.build_from_w(&mut w);
        Self{
            uvw:abc,
        }
    }
}

impl Pdf for CosinePdf{
    fn value(&self, direction: &mut Vec3) -> f64 {
        let cosine = Vec3::dot(Vec3::unit_vector(*direction) , self.uvw.axis[2]);
        if cosine <= 0.0 {
            return 0.0;
        }else {
            return cosine / PI;
        }
    }

    fn generate(&self) -> Vec3 {
        return self.uvw.local0(random_cosine_direction());
    }
}

pub struct HittablePdf {
    pub o:Vec3,
    pub ptr:Arc<dyn Hittable>,
}

impl HittablePdf{
    pub fn new(p:Arc<dyn Hittable> , origin:Vec3)->Self{
        Self{
            ptr:p,
            o: origin,
        }
    }
}

impl Pdf for HittablePdf{
    fn value(&self, direction: &mut Vec3) -> f64 {
        return self.ptr.pdf_value(self.o , *direction);
    }

    fn generate(&self) -> Vec3 {
        return self.ptr.random(self.o);
    }
}

pub struct MixturePdf {
    pub p:[Arc<dyn Pdf>;2],
}

impl MixturePdf{
    pub fn new(p0:Arc<dyn Pdf> , p1:Arc<dyn Pdf>)->Self{
        Self{
            p:[p0,p1],
        }
    }
}

impl Pdf for MixturePdf{
    fn value(&self, direction: &mut Vec3) -> f64 {
        self.p[0].value(direction) * 0.5 + self.p[1].value(direction) * 0.5
    }



    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            return self.p[0].generate();
        }else {
            return self.p[1].generate();
        }
    }
}

pub struct NoPdf{}
impl Pdf for NoPdf{
    fn value(&self, direction: &mut Vec3) -> f64 {
        unreachable!()
    }

    fn generate(&self) -> Vec3 {
        unreachable!()
    }
}