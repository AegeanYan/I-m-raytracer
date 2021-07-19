use crate::rtweekend;
use crate::Vec3;
use std::sync::Arc;
use crate::perlin;
use crate::perlin::Perlin;

pub trait Texture {
    fn value(&self , u:f64 , v:f64 , p:&mut Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color_value:Vec3,
}
impl SolidColor{
    pub fn new(c:Vec3)->Self{
        Self{
            color_value: c,
        }
    }
}
impl Texture for SolidColor{
    fn value(&self, u: f64, v: f64, p: &mut Vec3)->Vec3 {
        return self.color_value;
    }
}

pub struct CheckerTexture {
    pub odd:Arc<dyn Texture>,
    pub even:Arc<dyn Texture>,
}

impl Texture for CheckerTexture{
    fn value(&self, u: f64, v: f64, p: &mut Vec3) -> Vec3 {
        let sines:f64 = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u , v , p);
        }else {
            return self.even.value(u , v , p);
        }
    }
}

impl CheckerTexture{
    pub fn new(c1:Vec3 , c2:Vec3)->Self{
        Self{
            odd: Arc::new(SolidColor::new(c2)),
            even: Arc::new(SolidColor::new(c1)),
        }
    }
}

pub struct NoiseTexture {
    pub noise:perlin::Perlin,
}

impl NoiseTexture{
    pub fn new0()->Self{
        Self{
            noise:Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture{
    fn value(&self, u: f64, v: f64, p: &mut Vec3) -> Vec3 {
        return Vec3::new(1.0 , 1.0 , 1.0) * self.noise.noise(*p);
    }
}