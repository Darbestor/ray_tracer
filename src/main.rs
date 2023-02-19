use std::sync::Arc;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use rust_ray_tracer::raytracing::renderer::Renderer;
use rust_ray_tracer::{
    math::vec3::Vec3,
    ppm::{color::Color, image::PpmImage},
    raytracing::{
        camera::Camera,
        material::{MatDielectric, MatLabmertian, MatMetalic, Material},
        sphere::Sphere,
        world::WorldObjects,
    },
};

fn main() {
    // Constants
    let aspect_ratio: f32 = 3.0 / 2.0;
    let width = 1200;
    let height = (width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_ray_bounces = 50;

    // Camera
    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let rotation = Vec3::new(0., 1., 0.);
    let vfov = 20.0;
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        rotation,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut renderer = Renderer::init(camera, samples_per_pixel, max_ray_bounces);

    renderer.objects = random_scene();

    let scene = renderer.render(width, height, true);

    save_to_ppm("refactored.ppm", width, height, scene);
}

#[allow(dead_code)]
fn test_scene() -> WorldObjects {
    //Materials
    let material_ground = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    }));
    let material_center = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    }));
    let material_left = Arc::new(Material::Dielectric(MatDielectric {
        refraction_index: 1.7,
    }));
    let material_right = Arc::new(Material::Metalic(MatMetalic::new(
        Vec3::new(0.8, 0.6, 0.2),
        1.0,
    )));

    // Objects
    WorldObjects {
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
    }
}

#[allow(dead_code)]
fn random_scene() -> WorldObjects {
    let mut world = WorldObjects::new();

    let ground_material = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    }));
    world.objects.push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    let mut rng = StdRng::seed_from_u64(1);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random(0., 1.) * Vec3::random(0., 1.);
                    Arc::new(Material::Labmertian(MatLabmertian { albedo }))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random(0.5, 1.);
                    let roughness = rng.gen_range(0.0..0.5);
                    Arc::new(Material::Metalic(MatMetalic { albedo, roughness }))
                } else {
                    // glass
                    Arc::new(Material::Dielectric(MatDielectric {
                        refraction_index: 1.5,
                    }))
                };
                world
                    .objects
                    .push(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material = Arc::new(Material::Dielectric(MatDielectric {
        refraction_index: 1.5,
    }));
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1.0, material)));

    let material = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    }));
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1.0, material)));

    let material = Arc::new(Material::Metalic(MatMetalic {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        roughness: 0.0,
    }));
    world
        .objects
        .push(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1.0, material)));
    world
}

fn save_to_ppm(filename: &str, width: usize, height: usize, scene: Vec<Vec3>) {
    let mut ppm = PpmImage::new(width, height);
    ppm.pixels = scene
        .into_iter()
        .map(|p| Color::try_from(p).unwrap())
        .collect();

    let mut path = std::env::current_dir().unwrap();
    path.push("images");
    path.push(filename);
    ppm.save(path).unwrap();
}
