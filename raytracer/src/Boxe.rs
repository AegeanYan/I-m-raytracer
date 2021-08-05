use crate::aarect::{XyRect, XzRect, YzRect};
use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::AABB::Aabb;
use crate::{Ray, Vec3};
use std::option::Option::Some;

pub struct Boxes<T: Material + Clone> {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: (
        XyRect<T>,
        XyRect<T>,
        XzRect<T>,
        XzRect<T>,
        YzRect<T>,
        YzRect<T>,
    ),
}

impl<T: Material + Clone> Boxes<T> {
    pub fn new(p0: Vec3, p1: Vec3, ptr: T) -> Self {
        Self {
            sides: (
                XyRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr.clone()),
                XyRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, ptr.clone()),
                XzRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr.clone()),
                XzRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, ptr.clone()),
                YzRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr.clone()),
                YzRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr.clone()),
            ),
            box_min: p0,
            box_max: p1,
        }
    }
}
impl<T: Material + Clone> Hittable for Boxes<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //return self.sides.hit(r, t_min, t_max, rec);
        //return self.sides.0.hit(r , t_min , t_max , rec) || self.sides.1.hit(r, t_min ,t_max , rec) || self.sides.2.hit(r, t_min ,t_max , rec) || self.sides.3.hit(r, t_min ,t_max , rec) || self.sides.4.hit(r, t_min ,t_max , rec) || self.sides.5.hit(r, t_min ,t_max , rec);
        let mut temp: Option<HitRecord> = None;
        let mut max = t_max;
        if let Some(rec) = self.sides.0.hit(r, t_min, max) {
            temp = Some(rec.clone());
            max = rec.t;
        };
        if let Some(rec) = self.sides.1.hit(r, t_min, max) {
            temp = Some(rec.clone());
            max = rec.t;
        };
        if let Some(rec) = self.sides.2.hit(r, t_min, max) {
            temp = Some(rec.clone());
            max = rec.t;
        };
        if let Some(rec) = self.sides.3.hit(r, t_min, max) {
            temp = Some(rec.clone());
            max = rec.t;
        };
        if let Some(rec) = self.sides.4.hit(r, t_min, max) {
            temp = Some(rec.clone());
            max = rec.t;
        };
        if let Some(rec) = self.sides.5.hit(r, t_min, max) {
            temp = Some(rec.clone());
            max = rec.t;
        };
        temp
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(self.box_min, self.box_max);
        return true;
    }
}
