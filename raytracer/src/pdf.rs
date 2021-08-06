use crate::hit::Hittable;
use crate::onb::Onb;
use crate::{random_double, Vec3};
use std::f64::consts::PI;

pub trait Pdf: Sync + Send {
    fn value(&self, direction: &mut Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub struct CosinePdf {
    pub uvw: Onb,
}

impl CosinePdf {
    pub fn new(mut w: Vec3) -> Self {
        let mut abc: Onb = Onb {
            axis: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            ],
        };
        abc.build_from_w(&mut w);
        Self { uvw: abc }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &mut Vec3) -> f64 {
        let cosine = Vec3::dot(Vec3::unit_vector(*direction), self.uvw.axis[2]);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local0(random_cosine_direction())
    }
}

pub struct HittablePdf<'a, T: Hittable> {
    pub o: Vec3,
    pub ptr: &'a T,
}

impl<'a, T: Hittable> HittablePdf<'a, T> {
    pub fn new(p: &'a T, origin: Vec3) -> Self {
        Self { ptr: p, o: origin }
    }
}

impl<'a, T: Hittable> Pdf for HittablePdf<'a, T> {
    fn value(&self, direction: &mut Vec3) -> f64 {
        self.ptr.pdf_value(self.o, *direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}

pub struct MixturePdf<'a, T1: Pdf, T2: Pdf> {
    //pub p: [Arc<dyn Pdf>; 2],
    pub p1: &'a T1,
    pub p2: &'a T2,
}

impl<'a, T1: Pdf, T2: Pdf> MixturePdf<'a, T1, T2> {
    pub fn new(p1: &'a T1, p2: &'a T2) -> Self {
        Self { p1, p2 }
    }
}

impl<'a, T1: Pdf, T2: Pdf> Pdf for MixturePdf<'a, T1, T2> {
    fn value(&self, direction: &mut Vec3) -> f64 {
        self.p1.value(direction) * 0.5 + self.p2.value(direction) * 0.5
    }

    fn generate(&self) -> Vec3 {
        let vv: Vec3;
        if random_double() < 0.5 {
            vv = self.p1.generate();
        } else {
            vv = self.p2.generate();
        }
        vv
    }
}

pub struct NoPdf {}
impl Pdf for NoPdf {
    fn value(&self, _direction: &mut Vec3) -> f64 {
        unreachable!()
    }

    fn generate(&self) -> Vec3 {
        unreachable!()
    }
}
