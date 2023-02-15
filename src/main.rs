use rust_ray_tracer::ppm::{color::Color, image::PpmImage};

fn main() {
    let width = 256;
    let height = 256;
    let mut ppm = PpmImage::new(width, height);

    // for (int j = image_height-1; j >= 0; --j) {
    //     for (int i = 0; i < image_width; ++i) {
    //         auto r = double(i) / (image_width-1);
    //         auto g = double(j) / (image_height-1);
    //         auto b = 0.25;

    //         int ir = static_cast<int>(255.999 * r);
    //         int ig = static_cast<int>(255.999 * g);
    //         int ib = static_cast<int>(255.999 * b);

    for j in 0..height {
        for i in 0..width {
            let r = i as f32 / (width - 1) as f32;
            let g = j as f32 / (height - 1) as f32;
            let b = 0.25;

            ppm.pixels[j * height + i] = Color::from_unit_range(r, g, b).unwrap();
        }
    }
    let mut path = std::env::current_dir().unwrap();
    path.push("example.ppm");
    ppm.save(path).unwrap();
}
