use crate::vec3::*;

/*
NOTES:

1. Only spheres support emissive material
2. Reflectivity goes from 0 -> 1

*/

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Vec3,
    pub reflectivity: f32,
    pub emissivity: f32
}

impl Material {
    pub fn new(color: Vec3, reflectivity: f32, emissivity: f32) -> Material {
        Material { color, reflectivity, emissivity }
    }
}