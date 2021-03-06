use crate::hit::{HitRecord, Hittable, HittableList};
use crate::material::Lambertian;
use crate::moving_sphere::MovingSphere;
use crate::Vec3;
use crate::AABB::Aabb;
use crate::{random_int, Ray};
use std::sync::Arc;

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box0: Aabb,
}
impl BvhNode {
    pub fn new(mut list: HittableList, time0: f64, time1: f64) -> Self {
        let len = list.objects.len();
        // Self:BvhNode::new0(&mut list.objects , 0 , len , time0 , time1);
        BvhNode::new0(&mut list.objects, 0, len, time0, time1)
    }
}
impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.box0.hit(r, t_min, t_max) {
            return None;
        }
        if let Some(rec_tmp) = self.left.hit(r, t_min, t_max) {
            if let Some(rec_) = self.right.hit(r, t_min, rec_tmp.t) {
                return Some(rec_);
            } else {
                return Some(rec_tmp);
            }
        }
        if let Some(rec_tmp) = self.right.hit(r, t_min, t_max) {
            return Some(rec_tmp);
        }
        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        // output_box.maximum = self.box0.maximum;
        // output_box.minimum = self.box0.minimum;
        *output_box = self.box0;
        true
    }
}

impl BvhNode {
    pub fn new0(
        src_objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut objects = src_objects.clone();
        let axis: i32 = random_int(0, 2);
        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            _ => BvhNode::box_z_compare,
        };
        let object_span: usize = end - start;
        let lt: Arc<dyn Hittable>;
        let rt: Arc<dyn Hittable>;
        match object_span {
            1 => {
                lt = objects[start as usize].clone();
                rt = objects[start as usize].clone();
            }
            2 => {
                if comparator(
                    objects[start as usize].clone(),
                    objects[(start + 1) as usize].clone(),
                ) {
                    lt = objects[start as usize].clone();
                    rt = objects[(start + 1) as usize].clone();
                } else {
                    lt = objects[(start + 1) as usize].clone();
                    rt = objects[start as usize].clone();
                }
            }
            _ => {
                //quick_sort(&mut objects , start , end , comparator);
                objects.sort_by(|a, b| {
                    let mut xi = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
                    a.bounding_box(time0, time1, &mut xi);
                    let x = xi.minimum.get(axis);
                    let mut yi = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
                    b.bounding_box(time0, time1, &mut yi);
                    let y = yi.minimum.get(axis);
                    x.partial_cmp(&y).unwrap()
                });

                let mid = start + object_span / 2;
                lt = Arc::new(BvhNode::new0(&mut objects, start, mid, time0, time1));
                rt = Arc::new(BvhNode::new0(&mut objects, mid, end, time0, time1));
            }
        }
        let mut box_left: Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let mut box_right: Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
        if !lt.bounding_box(time0, time1, &mut box_left)
            || !rt.bounding_box(time0, time1, &mut box_right)
        {
            std::println!("Bo bounding box in bvh_node constructor.\n");
        }
        Self {
            left: lt,
            right: rt,
            box0: MovingSphere::<Lambertian>::surrounding_box(box_left, box_right),
        }
    }
    pub fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: i32) -> bool {
        let mut box_a: Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let mut box_b: Aabb = Aabb {
            minimum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            maximum: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            std::println!("No bounding box in bvh_node constructor.\n");
        }
        match axis {
            0 => box_a.minimum.x < box_b.minimum.x,
            1 => box_a.minimum.y < box_b.minimum.y,
            2 => box_a.minimum.z < box_b.minimum.z,
            _ => true,
        }
    }
    pub fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a, b, 2)
    }
}

// fn quick_sort(mut arr:& mut Vec<Arc<dyn Hittable>> , l:usize , r:usize , func:fn(Arc<dyn Hittable> , Arc<dyn Hittable>)->bool){
//     let mut left:usize = l;
//     let mut right:usize = r;
//     let mut mid:Arc<dyn Hittable> = arr[((l + r) >> 1) as usize].clone();
//     while left < right {
//         while func(arr[left as usize].clone() , mid.clone()) {
//             left += 1;
//         }
//         while func(mid.clone() , arr[right as usize].clone()) {
//             right += 1;
//         }
//         if left <= right {
//             swap(&mut arr[left] , &mut arr[right]);
//             // let temp:&Arc<dyn Hittable> = &arr[left];
//             // &arr[left] = &arr[right];
//             // &arr[right] = &temp;
//             left += 1;
//             right += 1;
//         }
//     }
//     if right > l {quick_sort(arr , l , right , func)};
//     if left < r {quick_sort(arr , left , r , func) };
// }
