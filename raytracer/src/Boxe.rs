use crate::{rtweekend, Vec3, Ray};
use crate::aarect;
use crate::HittableList;
use std::sync::Arc;
use crate::material::Material;
use crate::aarect::{XyRect, XzRect, YzRect};
use crate::hit::{Hittable, HitRecord};
use crate::AABB::Aabb;

pub struct Boxes{
    pub box_min:Vec3,
    pub box_max:Vec3,
    pub sides:HittableList,
}

impl Boxes{
    pub fn new(p0:Vec3 , p1:Vec3 , ptr:Arc<dyn Material>)->Self{
        let mut s:HittableList = HittableList { objects: vec![] };
        s.add(Arc::new(XyRect::new(p0.x , p1.x , p0.y , p1.y , p1.z , ptr.clone())));
        s.add(Arc::new(XyRect::new(p0.x , p1.x , p0.y , p1.y , p0.z , ptr.clone())));

        s.add(Arc::new(XzRect::new(p0.x , p1.x , p0.z , p1.z , p1.y , ptr.clone())));
        s.add(Arc::new(XzRect::new(p0.x , p1.x , p0.z , p1.z , p0.y , ptr.clone())));

        s.add(Arc::new(YzRect::new(p0.y , p1.y , p0.z , p1.z , p1.x , ptr.clone())));
        s.add(Arc::new(YzRect::new(p0.y , p1.y , p0.z , p1.z , p0.x , ptr.clone())));

        Self{
            box_min: p0,
            box_max: p1,
            sides: s,
        }
    }
}
impl Hittable for Boxes{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        return self.sides.hit(r , t_min , t_max , rec);
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(self.box_min , self.box_max);
        return true;
    }
}