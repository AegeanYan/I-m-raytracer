use crate::Vec3;
use rand::Rng;
use std::f64::consts::PI;

pub fn random_double() -> f64 {
    let mut dou = rand::thread_rng();
    dou.gen()
}

pub fn random_double_lim(min: f64, max: f64) -> f64 {
    let mut dou = rand::thread_rng();
    dou.gen_range(min, max)
}
pub fn random_int(min: i32, max: i32) -> i32 {
    let mut ins = rand::thread_rng();
    ins.gen_range(min, max + 1)
}
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
pub fn f_3_min(v0: f64, v1: f64, v2: f64) -> f64 {
    let a = v0.min(v1);
    a.min(v2)
}
pub fn f_3_max(v0: f64, v1: f64, v2: f64) -> f64 {
    let a = v0.max(v1);
    a.max(v2)
}

pub fn get_triangle_uv(v_ab: Vec3, v_bc: Vec3, v_ap: Vec3, v_bp: Vec3, u: &mut f64, v: &mut f64) {
    *u = Vec3::dot(v_ab, v_ap) / v_ab.length();
    *v = Vec3::dot(v_bc, v_bp) / v_bc.length();
}
