pub mod example_scenes;

use rust_ray_tracer::{
    math::vec3::Vec3,
    ppm::{color::Color, image::PpmImage},
};

pub struct GlobalSettings {
    pub aspect_ratio: f32,
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub max_ray_bounces: usize,
    pub animation_start_time: f32,
    pub animation_end_time: f32,
}

fn main() {
    // Constants
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let settings = GlobalSettings {
        aspect_ratio,
        width,
        height: (width as f32 / aspect_ratio) as usize,
        samples_per_pixel: 100,
        max_ray_bounces: 50,
        animation_start_time: 0.0,
        animation_end_time: 1.0,
    };

    let renderer = match std::env::args().nth(1).unwrap().as_str() {
        "1" => example_scenes::test_scene(&settings),
        "2" => example_scenes::random_scene(&settings),
        "3" => example_scenes::earth_scene(&settings),
        "4" => example_scenes::lighting_scene(&settings),
        _ => panic!("Unknown scene number"),
    };

    let scene = renderer.render(settings.width, settings.height, true);

    save_to_ppm("lighting.ppm", settings.width, settings.height, scene);
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
