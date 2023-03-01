use std::{sync::Arc, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rust_ray_tracer::{
    math::vec3::Vec3,
    raytracing::{
        camera::Camera,
        material::{MatDielectric, MatLabmertian, MatMetalic, Material},
        objects::{HittableObject, Sphere, WorldObjects},
        renderer::Renderer,
        texture::{SolidColorTexture, Texture},
    },
};

static ASPECT_RATIO: f32 = 16. / 9.;
static WIDTH: usize = 300;
static HEIGHT: usize = (WIDTH as f32 / ASPECT_RATIO) as usize;
static SAMPLES_PER_PIXEL: usize = 100;
static MAX_RAY_BOUNCES: usize = 50;

fn scene() -> WorldObjects {
    let mut objects: Vec<Arc<dyn HittableObject + Send + Sync>> = Vec::new();

    let ground_material = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(0.5, 0.5, 0.5))),
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
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random(0., 1.) * Vec3::random(0., 1.);
                    Arc::new(Material::Labmertian(MatLabmertian {
                        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::from(albedo))),
                    }))
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
                objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material = Arc::new(Material::Dielectric(MatDielectric {
        refraction_index: 1.5,
    }));
    objects.push(Arc::new(Sphere::new(Vec3::new(0., 1., 0.), 1.0, material)));

    let material = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::from(Vec3::new(
            0.4, 0.2, 0.1,
        )))),
    }));
    objects.push(Arc::new(Sphere::new(Vec3::new(-4., 1., 0.), 1.0, material)));

    let material = Arc::new(Material::Metalic(MatMetalic {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        roughness: 0.0,
    }));
    objects.push(Arc::new(Sphere::new(Vec3::new(4., 1., 0.), 1.0, material)));
    WorldObjects::new(objects)
}

fn setup_render() -> Renderer {
    // Camera
    let lookfrom = Vec3::new(5., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let rotation = Vec3::new(0., 1., 0.);
    let vfov = 90.0;
    let dist_to_focus = 5.;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        rotation,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Renderer::init(
        camera,
        SAMPLES_PER_PIXEL,
        MAX_RAY_BOUNCES,
        Box::new(scene()),
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut renderer = setup_render();

    let mut group = c.benchmark_group("raytracing");
    group.sample_size(30);
    group.warm_up_time(Duration::from_secs(10));

    group.bench_function("parallel raytracing", |b| {
        b.iter_batched(
            scene,
            |scene| {
                renderer.objects = Box::new(scene);
                renderer.render(WIDTH, HEIGHT, false)
            },
            criterion::BatchSize::PerIteration,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
