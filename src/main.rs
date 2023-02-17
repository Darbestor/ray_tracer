use std::{f32::consts::PI, rc::Rc};

use rand::{thread_rng, Rng};
use rust_ray_tracer::{
    math::vec3::Vec3,
    ppm::{color::Color, image::PpmImage},
    raytracing::{
        camera::Camera,
        material::{MatDielectric, MatLabmertian, MatMetalic, Material, MaterialScatter},
        ray::Ray,
        ray_hit::RayHitTester,
        sphere::Sphere,
        world::WorldObjects,
    },
};

fn ray_pixel_color(ray: &Ray, objects: &WorldObjects, depth: usize) -> Vec3 {
    if depth == 0 {
        return Vec3::zero();
    }

    if let Some(hit) = objects.hit(ray, 0.001, f32::INFINITY) {
        if let Some(scatter_result) = hit.material.scatter(ray, &hit) {
            return scatter_result.attenuation
                * ray_pixel_color(&scatter_result.ray, objects, depth - 1);
        } else {
            return Vec3::zero();
        }
    }

    let unit_direction = ray.direction.norm();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // Blend between white and blue
    (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    // Constants
    let aspect_ratio: f32 = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio) as usize;
    let samples_per_pixel: f32 = 100.;
    let max_ray_bounces = 50;

    //Materials
    let material_ground = Rc::new(Material::Labmertian(MatLabmertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    }));
    let material_center = Rc::new(Material::Labmertian(MatLabmertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    }));
    let material_left = Rc::new(Material::Dielectric(MatDielectric {
        refraction_index: 1.7,
    }));
    let material_right = Rc::new(Material::Metalic(MatMetalic::new(
        Vec3::new(0.8, 0.6, 0.2),
        1.0,
    )));

    let mut ppm = PpmImage::new(width, height);

    // Camera
    let lookfrom = Vec3::new(3., 3., 2.);
    let lookat = Vec3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let vfov = 20.0;
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let world_objects = WorldObjects {
        objects: vec![
            Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, material_center)),
            Box::new(Sphere::new(
                Vec3::new(0., -100.5, -1.),
                100.,
                material_ground,
            )),
            Box::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, material_left)),
            Box::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, material_right)),
        ],
    };

    let mut rng = thread_rng();
    for j in 0..height {
        println!("Processing: {}...{}", j + 1, height);
        for i in 0..width {
            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel as usize {
                let x = (i as f32 + rng.gen::<f32>()) / (width - 1) as f32;
                let y = ((height - j) as f32 + rng.gen::<f32>()) / (height - 1) as f32;
                pixel_color +=
                    ray_pixel_color(&camera.get_ray(x, y), &world_objects, max_ray_bounces);
            }
            ppm.pixels[j * width + i] =
                Color::sampled_color(pixel_color, samples_per_pixel).unwrap();
        }
    }
    let mut path = std::env::current_dir().unwrap();
    path.push("images");
    path.push("camera.ppm");
    ppm.save(path).unwrap();
}
