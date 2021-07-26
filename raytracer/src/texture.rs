use crate::perlin;
use crate::perlin::Perlin;
use crate::Vec3;
use crate::{clamp, rtweekend};
use image::{GenericImageView};
use imageproc::drawing::Canvas;
use imageproc::noise;
use std::path::Path;
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &mut Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color_value: Vec3,
}
impl SolidColor {
    pub fn new(c: Vec3) -> Self {
        Self { color_value: c }
    }
}
impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &mut Vec3) -> Vec3 {
        return self.color_value;
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &mut Vec3) -> Vec3 {
        let sines: f64 = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

impl CheckerTexture {
    pub fn new(c1: Vec3, c2: Vec3) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(c2)),
            even: Arc::new(SolidColor::new(c1)),
        }
    }
}

pub struct NoiseTexture {
    pub noise: perlin::Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new0(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &mut Vec3) -> Vec3 {
        //return Vec3::new(1.0 , 1.0 , 1.0) * 0.5 * (self.noise.noise(&((*p) * self.scale)) + 1.0);
        //return Vec3::new(1.0 , 1.0 , 1.0) * self.noise.turb(&mut((*p) * self.scale) , 7);
        return Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + self.noise.turb(&mut p.clone(), 7) * 10.0).sin());
    }
}
const BYTES_PER_PIXEL: i32 = 3;
pub struct ImageTexture {
    pub data: image::RgbImage,
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        // let components: i32 = BYTES_PER_PIXEL;
        // let ima= image::open(&Path::new(filename)).unwrap();
        // let w = GenericImageView::dimensions(&ima).0 as i32;
        // Self {
        //     width: GenericImageView::dimensions(&ima).0 as i32,
        //     height: GenericImageView::dimensions(&ima).1 as i32,
        //     bytes_per_scanline: 0,
        //     data: ima.clone(),
        // }
        let ima = image::open(filename).expect("failed").to_rgb();
        let w = ima.width();
        let h = ima.height();
        Self{
            width:w as i32,
            height:h as i32,
            bytes_per_scanline:0,
            data:ima
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, p: &mut Vec3) -> Vec3 {
        let mut u = clamp(u, 0.0, 1.0);
        let mut v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as i32;
        let mut j = (v * self.height as f64) as i32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale: f64 = 1.0 / 255.0;

        //let pixel = image::GenericImageView::get_pixel(&self.data , i as u32 , j as u32);
        //let pixel = self.data.get_pixel(i as u32, j as u32);
        //let pixel = GenericImageView::get_pixel(&self.data , i as u32, j as u32);
        let pixel = self.data.get_pixel(i as u32,  j as u32);
        return Vec3::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        );
    }
}
