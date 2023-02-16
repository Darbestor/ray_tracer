use rust_ray_tracer::{
    math::vec3::Vec3,
    ppm::{color::Color, image::PpmImage},
    raytracing::{camera::Camera, ray::Ray, ray_hit::RayHitTester, sphere::Sphere},
};

fn ray_color(ray: &Ray) -> Color {
    static TEST_SPHERE: Sphere = Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
    };

    if let Some(hit) = TEST_SPHERE.hit(ray) {
        let normal = (ray.at(hit) - Vec3::new(0., 0., -1.)).unit();
        let normal_color_vec = 0.5 * Vec3::new(normal.x() + 1., normal.y() + 1., normal.z() + 1.);
        return Color::from_unit_range(
            normal_color_vec.x(),
            normal_color_vec.y(),
            normal_color_vec.z(),
        )
        .unwrap();
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // Blend between white and blue
    let blender_color = (1.0 - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0);
    Color::from_unit_range(blender_color.x(), blender_color.y(), blender_color.z()).unwrap()
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio) as usize;
    let mut ppm = PpmImage::new(width, height);
    let camera = Camera::new(2.0, aspect_ratio * 2.0);

    for j in 0..height {
        for i in 0..width {
            let u = i as f32 / (width - 1) as f32;
            let v = (height - j) as f32 / (height - 1) as f32;

            let direction =
                camera.lower_left_corner() + &(u * camera.horizontal()) + v * camera.vertical()
                    - camera.origin;
            let ray = Ray::new(&camera.origin, &direction);
            ppm.pixels[j * width + i] = ray_color(&ray);
        }
    }
    let mut path = std::env::current_dir().unwrap();
    path.push("example.ppm");
    ppm.save(path).unwrap();
}
