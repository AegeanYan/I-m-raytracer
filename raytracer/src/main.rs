#[allow(clippy::float_cmp)]
mod vec3;
mod hit;
mod camera;
mod rtweekend;
mod material;

use image::{ImageBuffer, RgbImage};
pub use vec3::Vec3;
pub use vec3::Ray;
pub use rtweekend::*;
use std::f64::consts::PI;
use crate::hit::{HittableList, Sphere, Hittable, HitRecord};
use crate::material::{Material, Lambertian ,Metal,Dielectric};
use crate::camera::Camera;
use std::sync::Arc;
// fn main() {
//     let x = Vec3::new(1.0, 1.0, 1.0);
//     println!("{:?}", x);
//
//     let mut img: RgbImage = ImageBuffer::new(256, 256);
//     let bar = ProgressBar::new(256);
//
//     for j in (0..256) {
//         for i in 0..256 {
//             let pixel = img.get_pixel_mut(i, j);
//             // let color = (x / 4) as u8;
//             let r:f64 = (i as f64/ 255.0) ;
//             let g:f64 = ((256 - j) as f64/ 255.0) ;
//             let b:f64 = 0.25;
//             // let ir:u8 = (255.999 * r) as u8;
//             // let ig:u8 = (255.999 * g) as u8;
//             // let ib:u8 = (255.999 * b) as u8;
//             // *pixel = image::Rgb([ir , ig , ib]);
//             *pixel = write_color(&r , &g , &b);
//         }
//         //bar.inc(7);
//     }
//
//     img.save("output/test.png").unwrap();
//     bar.finish();
// }

fn main(){


    //Image
    const ASPECT_RATIO:f64 = 16.0/9.0;
    const IMAGE_WIDTH:u32 = 400;
    const IMAGE_HEIGHT:u32 = (IMAGE_WIDTH as f64/ ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH:u32 = 50;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    //World
    let mut world:HittableList = HittableList { objects: vec![] };
    let mg:Lambertian = Lambertian{
        albedo: Vec3 {
            x: 0.8,
            y: 0.8,
            z: 0.0
        }
    };
    let material_ground:Sphere = Sphere{
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0
        },
        radius: 100.0,
        mat_ptr: Arc::new(mg),
    };

    world.add(Arc::new(material_ground));
    let mc:Lambertian = Lambertian{
        albedo: Vec3 {x:0.1 , y:0.2 , z:0.5},
    };
    let material_center:Sphere = Sphere{
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0
        },
        radius: 0.5,
        mat_ptr: Arc::new(mc),
    };
    world.add(Arc::new(material_center));
    let ml:Dielectric = Dielectric{
        ref_idx: 1.5,
    };
    let material_left:Sphere = Sphere{
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0
        },
        radius: 0.5,
        mat_ptr: Arc::new(ml),
    };
    let material_left1:Sphere = Sphere{
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0
        },
        radius: -0.45,
        mat_ptr: Arc::new(ml),
    };

    world.add(Arc::new(material_left));
    world.add(Arc::new(material_left1));
    let mr:Metal = Metal{
        albedo: Vec3 {
            x: 0.8,
            y: 0.6,
            z: 0.2
        },
        fuzz: 0.0,
    };
    let material_right:Sphere = Sphere{
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0
        },
        radius: 0.5,
        mat_ptr: Arc::new(mr),
    };

    world.add(Arc::new(material_right));
    //四球例子

    // let R:f64 = (PI / 4.0).cos();
    // let ml:Lambertian = Lambertian{
    //     albedo: Vec3 {x:0.0 , y:0.0 , z:1.0},
    // };
    // let material_left:Sphere = Sphere{
    //     center: Vec3 {x:-R , y:0.0 ,z:-1.0},
    //     radius: R,
    //     mat_ptr: Arc::new(ml),
    // };
    // world.add(Arc::new(material_left));
    //
    // let mr:Lambertian = Lambertian{
    //     albedo: Vec3 {x:1.0 , y:0.0 , z:0.0},
    // };
    // let material_right:Sphere = Sphere{
    //     center: Vec3 {x:R , y:0.0 , z:-1.0},
    //     radius: R,
    //     mat_ptr: Arc::new(mr)
    // };
    // world.add(Arc::new(material_right));
    //双球例子

    // let sph1 = Sphere{
    //     center: Vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: -1.0,
    //     },
    //     radius: 0.5,
    // };
    // let sph2 = Sphere{
    //     center: Vec3 {
    //         x: 0.0,
    //         y: -100.5,
    //         z: -1.0,
    //     },
    //     radius: 100.0,
    // };
    // world.add(Box::new(sph2));
    // world.add(Box::new(sph1));

    //Camera
    // let viewport_height:f64 = 2.0;
    // let viewport_width:f64 = ASPECT_RATIO * viewport_height;
    // let focal_length = 1.0;
    //
    // let origin:Vec3 = Vec3::new(0.0 , 0.0 ,0.0);
    // let horizontal:Vec3 = Vec3::new(viewport_width , 0.0 , 0.0);
    // let vertical:Vec3 = Vec3::new(0.0 , viewport_height , 0.0);

    // let cam:Camera = Camera::new(); //四球
    let lookfrom = Vec3::new(3.0 , 3.0 , 2.0);
    let lookat = Vec3::new(0.0 , 0.0 , -1.0);
    let vup = Vec3::new(0.0 , 1.0 , 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let cam:Camera = Camera::camera_from_where(lookfrom , lookat , vup , 20.0 , 16.0 / 9.0 , aperture , dist_to_focus);

    // //视口左下角的坐标
    // let lower_left_corner:Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0 , 0.0 , focal_length);

    //Render
    for j in (0 .. IMAGE_HEIGHT).rev(){
        for i in 0 .. IMAGE_WIDTH {
            let mut pixel_color:Vec3 = Vec3::new(0.0 , 0.0 , 0.0);
            let pixel = img.get_pixel_mut(i, IMAGE_HEIGHT - 1 - j);
            for k in 0..SAMPLES_PER_PIXEL {
                let u:f64 = (i as f64 + random_double()) / ((IMAGE_WIDTH - 1) as f64);
                let v:f64 = (j as f64 + random_double()) / ((IMAGE_HEIGHT - 1) as f64);
                let r:Ray = cam.get_ray(u , v);
                pixel_color += ray_color(r , &world ,MAX_DEPTH);
            }
            *pixel = write_color(&pixel_color , SAMPLES_PER_PIXEL);
        }
    }
    img.save("output/test.png").unwrap();
}
fn write_color(pixel_color:&Vec3 , samples_per_pixel:u32) -> image::Rgb<u8>{
    let mut r:f64 = pixel_color.x;
    let mut g:f64 = pixel_color.y;
    let mut b:f64 = pixel_color.z;

    let scale:f64 = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    let ir:u8 = (256.0 * rtweekend::clamp(r , 0.0 , 0.999)) as u8;
    let ig:u8 = (256.0 * rtweekend::clamp(g , 0.0 , 0.999)) as u8;
    let ib:u8 = (256.0 * rtweekend::clamp(b , 0.0 , 0.999)) as u8;
    return image::Rgb([ir , ig , ib]);
}

