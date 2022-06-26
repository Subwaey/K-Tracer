use std::ops;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {x, y, z, w}
    }

    pub fn to_Vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl ops::Add for Vec4 {
    type Output = Self;

    fn add(self, other: Vec4) -> Self::Output {
        Vec4 { x: (self.x + other.x),
               y: (self.y + other.y),
               z: (self.z + other.z),
               w: (self.w + other.w)
             }
    }
}

impl ops::Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Vec4 { x: (self.x / other),
               y: (self.y / other),
               z: (self.z / other),
               w: (self.w / other)
             }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn unit_vec(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn dot(u: Vec3, v: Vec3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {x: u.y * v.z - u.z * v.y,
              y: u.z * v.x - u.x * v.z,
              z: u.x * v.y - u.y * v.x}
    }

    pub fn to_Vec4(self, other: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, other)
    }

    pub fn random(min: usize, max1: usize) -> Vec3 {
        let max = max1 * 100;
        let mut rng = thread_rng();
        let tx = rng.gen_range(0..=1);
        let ty = rng.gen_range(0..=1);
        let tz = rng.gen_range(0..=1);
        let mut x = (rng.gen_range(min..=max) as f32) / 100.;
        let mut y = (rng.gen_range(min..=max) as f32) / 100.;
        let mut z = (rng.gen_range(min..=max) as f32) / 100.;
        
        if tx == 0 {x *= -1.;}
        if ty == 0 {y *= -1.;}
        if tz == 0 {z *= -1.;}
        
        return Vec3::new(x, y, z);
    }

}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 { x: (self.x + other.x),
               y: (self.y + other.y),
               z: (self.z + other.z)
             }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 { x: (self.x - other.x),
               y: (self.y - other.y),
               z: (self.z - other.z)
             }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 { x: (self.x * other.x),
               y: (self.y * other.y),
               z: (self.z * other.z)
             }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Vec3 { x: (self.x * other),
               y: (self.y * other),
               z: (self.z * other)
             }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Vec3 { x: (self.x / other),
               y: (self.y / other),
               z: (self.z / other)
             }
    }
}