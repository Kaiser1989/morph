// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Using

use std::cmp::Ordering;

use nalgebra_glm::*;

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// Functions

pub fn vec4_cmp(a: &Vec4, b: &Vec4) -> Ordering {
    let c = a - b;
    match c.x.partial_cmp(&0.0).unwrap() {
        Ordering::Equal => match c.y.partial_cmp(&0.0).unwrap() {
            Ordering::Equal => match c.z.partial_cmp(&0.0).unwrap() {
                Ordering::Equal => c.w.partial_cmp(&0.0).unwrap(),
                x => x,
            },
            x => x,
        },
        x => x,
    }
}

#[inline]
pub fn project(x: &Vec2, n: &Vec2) -> Vec2 {
    n * dot(&n, &x)
}
