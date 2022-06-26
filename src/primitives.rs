use crate::vec3::*;
use crate::ray::*;
use crate::material::*;

#[derive(Clone, Copy)]
pub struct Plane {
    pub p0: Vec3, 
    pub normal: Vec3,
    pub material: Material
}

impl Plane {
    pub fn new(p0: Vec3, normal: Vec3, material: Material) -> Plane {
        Plane{p0, normal, material}
    }

    pub fn hit(self, ray: &Ray) -> f32{

        //almost zero (hehe haha)
        let aZero:f32 = 0.00000001;

        let normal = self.normal;
        let denom = Vec3::dot(normal, ray.direction());
        if denom > aZero {
            let v = self.p0 - ray.origin();
            let dist = Vec3::dot(v, normal) / denom;
            if dist > 0. {
                return dist;
            }
        }
        return -1.; 
    }

    pub fn t(self, ray: &Ray) -> f32 {
        return self.hit(&ray);
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3, 
    pub v2: Vec3,
    pub material: Material
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Triangle {
        Triangle {v0, v1, v2, material}
    }

    pub fn hit(self, ray: &Ray) -> Vec<Vec4> {

        //almost zero (hehe haha)
        let aZero:f32 = 0.00000001;

        //compute un-normalized normal
        let orig = ray.origin();
        let dir = ray.direction();
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let pvec = Vec3::cross(dir, v0v2);
        let det = Vec3::dot(v0v1, pvec);

        if det.abs() < aZero {
            return vec![Vec4::new(0.,0.,0.,0.)]}

        let invDet = 1. / det;

        let tvec = orig - self.v0;
        let u = Vec3::dot(tvec, pvec) * invDet;
        if u < 0. || u > 1. {
            return vec![Vec4::new(0.,0.,0.,0.)]}
        
        let qvec = Vec3::cross(tvec, v0v1);
        let v = Vec3::dot(dir, qvec) * invDet;
        if v < 0. || u + v > 1. {
            return vec![Vec4::new(0.,0.,0.,0.)]}
        
        let t = Vec3::dot(v0v2, qvec) * invDet;
        
        let pNorm = Vec3::unit_vec(Vec3::cross(self.v1-self.v0, self.v2-self.v0));

        return vec![Vec4::new(u, v, 1., t), Vec4::new(pNorm.x, pNorm.y, pNorm.z, 0.)];

    }

    pub fn t(self, ray: &Ray) -> f32 {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let pvec = Vec3::cross(ray.direction(), v0v2);
        let det = Vec3::dot(v0v1, pvec);
        if det.abs() < 0.00000001 {return 0.}
        let tvec = ray.origin() - self.v0;
        let u = Vec3::dot(tvec, pvec) / det;
        if u < 0. || u > 1. {return 0.;}
        let qvec = Vec3::cross(tvec, v0v1);
        let v = Vec3::dot(ray.direction(), qvec) / det;
        if v < 0. || u + v > 1. {return 0.;}
        return Vec3::dot(v0v2, qvec) / det;
    }
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3, 
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {center, radius, material}
    }

    pub fn hit(self, ray: &Ray) -> f32 {
        let oc: Vec3 = ray.origin() - self.center;
        let a= Vec3::dot(ray.direction(), ray.direction());
        let b= 2.0 * Vec3::dot(oc, ray.direction());
        let c= Vec3::dot(oc, oc) - self.radius * self.radius;
        let disc= b*b - 4.*a*c;

        if disc < 0. {
            return -1.0;
        } else {
            return (-b - disc.sqrt()) / (2.0 * a);
        }
    }

    pub fn t(self, ray: &Ray) -> f32 {
        return self.hit(&ray);
    }
}