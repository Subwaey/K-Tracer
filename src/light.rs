use crate::vec3::*;

#[derive(Clone, Copy)]
pub struct Light {
    /*
    BUG: Lighting gets gradually glitchier the higher it is on the y axis
    Y58 is the highest the light can be without glitches
     */
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32, 
    pub radius: f32
}

impl Light {
    pub fn new(position: Vec3, color: Vec3, intensity: f32, radius: f32) -> Light {
        Light {position, color, intensity, radius}
    }
}
