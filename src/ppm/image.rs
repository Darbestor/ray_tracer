use super::color::{Color, RGBColor};

/// PPM image format.
/// Stores RGB colors with range from 0 to 255
pub struct PpmImage {
    rows: usize,
    columns: usize,
    pub pixels: Vec<Color>,
}

impl PpmImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            rows: height,
            columns: width,
            pixels: vec![RGBColor::White.into(); width * height],
        }
    }

    /// Reset all pixels to white color
    pub fn clear(&mut self) {
        self.pixels
            .resize(self.columns * self.rows, RGBColor::White.into());
    }
}
