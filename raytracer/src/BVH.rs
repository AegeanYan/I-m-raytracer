use crate::{rtweekend, Ray, random_int};
use crate::Vec3;
use crate::hit;
use crate::AABB::Aabb;
use std::sync::Arc;
use crate::hit::{Hittable, HitRecord, HittableList};
use std::vec;
use std::mem::swap;
use crate::moving_sphere::MovingSphere;

pub struct BvhNode{
    pub left:Arc<dyn Hittable>,
    pub right:Arc<dyn Hittable>,
    pub box0:Aabb,
}

impl Hittable for BvhNode{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.box0.hit(r , t_min , t_max) {
            return false;
        }
        let hit_left:bool = self.left.hit(r , t_min , t_max , rec);
        let hit_right:bool;
        if hit_left {
            hit_right = self.right.hit(r , t_min , rec.t , rec);
        }else {
            hit_right = self.right.hit(r , t_min , t_max , rec);
        }
        return hit_left || hit_right;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.maximum = self.box0.maximum;
        output_box.minimum = self.box0.minimum;
        return true;
    }
}

impl BvhNode{
    pub fn new(src_objects:& mut Vec<Arc<dyn Hittable>> , start:usize , end:usize , time0:f64 , time1:f64)->Self{
        let mut objects = src_objects.clone();
        let axis:i32 = random_int(0 , 2);
        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            _ => BvhNode::box_z_compare,
        };
        let object_span:usize = end - start;
        let lt:Arc<dyn Hittable>;
        let rt:Arc<dyn Hittable>;
        match object_span {
            1 => {
                lt = objects[start as usize].clone();
                rt = objects[start as usize].clone();
            }
            2 => {
                if comparator(objects[start as usize].clone() , objects[(start + 1) as usize].clone()) {
                    lt = objects[start as usize].clone();
                    rt = objects[(start + 1) as usize].clone();
                }else {
                    lt = objects[(start + 1) as usize].clone();
                    rt = objects[start as usize].clone();
                }
            }
            _ => {
                quick_sort(&mut objects , start , end , comparator);

                let mid = start + object_span / 2;
                lt = Arc::new(BvhNode::new(&mut objects , start , mid , time0 , time1));
                rt = Arc::new(BvhNode::new(&mut objects , mid , end , time0 , time1));
            }
        }
        let mut box_left:Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };
        let mut box_right:Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };
        if !lt.bounding_box(time0 , time1 , &mut box_left)||!rt.bounding_box(time0 , time1 , &mut box_right) {
            std::println!("Bo bounding box in bvh_node constructor.\n");
        }
        Self{
            left: lt,
            right: rt,
            box0: MovingSphere::surrounding_box(box_left , box_right),
        }
    }
    pub fn box_compare(a:Arc<dyn Hittable> , b:Arc<dyn Hittable> , axis:i32)->bool{
        let mut box_a:Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };
        let mut box_b:Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };

        if !a.bounding_box(0.0 , 0.0 ,&mut box_a) || !b.bounding_box(0.0 , 0.0 , &mut box_b) {
            std::println!("No bounding box in bvh_node constructor.\n");
        }
        return match axis {
            0 => box_a.minimum.x < box_b.minimum.x,
            1 => box_a.minimum.y < box_b.minimum.y,
            2 => box_a.minimum.z < box_b.minimum.z,
            _ => {true}
        }
    }
    pub fn box_x_compare(a:Arc<dyn Hittable> , b:Arc<dyn Hittable>)->bool{
        return BvhNode::box_compare(a , b , 0);
    }
    pub fn box_y_compare(a:Arc<dyn Hittable> , b:Arc<dyn Hittable>)->bool{
        return BvhNode::box_compare(a , b , 1);
    }
    pub fn box_z_compare(a:Arc<dyn Hittable> , b:Arc<dyn Hittable>)->bool{
        return BvhNode::box_compare(a , b , 2);
    }
}

fn quick_sort(mut arr:& mut Vec<Arc<dyn Hittable>> , l:usize , r:usize , func:fn(Arc<dyn Hittable> , Arc<dyn Hittable>)->bool){
    let mut left:usize = l;
    let mut right:usize = r;
    let mut mid:Arc<dyn Hittable> = arr[((l + r) >> 1) as usize].clone();
    while left < right {
        while func(arr[left as usize].clone() , mid.clone()) {
            left += 1;
        }
        while func(mid.clone() , arr[right as usize].clone()) {
            right += 1;
        }
        if left <= right {
            swap(&mut arr[left] , &mut arr[right]);
            // let temp:&Arc<dyn Hittable> = &arr[left];
            // &arr[left] = &arr[right];
            // &arr[right] = &temp;
            left += 1;
            right += 1;
        }
    }
    if right > l {quick_sort(arr , l , right , func)};
    if left < r {quick_sort(arr , left , r , func) };
}