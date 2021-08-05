mod AABB;
mod BVH;
mod Boxe;
mod aarect;
mod camera;
mod constant_medium;
mod hit;
mod material;
mod moving_sphere;
mod onb;
mod pdf;
mod perlin;
mod rtweekend;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;

use crate::aarect::{Triangle, XyRect, XzRect, YzRect};
use crate::camera::Camera;
use crate::constant_medium::ConstantMedium;
use crate::hit::{FlipFace, HitRecord, Hittable, HittableList, RotateY, Sphere, Translate};
use crate::material::{
    Dielectric, DiffuseLight, Lambertian, LambertianStatic, Material, Metal, NoMaterial,
    ScatterRecord,
};
use crate::moving_sphere::MovingSphere;
use crate::pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use crate::Boxe::Boxes;
use crate::BVH::BvhNode;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use rtweekend::*;
use std::f64::INFINITY;
pub use std::sync::mpsc::channel;
use std::sync::Arc;
pub use threadpool::ThreadPool;
pub use vec3::Ray;
pub use vec3::Vec3;

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
#[allow(clippy::many_single_char_names)]
#[warn(unused_mut)]
fn main() {
    //Image
    // let mut aspect_ratio: f64 = 16.0 / 9.0;
    // let mut image_width: u32 = 400;
    // let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    // let mut samples_per_pixel: u32 = 10;
    // let mut max_depth: u32 = 5;
    let aspect_ratio: f64 = 1.0;
    let image_width: u32 = 800;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 5;
    let max_depth: u32 = 10;

    let mut lights = HittableList::default();
    lights.add(Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        NoMaterial {},
    )));
    lights.add(Arc::new(Sphere::new(
        Vec3::new(190.0, 90.0, 190.0),
        90.0,
        NoMaterial {},
    )));
    //let mut lights = Arc::new(lights);
    //let lights:Arc<dyn Hittable> = Arc::new(XzRect::new(213.0,343.0,227.0,332.0,554.0 , Arc::new(Metal::new())));
    //World
    //let mut world: HittableList = random_scene();
    // let mg:Lambertian = Lambertian{
    //     albedo: Vec3 {
    //         x: 0.8,
    //         y: 0.8,
    //         z: 0.0
    //     }
    // };
    // let material_ground:Sphere = Sphere{
    //     center: Vec3 {
    //         x: 0.0,
    //         y: -100.5,
    //         z: -1.0
    //     },
    //     radius: 100.0,
    //     mat_ptr: Arc::new(mg),
    // };
    //
    // world.add(Arc::new(material_ground));
    // let mc:Lambertian = Lambertian{
    //     albedo: Vec3 {x:0.1 , y:0.2 , z:0.5},
    // };
    // let material_center:Sphere = Sphere{
    //     center: Vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: -1.0
    //     },
    //     radius: 0.5,
    //     mat_ptr: Arc::new(mc),
    // };
    // world.add(Arc::new(material_center));
    // let ml:Dielectric = Dielectric{
    //     ref_idx: 1.5,
    // };
    // let material_left:Sphere = Sphere{
    //     center: Vec3 {
    //         x: -1.0,
    //         y: 0.0,
    //         z: -1.0
    //     },
    //     radius: 0.5,
    //     mat_ptr: Arc::new(ml),
    // };
    // let material_left1:Sphere = Sphere{
    //     center: Vec3 {
    //         x: -1.0,
    //         y: 0.0,
    //         z: -1.0
    //     },
    //     radius: -0.45,
    //     mat_ptr: Arc::new(ml),
    // };
    //
    // world.add(Arc::new(material_left));
    // world.add(Arc::new(material_left1));
    // let mr:Metal = Metal{
    //     albedo: Vec3 {
    //         x: 0.8,
    //         y: 0.6,
    //         z: 0.2
    //     },
    //     fuzz: 0.0,
    // };
    // let material_right:Sphere = Sphere{
    //     center: Vec3 {
    //         x: 1.0,
    //         y: 0.0,
    //         z: -1.0
    //     },
    //     radius: 0.5,
    //     mat_ptr: Arc::new(mr),
    // };
    //
    // world.add(Arc::new(material_right));
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
    // let viewport_width:f64 = aspect_ratio * viewport_height;
    // let focal_length = 1.0;
    //
    // let origin:Vec3 = Vec3::new(0.0 , 0.0 ,0.0);
    // let horizontal:Vec3 = Vec3::new(viewport_width , 0.0 , 0.0);
    // let vertical:Vec3 = Vec3::new(0.0 , viewport_height , 0.0);

    // let cam:Camera = Camera::new(); //四球
    // let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    // let lookat = Vec3::new(0.0, 0.0, 0.0);
    // let vup = Vec3::new(0.0, 1.0, 0.0);
    // let dist_to_focus = 10.0;
    // let aperture = 0.1;
    let world: HittableList;
    let lookfrom: Vec3;
    let lookat: Vec3;
    let mut vfov: f64 = 40.0;
    let mut aperture: f64 = 0.0;
    let mut background: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let op = 5;
    match op {
        0 => {
            world = random_scene();
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            background = Vec3::new(0.7, 0.8, 1.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        1 => {
            world = two_spheres();
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            background = Vec3::new(0.7, 0.8, 1.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        2 => {
            world = two_perlin_spheres();
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            background = Vec3::new(0.7, 0.8, 1.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            world = earth();
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            background = Vec3::new(0.7, 0.8, 1.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            world = simple_light();
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            background = Vec3::new(0.0, 0.0, 0.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            world = cornell_box();
            // aspect_ratio = 1.0;
            // image_width = 600;
            // image_height = (image_width as f64 / aspect_ratio) as u32;
            // samples_per_pixel = 200;
            // let mut lights = Arc::new(HittableList::new());
            // lights.add(Arc::new(XzRect::new(213.0,343.0,227.0,332.0,554.0 , Arc::new(Metal::new()))));
            // lights.add(Arc::new(Sphere::new(Vec3::new(190.0,90.0,190.0) , 90.0 , Arc::new(Metal::new()))));

            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        6 => {
            world = cornell_smoke();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = final_scene();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(478.0, 278.0, -600.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            lookfrom = Vec3::new(478.0, 278.0, -600.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
        }
    }
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;

    let cam: Camera = Camera::camera_from_where(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // //视口左下角的坐标
    // let lower_left_corner:Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0 , 0.0 , focal_length);

    //Render
    // let bar = ProgressBar::new(1024);
    // let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    // for j in (0..image_height).rev() {
    //     for i in 0..image_width {
    //         let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    //         let pixel = img.get_pixel_mut(i, image_height - 1 - j);
    //         for k in 0..samples_per_pixel {
    //             let u: f64 = (i as f64 + random_double()) / ((image_width - 1) as f64);
    //             let v: f64 = (j as f64 + random_double()) / ((image_height - 1) as f64);
    //             let r: Ray = cam.get_ray(u, v);
    //             pixel_color += ray_color(r, background, &world, lights.clone(), max_depth);
    //         }
    //         *pixel = write_color(&pixel_color, samples_per_pixel);
    //     }
    //     bar.inc(1);
    // }
    // img.save("output/test.png").unwrap();
    // bar.finish();

    let (tx, rx) = channel();
    let n_jobs: usize = 32;
    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);
    let bar = ProgressBar::new((n_jobs + 1) as u64);
    let world_in_thread = Arc::new(world);

    for i in 0..n_jobs {
        let tx = tx.clone();
        let light_ptr = lights.clone();
        let world_ptr = world_in_thread.clone();
        let bars = bar.clone();
        pool.execute(move || {
            let row_begin = image_height as usize * i / n_jobs;
            let row_end = image_height as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let pixel = img.get_pixel_mut(x, img_y as u32);
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (x as f64 + random_double()) / (image_width - 1) as f64;
                        let v = (image_height as f64 - y as f64 + random_double())
                            / (image_height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(r, background, &*world_ptr, &light_ptr, max_depth);
                    }
                    *pixel = write_color(&pixel_color, samples_per_pixel);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
            bars.inc(1);
        });
    }
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *img.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
            }
        }
    }
    img.save("output/test.png").unwrap();
    bar.finish();
}
fn write_color(pixel_color: &Vec3, samples_per_pixel: u32) -> image::Rgb<u8> {
    let mut r: f64 = pixel_color.x;
    let mut g: f64 = pixel_color.y;
    let mut b: f64 = pixel_color.z;

    let scale: f64 = 1.0 / (samples_per_pixel as f64);

    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    let ir: u8 = (256.0 * rtweekend::clamp(r, 0.0, 0.999)) as u8;
    let ig: u8 = (256.0 * rtweekend::clamp(g, 0.0, 0.999)) as u8;
    let ib: u8 = (256.0 * rtweekend::clamp(b, 0.0, 0.999)) as u8;
    return image::Rgb([ir, ig, ib]);
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
#[warn(unused_assignments)]
fn ray_color(
    mut r: Ray,
    background: Vec3,
    world: &dyn Hittable,
    lights: &HittableList,
    depth: u32,
) -> Vec3 {
    // let mut rec = HitRecord {
    //     p: Vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    //     normal: Vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    //     mat_ptr: &Lambertian::new(Vec3::new(0.0, 0.0, 0.0)),
    //     t: 0.0,
    //     u: 0.0,
    //     v: 0.0,
    //     front_face: true,
    // };
    let mut rec:HitRecord;
    #[deny(clippy::absurd_extreme_comparisons)]
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    match world.hit(r, 0.001, INFINITY) {
        Some(rec_) => {
            rec = rec_;
        }
        None => {
            return background;
        }
    }
    let mut scattered: Ray = Ray {
        orig: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        dir: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        time: 0.0,
    };
    // let mut albedo: Vec3 = Vec3 {
    //     x: 0.0,
    //     y: 0.0,
    //     z: 0.0,
    // };
    let mut srec: ScatterRecord = ScatterRecord {
        specular_ray: Ray {
            orig: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            dir: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            time: 0.0,
        },
        is_specular: false,
        attenuation: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        pdf_ptr: CosinePdf::new(Vec3::new(0.0, 0.0, 0.0)),
    };
    let emitted: Vec3 =
        rec.mat_ptr
            .emitted(&mut r.clone(), &mut rec.clone(), rec.u, rec.v, &mut rec.p);
    let pdf_val: f64;

    if !rec.mat_ptr.scatter(&mut r, &mut rec.clone(), &mut srec) {
        return emitted;
    };
    if srec.is_specular {
        return srec.attenuation
            * ray_color(srec.specular_ray, background, world, lights, depth - 1);
    }
    // let on_light = Vec3::new(random_double_lim(213.0 , 343.0) , 554.0 , random_double_lim(227.0,332.0));
    // let mut to_light = on_light - rec.p;
    // let distance_squared = to_light.length_squared();
    // to_light = Vec3::unit_vector(to_light);
    //
    // if Vec3::dot(to_light , rec.normal) < 0.0 {
    //     return emitted;
    // }
    //
    // let light_area = (343.0 - 213.0) * (332.0 - 227.0);
    // let light_cosine = to_light.y.abs();
    // if light_cosine < 0.000001 {
    //     return emitted;
    // }
    //
    // pdf_val = distance_squared / (light_cosine * light_area);

    // let p:CosinePdf = CosinePdf::new(rec.normal);
    //
    // scattered.orig = rec.p;
    // scattered.dir = p.generate();
    // scattered.time = r.time;
    //let p0 = Arc::new(HittablePdf::new(lights.clone() , rec.p));
    //let p1 = Arc::new(CosinePdf::new(rec.normal));
    //let mut mixed_pdf = MixturePdf::new(p0, p1);
    //let mut light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.p));
    let light_ptr = HittablePdf::new(lights, rec.p);

    //let mut light_pdf:HittablePdf = HittablePdf::new(lights.clone() , rec.p);

    //let p: MixturePdf = MixturePdf::new(light_ptr, srec.pdf_ptr);
    let p = MixturePdf::new(&light_ptr, &srec.pdf_ptr);
    scattered.orig = rec.p;
    scattered.dir = p.generate();
    scattered.time = r.time;

    //return emitted + albedo * ray_color(scattered , background , world , depth - 1);

    // let emi:Vec3 = emitted;
    //
    // let alb:Vec3 = albedo;
    //
    // let res:f64 = rec.mat_ptr.scattering_pdf(&mut r , &mut rec.clone() , &mut scattered);
    //
    // let rays:Vec3 = ray_color(scattered, background, world, depth - 1);
    //
    // return emi + alb * res * rays / pdf_val;

    //pdf_val = p.value(&mut scattered.dir);
    //pdf_val = mixed_pdf.value(&mut scattered.dir);
    pdf_val = p.value(&mut scattered.dir);
    //return  emitted + albedo * rec.mat_ptr.scattering_pdf(&mut r , &mut rec.clone() , &mut scattered) * ray_color(scattered, background, world, lights.clone(), depth - 1) / pdf_val;
    let recs = rec
        .mat_ptr
        .scattering_pdf(&mut r, &mut rec.clone(), &mut scattered);

    let ans = emitted
        + ray_color(scattered, background, world, &lights, depth - 1) * srec.attenuation * recs
            / pdf_val;
    return ans;
    //return Vec3::new(0.0, 0.0, 0.0);
    // let unit_direction: Vec3 = Vec3::unit_vector(r.dir);
    // let t: f64 = 0.5 * (unit_direction.y + 1.0);
    //线性插值
    //return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList { objects: vec![] };
    // let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    // let gd: Sphere = Sphere {
    //     center: Vec3 {
    //         x: 0.0,
    //         y: -1000.0,
    //         z: 0.0,
    //     },
    //     radius: 1000.0,
    //     mat_ptr: ground_material,
    // };
    // world.add(Arc::new(gd));
    let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let gd: Sphere<LambertianStatic<CheckerTexture>> = Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: LambertianStatic::<CheckerTexture>::news(checker),
    };
    world.add(Arc::new(gd));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_double();
            let center: Vec3 = Vec3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                //let sphere_material: dyn Material;
                if choose_mat < 0.8 {
                    let albedo = Vec3::new(random_double(), random_double(), random_double())
                        * Vec3::new(random_double(), random_double(), random_double());
                    let sphere_material = LambertianStatic::<SolidColor>::new(albedo);
                    let center2: Vec3 = center + Vec3::new(0.0, random_double_lim(0.0, 0.5), 0.0);
                    let ms: MovingSphere<LambertianStatic<SolidColor>> = MovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    };
                    world.add(Arc::new(ms));
                } else if choose_mat < 0.95 {
                    let albedo: Vec3 = Vec3::new(
                        random_double_lim(0.5, 1.0),
                        random_double_lim(0.5, 1.0),
                        random_double_lim(0.5, 1.0),
                    );
                    let fuzz: f64 = random_double_lim(0.0, 0.5);
                    let sphere_material = Metal::news(albedo, fuzz);
                    let met: Sphere<Metal> = Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    };
                    world.add(Arc::new(met));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    let gla: Sphere<Dielectric> = Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    };
                    world.add(Arc::new(gla));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    let mat1: Sphere<Dielectric> = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material1,
    };
    world.add(Arc::new(mat1));
    let material2 = LambertianStatic::<SolidColor>::new(Vec3::new(0.4, 0.2, 0.1));
    let mat2: Sphere<LambertianStatic<SolidColor>> = Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material2,
    };
    world.add(Arc::new(mat2));
    let material3 = Metal::news(Vec3::new(0.7, 0.6, 0.5), 0.0);
    let mat3: MovingSphere<Metal> = MovingSphere {
        center0: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        center1: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        time0: 0.0,
        radius: 1.0,
        mat_ptr: material3,
        time1: 1.0,
    };
    world.add(Arc::new(mat3));
    return world;
}

pub fn two_spheres() -> HittableList {
    let mut objects: HittableList = HittableList { objects: vec![] };

    let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let sph1: Sphere<LambertianStatic<CheckerTexture>> = Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        mat_ptr: LambertianStatic::news(checker.clone()),
    };
    let sph2: Sphere<LambertianStatic<CheckerTexture>> = Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        mat_ptr: LambertianStatic::news(checker.clone()),
    };

    objects.add(Arc::new(sph1));
    objects.add(Arc::new(sph2));

    return objects;
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects: HittableList = HittableList { objects: vec![] };
    let pertext = NoiseTexture::new0(4.0);
    //let pertext = Arc::new(CheckerTexture::new(Vec3::new(0.2 , 0.3 , 0.1) , Vec3::new(0.9 , 0.9 , 0.9)));

    let sph1: Sphere<LambertianStatic<NoiseTexture>> = Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: LambertianStatic::news(pertext.clone()),
    };
    let sph2: Sphere<LambertianStatic<NoiseTexture>> = Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: LambertianStatic::news(pertext.clone()),
    };

    objects.add(Arc::new(sph1));
    objects.add(Arc::new(sph2));

    return objects;
}

