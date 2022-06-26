use std::f32::consts::PI;

use crate::primitives::*;
use crate::vec3::*;
use crate::ray::*;
use crate::light::*;
use crate::util::*;

pub struct World {

    pub planes: Vec<Plane>,
    pub spheres: Vec<Sphere>,
    pub triangles: Vec<Triangle>,
    pub lights: Vec<Light>,

}

impl World {
    pub fn new(planes: Vec<Plane>, spheres: Vec<Sphere>, triangles: Vec<Triangle>, lights: Vec<Light>) -> World {
        World { planes, spheres, triangles, lights}
    }

    pub fn hit(world: &World, ray: &Ray) -> Vec4 {
        let mut worldLights: Vec<Light> = vec![];
        for l in 0..world.lights.len() {
            worldLights.push(world.lights[l]);
        }
        let mut t_buffer: Vec<f32> = vec![];
        let mut small_t_index: usize = 0;
        let mut small_t: f32 = 0.;
        let mut type_buffer: Vec<Vec3> = vec![];
        let mut vote: f32 = 0.;
        //type buffer stores Vec3([primitive type],[index in world object list for said primitive type],[0.0])

        fn vote_for_sky(votes: f32, world: &World, ray: &Ray) -> Vec4 {
            if votes == (world.triangles.len()+world.spheres.len()+world.planes.len()) as f32 {
                return RenderSky(ray);
            } else {
                return Vec4::new(-1., -1., -1., -1.);
            }
        }

        for i in 0..world.triangles.len() {
            let tT = world.triangles[i].t(&ray);
            if tT > 0. {
                t_buffer.push(tT);
                type_buffer.push(Vec3::new(0.,i as f32,0.));
            }else {
                vote += 1.;
            }
        }
        for i in 0..world.spheres.len() {
            if world.spheres[i].material.emissivity > 0. {
                worldLights.push(Light::new(world.spheres[i].center, world.spheres[i].material.color, world.spheres[i].material.emissivity * 5., world.spheres[i].radius + 99.));
            }
            let tS = world.spheres[i].t(&ray);
            if tS > 0. {
                t_buffer.push(tS);
                type_buffer.push(Vec3::new(1.,i as f32,0.));
            }else {
                vote += 1.;
            }
        }
        for i in 0..world.planes.len() {
            let tP = world.planes[i].t(&ray);
            if tP > 0. {
                t_buffer.push(tP);
                type_buffer.push(Vec3::new(2.,i as f32,0.));
            }else {
                vote += 1.;
            }
        }
        
        let vfs = vote_for_sky(vote, world, ray);

        if vfs != Vec4::new(-1., -1., -1., -1.) {
            return vfs;
        }

        small_t = (t_buffer.iter().fold(f64::INFINITY, |a, &b| a.min(b.into()))) as f32;
        while t_buffer[small_t_index] != small_t {
            small_t_index += 1;
        }

        let mut dir_to_light: Vec<Vec3> = vec![];
        for l in 0..worldLights.len() {
            dir_to_light.push(Vec3::unit_vec(worldLights[l].position - (ray.origin() + (ray.direction()*small_t))));
        }

        if type_buffer[small_t_index].x == 0. {
            return RenderTriangle(type_buffer[small_t_index].y, world, ray, dir_to_light, worldLights);
        }else if type_buffer[small_t_index].x == 1. {
            return RenderSphere(type_buffer[small_t_index].y, world, ray, dir_to_light, worldLights);
        }else {
            return RenderPlane(type_buffer[small_t_index].y, world, ray, dir_to_light, worldLights);
        }

        fn RenderTriangle(t: f32, world: &World, ray: &Ray, dir_to_light: Vec<Vec3>, worldLights: Vec<Light>) -> Vec4 {
            let tempy = world.triangles[t as usize].hit(&ray);
            if tempy[0].z == 1. {
                let mut return_buffer:Vec<Vec4> = vec![];
                let mut hitT = Vec3::new(0.,0.,0.);
                let mut normalT = Vec3::new(0.,0.,0.);
                for l in 0..worldLights.len() {
                    //DIFFUSE SHADER 2.0 :: Better Lighting
                    //https://medium.com/@alexander.wester/ray-tracing-soft-shadows-in-real-time-a53b836d123b
                    let t1 = tempy[0].w;
                    hitT = ray.origin() + (ray.direction() * t1);
                    let mut perpL = Vec3::cross(dir_to_light[l], Vec3::new(0.,1.,0.));
                    if perpL == Vec3::new(0.,0.,0.) {perpL.x = 1.;}
                    let to_light_edge = Vec3::unit_vec((worldLights[l].position + perpL * worldLights[l].radius) - hitT);
                    let coneAngle = (Vec3::dot(dir_to_light[l], to_light_edge)).acos() * 2.;
                    normalT = tempy[1].to_Vec3() * -1.;
                    let shadow_ray = Ray::ray(hitT + (normalT * 0.00001), getConeSample(dir_to_light[l], coneAngle));
                    let mut light_intensity: f32 = worldLights[l].intensity;
                    for i in 0..world.spheres.len() {
                        if Sphere::hit(world.spheres[i], &shadow_ray) > 0. {
                            light_intensity *= 0.1;
                        }
                    }
                    for i in 0..world.triangles.len() {
                        if i as f32 != t {
                            let thit = Triangle::hit(world.triangles[i], &shadow_ray);
                            if thit[0].z > 0. {
                                light_intensity *= 0.1;
                            }
                        }
                    }
                    
                    let light_powT = Vec3::dot(normalT, dir_to_light[l]).max(0.0) * light_intensity;
                    let tempColT = world.triangles[t as usize].material.color.clone() * worldLights[l].color.clone() * light_powT;
                    return_buffer.push(Vec4::new(tempColT.x, tempColT.y, tempColT.z, 1.));
                }
                let mut r = Vec4::new(0.,0.,0.,0.);
                for b in 0..return_buffer.len() {
                    r = r + return_buffer[b];
                }
                r = r / return_buffer.len() as f32;
                let mut color = r.to_Vec3();
                let reflectivity = clamp(world.triangles[t as usize].material.reflectivity, 0., 1.);
                if reflectivity > 0. {
                    let bounce_ray = Ray::reflect(normalT, ray.direction(), hitT, -0.000001);
                    color = color * (1. - reflectivity);
                    let tempHit = World::hit(world, &bounce_ray);
                    color = color + (tempHit.to_Vec3() * reflectivity);
                }
                return Vec4::new(color.x, color.y, color.z, 1.);
            }
            return RenderSky(ray);
        }

        fn RenderSphere(s: f32, world: &World, ray: &Ray, dir_to_light: Vec<Vec3>, worldLights: Vec<Light>) -> Vec4 {
            let mut return_buffer:Vec<Vec4> = vec![];
            let mut hitS = Vec3::new(0.,0.,0.);
            let mut normalS = Vec3::new(0.,0.,0.);
            for l in 0..worldLights.len() {
                if world.spheres[s as usize].material.emissivity > 0. {
                    return (world.spheres[s as usize].material.color * world.spheres[s as usize].material.emissivity).to_Vec4(1.);
                }
                let t1 = world.spheres[s as usize].hit(&ray);
                hitS = ray.at(t1);
                let mut perpL = Vec3::cross(dir_to_light[l], Vec3::new(0.,1.,0.));
                if perpL == Vec3::new(0.,0.,0.) {perpL.x = 1.;}
                let toLightEdge = Vec3::unit_vec((worldLights[l].position + perpL * worldLights[l].radius) - hitS);
                let coneAngle = (Vec3::dot(dir_to_light[l], toLightEdge)).acos() * 2.;
                normalS = Vec3::unit_vec(ray.at(t1) - world.spheres[s as usize].center);
                let shadow_ray = Ray::ray(hitS + (normalS * 0.00001), getConeSample(dir_to_light[l], coneAngle));
                let mut light_intensity: f32 = worldLights[l].intensity;
                for i in 0..world.spheres.len() {
                    if i as f32 != s {
                        if Sphere::hit(world.spheres[i], &shadow_ray) > 0. {
                            light_intensity *= 0.1;
                        }
                    }
                }
                for i in 0..world.triangles.len() {
                    let thit = Triangle::hit(world.triangles[i], &shadow_ray);
                    if thit[0].z > 0. {
                        light_intensity *= 0.1;
                    }
                }

                let light_powS = Vec3::dot(normalS, dir_to_light[l]).max(0.0) * light_intensity;
                let tempColS = world.spheres[s as usize].material.color.clone() * worldLights[l].color.clone() * light_powS;
                return_buffer.push(Vec4::new(tempColS.x, tempColS.y, tempColS.z, 1.));
            }
            let mut r = Vec4::new(0.,0.,0.,0.);
            for b in 0..return_buffer.len() {
                r = r + return_buffer[b];
            }
            r = r / return_buffer.len() as f32;
            let mut color = r.to_Vec3();
            let reflectivity = clamp(world.spheres[s as usize].material.reflectivity, 0., 1.);
            if reflectivity > 0. {
                let bounce_ray = Ray::reflect(normalS, ray.direction(), hitS, -0.000001);
                color = color * (1. - reflectivity);
                let tempHit = World::hit(world, &bounce_ray);
                color = color + (tempHit.to_Vec3() * reflectivity);
            }
            return Vec4::new(color.x, color.y, color.z, 1.);
        }

        fn RenderPlane(p: f32, world: &World, ray: &Ray, dir_to_light: Vec<Vec3>, worldLights: Vec<Light>) -> Vec4 {
            let mut return_buffer:Vec<Vec4> = vec![];
            let mut hitP = Vec3::new(0.,0.,0.);
            let mut normalP = Vec3::new(0.,0.,0.);
            for l in 0..worldLights.len() {
                let t1 = world.planes[p as usize].hit(&ray);
                hitP = ray.origin() + (ray.direction() * t1);
                let mut perpL = Vec3::cross(dir_to_light[l], Vec3::new(0.,1.,0.));
                if perpL == Vec3::new(0.,0.,0.) {perpL.x = 1.;}
                let toLightEdge = Vec3::unit_vec((worldLights[l].position + perpL * worldLights[l].radius) - hitP);
                let coneAngle = (Vec3::dot(dir_to_light[l], toLightEdge)).acos() * 2.;
                normalP = Vec3::unit_vec(world.planes[p as usize].normal * -1.);
                let shadow_ray = Ray::ray(hitP + (normalP * 0.00001), getConeSample(dir_to_light[l], coneAngle));
                let mut light_intensity: f32 = worldLights[l].intensity;
                for i in 0..world.spheres.len() {
                    if Sphere::hit(world.spheres[i], &shadow_ray) > 0. {
                        light_intensity *= 0.1;
                    }
                }
                for i in 0..world.triangles.len() {
                    let thit = Triangle::hit(world.triangles[i], &shadow_ray);
                    if thit[0].z > 0. {
                        light_intensity *= 0.1;
                    }
                }

                
                let light_powP = Vec3::dot(normalP, dir_to_light[l]).max(0.0) * light_intensity;
                let tempColP = world.planes[p as usize].material.color.clone() * worldLights[l].color.clone() * light_powP;
                return_buffer.push(Vec4::new(tempColP.x, tempColP.y, tempColP.z, 1.));
            }
            let mut r = Vec4::new(0.,0.,0.,0.);
            for b in 0..return_buffer.len() {
                r = r + return_buffer[b];
            }
            r = r / return_buffer.len() as f32;
                let mut color = r.to_Vec3();
                let reflectivity = clamp(world.planes[p as usize].material.reflectivity, 0., 1.);
                if reflectivity > 0. {
                    let bounce_ray = Ray::reflect(normalP, ray.direction(), hitP, -0.000001);
                    color = color * (1. - reflectivity);
                    let tempHit = World::hit(world, &bounce_ray);
                    color = color + (tempHit.to_Vec3() * reflectivity);
                }
                return Vec4::new(color.x, color.y, color.z, 1.);
        }

        fn RenderSky(ray: &Ray) -> Vec4 {
            let unit_dir: Vec3 = Vec3::unit_vec(ray.direction());
            let t: f32 = 0.5 * (unit_dir.y + 1.0);
            
            let tempColSky = Vec3::new(1. ,0.7 ,0.5) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
            return Vec4::new(tempColSky.x, tempColSky.y, tempColSky.z, 0.);
        }

        fn getConeSample(dir: Vec3, coneAngle: f32) -> Vec3 {
            let cosAngle = coneAngle.cos();

            let z = (Vec3::random(0, 1).x).abs() * (1. - cosAngle) + cosAngle;
            let phi = (Vec3::random(0, 1).x).abs() * 2. * PI;

            let x = (1. - z * z).sqrt() * (phi).cos();
            let y = (1. - z * z).sqrt() * (phi).sin();
            let north = Vec3::new(0., 0., 1.);

            let axis = Vec3::unit_vec(Vec3::cross(north, Vec3::unit_vec(dir)));
            let angle = Vec3::dot(Vec3::unit_vec(dir), north).acos();

            let r = angleAxis3x3(angle, axis);

            let temp0 = Vec3::new(x, y, z);
            let temp1 = Vec3::new(Vec3::dot(r[0], temp0) , Vec3::dot(r[1], temp0) , Vec3::dot(r[2], temp0));
            return temp1;
        }
    } 

    pub fn color(world: &World, ray: &Ray) -> Vec3 {
        let temp = World::hit(&world, &ray);
        return Vec3::new(temp.x, temp.y, temp.z);
    }
}