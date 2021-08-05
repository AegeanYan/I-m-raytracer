use proc_macro2::TokenStream;
use quote::quote;
use rand::{rngs::SmallRng , Rng , SeedableRng};
use crate::vec3::*;
struct Object{
    pub bounding_box_min:Vec3,
    pub code:TokenStream,
}

fn bvh_build(objects:& mut Vec<Object>) -> TokenStream{
    let axis = rand::thread_rng().gen_range(0 , 3);
    match axis {
        0 => objects.sort_by(|a , b|{
            a.bounding_box_min.x.partial_cmp(&b.bounding_box_min.x).unwrap()
        }),
        1 => objects.sort_by(|a , b|{
            a.bounding_box_min.y.partial_cmp(&b.bounding_box_min.y).unwrap()
        }),
        2 => objects.sort_by(|a , b|{
                        a.bounding_box_min.z.partial_cmp(&b.bounding_box_min.z).unwrap()
        }),
        _ => panic!("axis error"),
    };
    let len = objects.len();
    if len == 1 {
        let tmp = objects.remove(0);
        let code = tmp.code;
        quote! {
            #code
        }
    }else {
        let mut objects2 = objects.split_off(objects.len() / 2);
        let left = bvh_build(objects);
        let right = bvh_build(&mut objects2);
        quote! {
            Box::new(BvhNodeStatic::new(#left , #right , 0.0 , 1.0))
        }
    }
}