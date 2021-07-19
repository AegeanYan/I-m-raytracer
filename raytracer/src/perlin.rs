use crate::{rtweekend, random_int, random_double};
use crate::Vec3;
use std::vec;
const POINT_COUNT:u32 = 256;
pub struct Perlin{
    pub ranfloat:Vec<f64>,
    pub perm_x:Vec<i32>,
    pub perm_y:Vec<i32>,
    pub perm_z:Vec<i32>,
}

impl Perlin{

    pub fn permute(p:&mut Vec<i32> , n:i32){
        for i in (1..n).rev() {
            let target:i32 = random_int(0 , i);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
    pub fn perlin_generate_perm()->Vec<i32>{
        let mut p:Vec<i32> = Vec::with_capacity(POINT_COUNT as usize);

        for i in 0..POINT_COUNT {
            p.push(i as i32);
        }
        Perlin::permute(&mut p, POINT_COUNT as i32);

        return p;
    }

    pub fn new()->Self{
        let mut v:Vec<f64> = Vec::with_capacity(POINT_COUNT as usize);
        for i in 0..POINT_COUNT {
            v.push(random_double());
        }
        Self{
            ranfloat:v,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
}

impl Perlin{
    pub fn noise(&self , p:Vec3)->f64{
        let i:usize = (4.0 * p.x) as usize & 255;
        let j:usize = (4.0 * p.y) as usize & 255;
        let k:usize = (4.0 * p.z) as usize & 255;

        return self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize];
    }
}