pub fn earth() -> HittableList {
    let mut objects: HittableList = HittableList { objects: vec![] };
    let earth_texture = texture::ImageTexture::new("input/earthmap.jpg");
    //let earth_surface = Arc::new(Lambertian::news(earth_texture));
    let sph: Sphere<LambertianStatic<ImageTexture>> = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        mat_ptr: LambertianStatic::news(earth_texture),
    };
    objects.add(Arc::new(sph));
    return objects;
}
pub fn simple_light() -> HittableList {
    let mut objects: HittableList = HittableList { objects: vec![] };

    let pertext = NoiseTexture::new0(4.0);
    let sph1: Sphere<LambertianStatic<NoiseTexture>> = Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: LambertianStatic::news(pertext.clone()),
    };
    let sph2: Sphere<LambertianStatic<NoiseTexture>> = Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: LambertianStatic::news(pertext.clone()),
    };
    objects.add(Arc::new(sph1));
    objects.add(Arc::new(sph2));
    let difflight = DiffuseLight::new0(Vec3::new(4.0, 4.0, 4.0));
    objects.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    return objects;
}

pub fn cornell_box() -> HittableList {
    let mut objects: HittableList = HittableList { objects: vec![] };

    let red = Lambertian::new(Vec3::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Vec3::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Vec3::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new0(Vec3::new(15.0, 15.0, 15.0));
    let allin = Translate::new(
        get_obj("input/bunny.fine.obj", 1000.0),
        Vec3::new(260.0, 50.0, 290.0),
    );
    // let allin = RotateY::new(allin , 90.0);
    objects.add(Arc::new(allin));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(FlipFace::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    ))));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    // let mut aluminum = Metal::news(Vec3::new(0.8,0.85,0.88) , 0.0);
    // let mut box1 = Boxes::new(Vec3::new(0.0,0.0,0.0),Vec3::new(165.0,330.0,165.0) , aluminum);
    // let box1 = RotateY::new(box1, 15.0);
    // let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    // objects.add(box1);
    //
    // let glass = Dielectric::new(1.5);
    // objects.add(Arc::new(Sphere::new(
    //     Vec3::new(190.0, 90.0, 190.0),
    //     90.0,
    //     glass.clone(),
    // )));

    return objects;
}

