use rand::Rng;
use std::f64::consts::PI;

pub fn random_double() -> f64 {
    let mut dou = rand::thread_rng();
    return dou.gen();
}

pub fn random_double_lim(min: f64, max: f64) -> f64 {
    let mut dou = rand::thread_rng();
    return dou.gen_range(min, max);
}
pub fn random_int(min: i32, max: i32) -> i32 {
    let mut ins = rand::thread_rng();
    return ins.gen_range(min, max + 1);
}
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}