// fn ray_color(r:Ray) -> Vec3{
//     let t:f64 = hit_sphere(Vec3::new(0.0 , 0.0 , -1.0) , 0.5 , &r);
//     if t > 0.0 {
//         let n:Vec3 = Vec3::unit_vector(r.at(t) - Vec3::new(0.0 , 0.0 , -1.0));
//         return Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
//     }
//     let unit_direction:Vec3 = Vec3::unit_vector(r.dir);
//     let t:f64 = 0.5 * (unit_direction.y + 1.0);
//     return Vec3::new(1.0 , 1.0 , 1.0) * (1.0 - t) + Vec3::new(0.5 , 0.7 , 1.0) * t;
// }
//
// fn hit_sphere(center:Vec3 , radius:f64 , r:&Ray)->f64{
//     let oc:Vec3 = r.orig - center;
//     let a:f64 = Vec3::dot(r.dir , r.dir);
//     let half_b:f64 = Vec3::dot(r.dir , oc);
//     let c:f64 = oc.length_squared() - radius * radius;
//     let discriminant:f64 = half_b * half_b - a * c;
//     if discriminant < 0.0 {
//         return -1.0;
//     }else {
//         let t1:f64 = (-half_b - discriminant.sqrt()) /a;
//         if t1 > 0.0 {
//             return t1;
//         }
//         let t2:f64 = (-half_b + discriminant.sqrt()) /a;
//         if t2 > 0.0 {
//             return t2;
//         }
//         return -1.0;
//     }
// }

fn ray_color(r:Ray, world: &dyn Hittable , depth:u32) -> Vec3{
    let mut rec = HitRecord{
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        mat_ptr: Arc::new(Metal::new()),
        t: 0.0,
        front_face: true,
    };
    if depth <= 0 {
        return Vec3::new(0.0 , 0.0 , 0.0);
    }
    if world.hit(r , 0.001 , f64::INFINITY , &mut rec) {
        let mut scattered: Ray = Ray {
            orig: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            dir: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };
        let mut attenuation: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };

        if rec.mat_ptr.scatter(r , rec.clone() ,&mut attenuation ,&mut scattered) {
            return ray_color(scattered , world , depth - 1) * attenuation;
        }
        return Vec3::new(0.0 , 0.0 ,0.0);
    }
    let unit_direction:Vec3 = Vec3::unit_vector(r.dir);
    let t:f64 = 0.5 * (unit_direction.y + 1.0);
    //线性插值
    return Vec3::new(1.0 , 1.0 , 1.0) * (1.0 - t) + Vec3::new(0.5 , 0.7 , 1.0) * t;
}
