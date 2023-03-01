use std::sync::Arc;

use rand::{rngs::StdRng, Rng, SeedableRng};
use rust_ray_tracer::{
    math::vec3::Vec3,
    raytracing::{
        camera::Camera,
        material::{MatDielectric, MatDiffuseLight, MatLabmertian, MatMetalic, Material},
        objects::{HittableObject, MovingSphere, PlaneX, PlaneY, PlaneZ, Sphere, WorldObjects},
        renderer::Renderer,
        texture::{CheckerTexture, ImageTexture, SolidColorTexture, Texture},
    },
};

use crate::GlobalSettings;

pub fn earth_scene(settings: &GlobalSettings) -> Renderer {
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
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        settings.animation_start_time,
        settings.animation_end_time,
    );

    let mut path = std::env::current_dir().unwrap();
    path.push("images");
    path.push("earthmap.jpg");
    let earth_texture = Arc::new(Texture::Image(ImageTexture::new(path).unwrap()));
    let earth_surface = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: earth_texture,
    }));
    let globe = Arc::new(Sphere::new(Vec3::new(0., 0., 0.), 2.0, earth_surface));
    let world = WorldObjects::new(vec![globe]);

    let mut render = Renderer::init(
        camera,
        settings.samples_per_pixel,
        settings.max_ray_bounces,
        Box::new(world),
    );
    render.background = Vec3::new(0.70, 0.80, 1.00);
    render
}

pub fn test_scene(settings: &GlobalSettings) -> Renderer {
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
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        settings.animation_start_time,
        settings.animation_end_time,
    );

    // --------World---------
    //Materials
    let material_ground = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(0.8, 0.8, 0.0))),
    }));
    let material_center = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(0.7, 0.3, 0.3))),
    }));
    let material_left = Arc::new(Material::Dielectric(MatDielectric {
        refraction_index: 1.7,
    }));
    let material_right = Arc::new(Material::Metalic(MatMetalic::new(
        Vec3::new(0.8, 0.6, 0.2),
        1.0,
    )));

    // Objects
    let objects: Vec<Arc<dyn HittableObject + Send + Sync>> = vec![
        Arc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, material_center)),
        Arc::new(Sphere::new(
            Vec3::new(0., -100.5, -1.),
            100.,
            material_ground,
        )),
        Arc::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, material_left)),
        Arc::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, material_right)),
    ];
    let world = WorldObjects::new(objects);
    // ---------
    let mut render = Renderer::init(
        camera,
        settings.samples_per_pixel,
        settings.max_ray_bounces,
        Box::new(world),
    );
    render.background = Vec3::new(0.70, 0.80, 1.00);
    render
}

pub fn random_scene(settings: &GlobalSettings) -> Renderer {
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
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        settings.animation_start_time,
        settings.animation_end_time,
    );

    // ------ World ------------
    let mut objects: Vec<Arc<dyn HittableObject + Send + Sync>> = vec![];

    let ground_material = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::Checker(CheckerTexture::new(
            Arc::new(SolidColorTexture::new(0.2, 0.3, 0.1)),
            Arc::new(SolidColorTexture::new(0.9, 0.9, 0.9)),
        ))),
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
                        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::from(albedo))),
                    }));
                    let center2 = center + Vec3::new(0., rng.gen_range(0.0..0.5), 0.);
                    objects.push(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        settings.animation_start_time,
                        settings.animation_end_time,
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
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(0.4, 0.2, 0.1))),
    }));
    objects.push(Arc::new(Sphere::new(Vec3::new(-4., 1., 0.), 1.0, material)));

    let material = Arc::new(Material::Metalic(MatMetalic {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        roughness: 0.0,
    }));
    objects.push(Arc::new(Sphere::new(Vec3::new(4., 1., 0.), 1.0, material)));
    let world = WorldObjects::new(objects);
    // ------------------------

    let mut render = Renderer::init(
        camera,
        settings.samples_per_pixel,
        settings.max_ray_bounces,
        Box::new(world),
    );

    render.background = Vec3::new(0.70, 0.80, 1.00);
    render
}

pub fn lighting_scene(settings: &GlobalSettings) -> Renderer {
    // Camera
    let lookfrom = Vec3::new(26., 3., 6.);
    let lookat = Vec3::new(0., 2., 0.);
    let rotation = Vec3::new(0., 1., 0.);
    let vfov = 20.0;
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        rotation,
        vfov,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        settings.animation_start_time,
        settings.animation_end_time,
    );

    // --------World---------
    //Materials
    let material_ground = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(1., 1., 1.))),
    }));
    let material_center = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(0.5, 0.2, 0.7))),
    }));
    let light = Arc::new(Material::DiffuseLight(MatDiffuseLight {
        emit: Arc::new(Texture::SolidColor(SolidColorTexture::new(1., 1., 1.))),
    }));

    // Objects
    let objects: Vec<Arc<dyn HittableObject + Send + Sync>> = vec![
        Arc::new(Sphere::new(
            Vec3::new(0., -1000.0, 0.),
            1000.0,
            material_ground,
        )),
        Arc::new(Sphere::new(Vec3::new(0., 2., 0.), 2.0, material_center)),
        Arc::new(PlaneZ::new(0., 1., -3., 4., 4., light.clone())),
        Arc::new(PlaneZ::new(0., 1., 3., 4., 4., light)),
    ];
    let world = WorldObjects::new(objects);
    // ---------
    Renderer::init(
        camera,
        settings.samples_per_pixel,
        settings.max_ray_bounces,
        Box::new(world),
    )
}

pub fn cornell_box(settings: &mut GlobalSettings) -> Renderer {
    settings.width = 600;
    settings.aspect_ratio = 1.0;
    settings.height = 600;
    settings.samples_per_pixel = 200;

    // Camera
    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let rotation = Vec3::new(0., 1., 0.);
    let vfov = 40.0;
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        rotation,
        vfov,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        settings.animation_start_time,
        settings.animation_end_time,
    );

    // --------World---------
    //Materials
    let red = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(
            0.65, 0.05, 0.05,
        ))),
    }));
    let green = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(
            0.12, 0.45, 0.15,
        ))),
    }));
    let white = Arc::new(Material::Labmertian(MatLabmertian {
        albedo: Arc::new(Texture::SolidColor(SolidColorTexture::new(
            0.73, 0.73, 0.73,
        ))),
    }));

    let light = Arc::new(Material::DiffuseLight(MatDiffuseLight {
        emit: Arc::new(Texture::SolidColor(SolidColorTexture::new(15., 15., 15.))),
    }));

    // Objects
    let objects: Vec<Arc<dyn HittableObject + Send + Sync>> = vec![
        Arc::new(PlaneX::new(555., 0., 0., 555., 555., green)),
        Arc::new(PlaneX::new(0., 0., 0., 555., 555., red)),
        Arc::new(PlaneY::new(213., 554., 227., 130., 105., light)),
        Arc::new(PlaneY::new(0., 0., 0., 555., 555., white.clone())),
        Arc::new(PlaneY::new(0., 555., 0., 555., 555., white.clone())),
        Arc::new(PlaneZ::new(0., 0., 555., 555., 555., white)),
    ];

    let world = WorldObjects::new(objects);
    // ---------
    Renderer::init(
        camera,
        settings.samples_per_pixel,
        settings.max_ray_bounces,
        Box::new(world),
    )
}
