use crate::{random_double, random_double_lim};
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub time: f64,
}
impl Ray {
    fn new(orig: Vec3, dir: Vec3, time: f64) -> Self {
        Self {
            orig: orig,
            dir: dir,
            time: time,
        }
    }
}
impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        return self.orig + self.dir * t;
    }
}

impl Vec3 {
    pub fn near_zero(&self) -> bool{
        let s:f64 = 1e-8;
        return (self.x < s && self.y < s && self.z < s);
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl Vec3 {
    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        return u.x * v.x + u.y * v.y + u.z * v.z;
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        return Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        );
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn length(&self) -> f64 {
        let sqe = Vec3::length_squared(&self);
        return sqe.sqrt() as f64;
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        let divs = Vec3::length(&v);
        return v / divs;
    }

    pub fn unit(&mut self) {
        let divs = Vec3::length(&self);
        *self = Self {
            x: self.x / divs,
            y: self.y / divs,
            z: self.z / divs,
        }
    }
}

impl Vec3 {
    pub fn random() -> Vec3 {
        return Vec3::new(random_double(), random_double(), random_double());
    }

    pub fn random_limit(min: f64, max: f64) -> Vec3 {
        return Vec3::new(
            random_double_lim(min, max),
            random_double_lim(min, max),
            random_double_lim(min, max),
        );
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        while true {
            let p: Vec3 = Vec3::random_limit(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    pub fn random_unit_vector() -> Vec3 {
        let a: f64 = random_double_lim(0.0, 2.0 * PI);
        let z: f64 = random_double_lim(-1.0, 1.0);
        let r: f64 = (1.0 - z * z).sqrt();
        return Vec3::new(r * a.cos(), r * a.sin(), z);
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere: Vec3 = Vec3::random_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return Vec3::new(0.0, 0.0, 0.0) - in_unit_sphere;
        }
    }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - n * 2.0 * Vec3::dot(v, n);
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(Vec3::new(0.0, 0.0, 0.0) - uv, n);
        let r_out_perp: Vec3 = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * (-((1.0 - r_out_perp.length_squared()).abs().sqrt()));
        return r_out_perp + r_out_parallel;
    }

    pub fn random_in_unit_disk() -> Vec3 {
        while true {
            let p: Vec3 = Vec3::new(
                random_double_lim(-1.0, 1.0),
                random_double_lim(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_new() {
//         assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
//     }
//
//     #[test]
//     fn test_add() {
//         assert_eq!(
//             Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
//             Vec3::new(3.0, 4.0, 5.0)
//         )
//     }
//
//     #[test]
//     fn test_add_assign() {
//         let mut x = Vec3::new(1.0, 0.0, -1.0);
//         x += Vec3::new(2.0, 4.0, 6.0);
//         assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
//     }
//
//     #[test]
//     fn test_add_f64() {
//         assert_eq!(
//             Vec3::new(1.0, 0.0, -1.0) + 233.0,
//             Vec3::new(234.0, 233.0, 232.0)
//         )
//     }
//
//
//     #[test]
//     fn test_add_assign_f64() {
//         let mut x = Vec3::new(1.0, 0.0, -1.0);
//         x += 233.0;
//         assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
//     }
//
//     #[test]
//     fn test_sub() {
//         assert_eq!(
//             Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
//             Vec3::new(-1.0, -4.0, -7.0)
//         )
//     }
//
//     #[test]
//     fn test_sub_assign() {
//         let mut x = Vec3::new(1.0, 0.0, -1.0);
//         x -= Vec3::new(2.0, 4.0, 6.0);
//         assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
//     }
//
//     #[test]
//     fn test_sub_f64() {
//         assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
//     }
//
//     #[test]
//     fn test_sub_assign_f64() {
//         let mut x = Vec3::new(1.0, 0.0, -1.0);
//         x -= 1.0;
//         assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
//     }
//
//     #[test]
//     fn test_mul() {
//         assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
//     }
//
//     #[test]
//     fn test_mul_assign() {
//         let mut x = Vec3::new(1.0, 0.0, -1.0);
//         x *= 2.0;
//         assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
//     }
//
//     #[test]
//     fn test_mul_f64() {
//         assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
//     }
//
//     #[test]
//     fn test_div() {
//         assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
//     }
//
//     #[test]
//     fn test_elemul() {
//         assert_eq!(
//             Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
//             Vec3::new(1.0, 4.0, 9.0)
//         );
//     }
//
//     #[test]
//     fn test_cross() {
//         assert_eq!(
//             Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
//             Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
//         );
//     }
//
//     #[test]
//     fn test_neg() {
//         assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
//     }
//
//
//     #[test]
//     fn test_squared_length() {
//         assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0_f64);
//     }
//
//
//     #[test]
//     fn test_length() {
//         assert_eq!(
//             Vec3::new(3.0, 4.0, 5.0).length(),
//             ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
//         );
//     }
//
//     #[test]
//     fn test_unit() {
//         assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
//         assert_eq!(
//             Vec3::new(-233.0, 0.0, 0.0).unit(),
//             Vec3::new(-1.0, 0.0, 0.0)
//         );
//     }
/*
#[test]
#[should_panic]
fn test_unit_panic() {
    Vec3::new(0.0, 0.0, 0.0).unit();
}
*/
// }
