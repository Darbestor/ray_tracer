use std::io::Write;
use std::{fs::File, io, path::Path};

use super::color::{Color, RGBColor};

/// PPM image format.
/// Stores RGB colors with range from 0 to 255
pub struct PpmImage {
    height: usize,
    width: usize,
    pub pixels: Vec<Color>,
}

impl PpmImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            pixels: vec![RGBColor::White.into(); width * height],
        }
    }

    /// Reset all pixels to white color
    pub fn clear(&mut self) {
        self.pixels
            .resize(self.width * self.height, RGBColor::White.into());
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> io::Result<()> {
        let mut file = File::create(path)?;
        self.write_header(&mut file)?;
        self.pixels.iter().try_for_each(|c| {
            writeln!(
                &mut file,
                "{} {} {}",
                c.get_red(),
                c.get_green(),
                c.get_blue()
            )
        })?;
        Ok(())
    }

    fn write_header(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "P3\n{} {}\n{}", self.width, self.height, 255)?;
        Ok(())
    }
}
