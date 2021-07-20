use crate::Vec3;
use crate::{random_double, random_int, rtweekend};
use std::vec;

const POINT_COUNT: usize = 256;
pub struct Perlin {
    //pub ranfloat:Vec<f64>,
    pub ranvec: Vec<Vec3>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
    // pub ranfloat:[f64;POINT_COUNT],
    // pub perm_x:[i32;POINT_COUNT],
    // pub perm_y:[i32;POINT_COUNT],
    // pub perm_z:[i32;POINT_COUNT],
}

impl Perlin {
    pub fn permute(p: &mut Vec<i32>, n: i32) {
        for i in n - 1..0 {
            let target: i32 = random_int(0, i as i32);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
    pub fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = Vec::with_capacity(POINT_COUNT as usize);

        for i in 0..POINT_COUNT {
            p.push(i as i32);
        }
        Perlin::permute(&mut p, POINT_COUNT as i32);

        return p;
    }

    // pub fn perlin_generate_perm(p:&mut [i32;POINT_COUNT]){
    //     for i in 0..POINT_COUNT {
    //         p[i] = i as i32;
    //     }
    //     Perlin::permute(p , POINT_COUNT);
    // }
    //
    // pub fn permute(p:&mut [i32;POINT_COUNT] , n:usize){
    //     for i in n-1 .. 0 {
    //         let mut target = random_int(0 , i as i32);
    //         let tmp = p[i];
    //         p[i] = p[target as usize];
    //         p[target as usize] = tmp;
    //     }
    // }
    pub fn new() -> Self {
        let mut v: Vec<Vec3> = Vec::with_capacity(POINT_COUNT as usize);
        for i in 0..POINT_COUNT {
            v.push(Vec3::unit_vector(Vec3::random_limit(-1.0, 1.0)));
        }
        Self {
            //ranfloat:v.clone(),
            ranvec: v,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
        // let mut ran_fl = [0.0;POINT_COUNT];
        // for i in 0..POINT_COUNT {
        //     ran_fl[i] = random_double();
        // }
        // let mut x = [0;POINT_COUNT];
        // Perlin::perlin_generate_perm(&mut x);
        // let mut y = [0;POINT_COUNT];
        // Perlin::perlin_generate_perm(&mut y);
        // let mut z = [0;POINT_COUNT];
        // Perlin::perlin_generate_perm(&mut z);

        // Self{
        //     ranfloat:ran_fl,
        //     perm_x:x,
        //     perm_y:y,
        //     perm_z:z,
        // }
    }
}

impl Perlin {
    // pub fn noise(&self , p:&Vec3)->f64{
    //     let i:usize = ((4.0 * p.x) as i32 & 255) as usize;
    //     let j:usize = ((4.0 * p.y) as i32 & 255) as usize;
    //     let k:usize = ((4.0 * p.z) as i32 & 255) as usize;
    //
    //     return self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize];
    //     //return self.ranfloat[self.perm_z[k] as usize];
    // }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        // u = u * u * (3.0 - 2.0 * u);
        // v = v * v * (3.0 - 2.0 * v);
        // w = w * w * (3.0 - 2.0 * w);
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = (self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize]);
                }
            }
        }
        return Perlin::trilinear_interp(c, u, v, w);
    }
    pub fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum: f64 = 0.0;
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (uu * i as f64 + (1.0 - uu) * (1 - i) as f64)
                        * (vv * j as f64 + (1.0 - vv) * (1 - j) as f64)
                        * (ww * k as f64 + (1.0 - ww) * (1 - k) as f64)
                        * Vec3::dot(c[i][j][k], weight_v);
                }
            }
        }
        return accum;
    }

    pub fn turb(&self, p: &mut Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * Perlin::noise(&self, &temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        return accum.abs();
    }
}
