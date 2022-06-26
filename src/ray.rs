use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub A: Vec3, 
    pub B: Vec3
}

impl Ray {
    pub fn ray(a: Vec3, b: Vec3) -> Ray {
        Ray { A: a, B: b }
    }

    pub fn origin(self) -> Vec3 {
        self.A
    }

    pub fn direction(self) -> Vec3 {
        self.B
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.A + (self.B * t)
    }

    pub fn reflect(normal: Vec3, incident: Vec3, intersection: Vec3, bias: f32) -> Ray {
        Ray { A: intersection + (normal * bias), B: Vec3::unit_vec(incident - (normal * 2.0 * Vec3::dot(incident,normal)))}
    }
}