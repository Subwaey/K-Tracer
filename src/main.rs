//**************************************
//FOCUS OF THE DAY: Random Little Fixies
//**************************************

//TODO: Bounce Lighting for Surfaces Not in Direct Light (2)
//TODO: Shadow Bilateral Blur (3)
//TODO: Optimization Session (1)

mod vec3;
mod ray;
mod primitives;
mod world;
mod light;
mod util;
mod material;

use rand::{thread_rng, Rng};
use vec3::*;
use ray::*;
use primitives::*;
use world::*;
use light::*;
use util::*;
use material::*;

fn main() {

    // World
    let floorMaterial = Material::new(Vec3::new(0.5, 0.5, 0.5), 0., 0.);
    let ballMaterial1 = Material::new(Vec3::new(0.3, 0.9, 1.), 0.6, 0.);
    let ballMaterial2 = Material::new(Vec3::new(0.9, 0.6, 0.2), 0., 0.);
    let triangleMaterial = Material::new(Vec3::new(0.3, 0.8, 0.), 0., 0.);
    let world = World::new(vec![Plane::new(Vec3::new(0., -1.1, 0.), Vec3::new(0., -1., 0.), floorMaterial)], 
                                 vec![Sphere::new(Vec3::new(0., -0.6, -2.), 0.5, ballMaterial1), Sphere::new(Vec3::new(-0.8, -0.6, -2.), 0.3, ballMaterial2)], 
                               vec![Triangle::new( Vec3::new(-1., -0.45, -1.), Vec3::new(-0.5, -0.5, -1.5), Vec3::new(0., -0.45, -1.), triangleMaterial)], 
                                  vec![Light::new(Vec3::new(0.25, 1., -0.5), Vec3::new(1., 1., 1.), 1., 0.2), Light::new(Vec3::new(-1., 1., -0.5), Vec3::new(1., 1., 1.), 1., 0.2)]);

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let h = 360;
    let w = (h as f32 * aspect_ratio) as i32;
    let samples = 30.;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., focal_length);

    // Render

    println!("P3\n{} {}\n{}", w, h, 255);

    for j in (0..h).rev() {
        for i in 0..w {
            let mut pixel_color = Vec3::new(0., 0., 0.);
            for s in 0..samples as i32 {

                let mut rng = thread_rng();
                let y:u8 = rng.gen_range(0..=1);
                let x:u8 = rng.gen_range(0..=1);
                let u = ((i as f32) + (x as f32)/2.) / ((w - 1) as f32);
                let v = ((j as f32) + (y as f32)/2.) / ((h - 1) as f32);
                
                let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);

                pixel_color = pixel_color + World::color(&world, &r);
                
            }
            let color = pixel_color / samples;

            let ir: i32 = (256. * clamp(color.x, 0., 0.999)) as i32;
            let ig: i32 = (256. * clamp(color.y, 0., 0.999)) as i32;
            let ib: i32 = (256. * clamp(color.z, 0., 0.999)) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
