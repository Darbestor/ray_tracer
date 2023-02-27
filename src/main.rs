use std::sync::Arc;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use rust_ray_tracer::raytracing::bvh::BvhNode;
use rust_ray_tracer::raytracing::hittable::Hittable;
use rust_ray_tracer::raytracing::moving_sphere::MovingSphere;
use rust_ray_tracer::raytracing::renderer::Renderer;
use rust_ray_tracer::raytracing::texture::checker::Checker;
use rust_ray_tracer::raytracing::texture::solid_color::SolidColor;
use rust_ray_tracer::raytracing::world::WorldObjects;
use rust_ray_tracer::{
    math::vec3::Vec3,
    ppm::{color::Color, image::PpmImage},
    raytracing::{
        camera::Camera,
        material::{MatDielectric, MatLabmertian, MatMetalic, Material},
        sphere::Sphere,
    },
};

fn main() {
    // Constants
    let aspect_ratio: f32 = 16.0 / 9.0;
    let width = 400;
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
    let start_time = 0.;
    let end_time = 1.;

    let camera = Camera::new(
        lookfrom,
        lookat,
        rotation,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        start_time,
        end_time,
    );

    let renderer = Renderer::init(
        camera,
        samples_per_pixel,
        max_ray_bounces,
        Box::new(random_scene(start_time, end_time)),
    );

    let scene = renderer.render(width, height, true);

    save_to_ppm("textures.ppm", width, height, scene);
}

#[allow(dead_code)]
fn test_scene(start_time: f32, end_time: f32) -> BvhNode {
    //Materials
    let material_ground = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(SolidColor::new(0.8, 0.8, 0.0)),
    }));
    let material_center = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(SolidColor::new(0.7, 0.3, 0.3)),
    }));
    let material_left = Arc::new(Material::Dielectric(MatDielectric {
        refraction_index: 1.7,
    }));
    let material_right = Arc::new(Material::Metalic(MatMetalic::new(
        Vec3::new(0.8, 0.6, 0.2),
        1.0,
    )));

    // Objects
    let objects: Vec<Arc<dyn Hittable + Send + Sync>> = vec![
        Arc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, material_center)),
        Arc::new(Sphere::new(
            Vec3::new(0., -100.5, -1.),
            100.,
            material_ground,
        )),
        Arc::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, material_left)),
        Arc::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, material_right)),
    ];
    BvhNode::new(&objects, start_time, end_time).unwrap()
}

#[allow(dead_code)]
fn random_scene(start_time: f32, end_time: f32) -> WorldObjects {
    let mut objects: Vec<Arc<dyn Hittable + Send + Sync>> = vec![];

    let ground_material = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Checker::new(
            Arc::new(SolidColor::new(0.2, 0.3, 0.1)),
            Arc::new(SolidColor::new(0.9, 0.9, 0.9)),
        )),
    }));
    objects.push(Arc::new(Sphere::new(
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
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random(0., 1.) * Vec3::random(0., 1.);
                    let sphere_material = Arc::new(Material::Labmertian(MatLabmertian {
                        albedo: Arc::new(SolidColor::from(albedo)),
                    }));
                    let center2 = center + Vec3::new(0., rng.gen_range(0.0..0.5), 0.);
                    objects.push(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        start_time,
                        end_time,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random(0.5, 1.);
                    let roughness = rng.gen_range(0.0..0.5);
                    let sphere_material =
                        Arc::new(Material::Metalic(MatMetalic { albedo, roughness }));
                    objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Material::Dielectric(MatDielectric {
                        refraction_index: 1.5,
                    }));
                    objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                };
            }
        }
    }

    let material = Arc::new(Material::Dielectric(MatDielectric {
        refraction_index: 1.5,
    }));
    objects.push(Arc::new(Sphere::new(Vec3::new(0., 1., 0.), 1.0, material)));

    let material = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(SolidColor::new(0.4, 0.2, 0.1)),
    }));
    objects.push(Arc::new(Sphere::new(Vec3::new(-4., 1., 0.), 1.0, material)));

    let material = Arc::new(Material::Metalic(MatMetalic {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        roughness: 0.0,
    }));
    objects.push(Arc::new(Sphere::new(Vec3::new(4., 1., 0.), 1.0, material)));
    // BvhNode::new(&objects, start_time, end_time).unwrap()
    WorldObjects::new(objects)
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
