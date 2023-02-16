use rand::{thread_rng, Rng};
use rust_ray_tracer::{
    math::{random_in_unit_sphere, vec3::Vec3},
    ppm::{color::Color, image::PpmImage},
    raytracing::{
        camera::Camera, ray::Ray, ray_hit::RayHitTester, sphere::Sphere, world::WorldObjects,
    },
};

fn ray_pixel_color(ray: &Ray, objects: &WorldObjects, depth: usize) -> Vec3 {
    if depth == 0 {
        return Vec3::zero();
    }

    if let Some(hit) = objects.hit(ray, 0.001, f32::INFINITY) {
        let target = hit.location + hit.normal + random_in_unit_sphere().norm();
        let normal_color_vec = 0.5
            * ray_pixel_color(
                &Ray::new(hit.location, target - hit.location),
                objects,
                depth - 1,
            );
        return normal_color_vec;
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

    let mut ppm = PpmImage::new(width, height);
    let camera = Camera::new(2.0, aspect_ratio * 2.0);

    let world_objects = WorldObjects {
        objects: vec![
            Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
            Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
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
    path.push("lambertian_reflection.ppm");
    ppm.save(path).unwrap();
}
