use crate::{degrees_to_radians, random_double_lim, Ray, Vec3};
#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    #[warn(dead_code)]
    pub fn new() -> Self {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;
        Self {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            horizontal: Vec3 {
                x: viewport_width,
                y: 0.0,
                z: 0.0,
            },
            vertical: Vec3 {
                x: 0.0,
                y: viewport_height,
                z: 0.0,
            },
            lower_left_corner: Vec3 {
                x: -viewport_width / 2.0,
                y: -viewport_height / 2.0,
                z: -focal_length,
            },
            lens_radius: 0.0,
            u: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            v: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            w: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            time0: 0.0,
            time1: 0.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        let ray = Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            time: random_double_lim(self.time0, self.time1),
            //time: 0.0,
        };
        ray
    }
    #[warn(dead_code)]
    pub fn camera(vfov: f64, aspect_ratio: f64) -> Self {
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let focal_length: f64 = 1.0;
        let ori: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let hori: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let verti: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
        let llc: Vec3 = ori - hori / 2.0 - verti / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self {
            origin: ori,
            horizontal: hori,
            vertical: verti,
            lower_left_corner: llc,
            lens_radius: 0.0,
            time0: 0.0,
            u: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            v: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            w: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            time1: 0.0,
        }
    }
    #[warn(clippy::too_many_arguments)]
    pub fn camera_from_where(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta: f64 = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let ww: Vec3 = Vec3::unit_vector(lookfrom - lookat);
        let uu: Vec3 = Vec3::unit_vector(Vec3::cross(vup, ww));
        let vv: Vec3 = Vec3::cross(ww, uu);

        let ori = lookfrom;
        let hori = uu * viewport_width * focus_dist;
        let verti = vv * viewport_height * focus_dist;
        let llc = ori - hori / 2.0 - verti / 2.0 - ww * focus_dist;
        Self {
            origin: ori,
            lower_left_corner: llc,
            horizontal: hori,
            vertical: verti,
            lens_radius: aperture / 2.0,
            time0,
            w: ww,
            u: uu,
            v: vv,
            time1,
        }
    }
}