pub fn cornell_smoke() -> HittableList {
    let mut objects: HittableList = HittableList { objects: vec![] };

    let red = Lambertian::new(Vec3::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Vec3::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Vec3::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new0(Vec3::new(15.0, 15.0, 15.0));
    objects.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    objects.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    // objects.add(Arc::new(XzRect::new(
    //     113.0,
    //     443.0,
    //     127.0,
    //     432.0,
    //     554.0,
    //     light.clone(),
    // )));
    objects.add(Arc::new(FlipFace::new(XzRect::new(
        123.0,
        443.0,
        127.0,
        442.0,
        554.0,
        light.clone(),
    ))));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Boxes::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    objects.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    )));

    let box2 = Boxes::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));
    objects.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    return objects;
}

pub fn final_scene() -> HittableList {
    let mut boxes1: HittableList = HittableList { objects: vec![] };

    let boxes_per_side: i32 = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_lim(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Boxes::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                LambertianStatic::<SolidColor>::new(Vec3::new(0.48, 0.83, 0.53)),
            )));
        }
    }

    let mut objects: HittableList = HittableList { objects: vec![] };
    objects.add(Arc::new(BvhNode::new(boxes1, 0.0, 1.0)));

    let lighting = DiffuseLight::new0(Vec3::new(17.0, 17.0, 17.0));
    // objects.add(Arc::new(XzRect::new(
    //     123.0,
    //     423.0,
    //     147.0,
    //     412.0,
    //     554.0,
    //     lighting.clone(),
    // )));

    objects.add(Arc::new(FlipFace::new(XzRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        lighting.clone(),
    ))));
    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = Vec3::new(30.0, 0.0, 0.0) + center1;
    let moving_sphere_material = LambertianStatic::<SolidColor>::new(Vec3::new(0.7, 0.3, 0.1));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::news(Vec3::new(0.8, 0.8, 0.9), 1.0),
    )));

    let mut boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new(
        *boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Dielectric::new(1.5),
    ));
    objects.add(Arc::new(ConstantMedium::new(
        *boundary,
        0.0001,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    let emat = Lambertian::news(Arc::new(ImageTexture::new("earthmap.jpg")));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = NoiseTexture::new0(0.1);
    objects.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        LambertianStatic::news(pertext.clone()),
    )));

    let mut boxes2: HittableList = HittableList { objects: vec![] };
    let white = LambertianStatic::<SolidColor>::new(Vec3::new(0.73, 0.73, 0.73));
    let ns: i32 = 1000;
    #[warn(unused_variables)]
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::new(
                random_double_lim(0.0, 165.0),
                random_double_lim(0.0, 165.0),
                random_double_lim(0.0, 165.0),
            ),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        RotateY::new(BvhNode::new(boxes2, 0.0, 1.0), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    return objects;
}
pub fn get_obj(filename: &str, rate: f64) -> HittableList {
    let mut objects = HittableList { objects: vec![] };
    let cornell_box = tobj::load_obj(
        filename,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    assert!(cornell_box.is_ok());
    let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
    for (_i, m) in models.iter().enumerate() {
        let mut boxes2 = HittableList { objects: vec![] };
        let mesh = &m.mesh;
        for v in 0..mesh.indices.len() / 3 {
            let x1 = mesh.indices[3 * v];
            let x2 = mesh.indices[3 * v + 1];
            let x3 = mesh.indices[3 * v + 2];
            let triange = Triangle::new(
                Vec3 {
                    x: rate * mesh.positions[(3 * x1) as usize] as f64,
                    y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x2) as usize] as f64,
                    y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x3) as usize] as f64,
                    y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                },
                Metal::news(Vec3::new(0.99, 0.78, 0.0), 0.1),
            );
            boxes2.add(Arc::new(triange));
        }

        objects.add(Arc::new(BvhNode::new(boxes2, 0.0, 1.0)));
    }
    objects
}
