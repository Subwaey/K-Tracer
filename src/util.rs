use crate::vec3::*;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {return min};
    if x > max {return max};
    return x;
}

pub fn angleAxis3x3(angle: f32, axis: Vec3) -> Vec<Vec3> {
    let c = angle.cos();
    let s = angle.sin();

    let t = 1. - c;
    let x = axis.x;
    let y = axis.y;
    let z = axis.z;

    return vec![Vec3::new(t * x * x + c,      t * x * y - s * z,  t * x * z + s * y),
                Vec3::new(t * x * y + s * z,  t * y * y + c,      t * y * z - s * x),
                Vec3::new(t * x * z - s * y,  t * y * z + s * x,  t * z * z + c)
    ];
}