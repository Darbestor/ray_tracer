use thiserror::Error;

use crate::math::vec3::Vec3;

#[derive(Error, Debug)]
#[error("Value must be between 0 and 1.0: {0}")]
pub struct OutOfBoundsError(String);

/// RGB color
#[derive(Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Returns red channel
    pub fn get_red(&self) -> u8 {
        self.red
    }

    /// Returns green channel
    pub fn get_green(&self) -> u8 {
        self.green
    }

    /// Returns blue channel
    pub fn get_blue(&self) -> u8 {
        self.blue
    }
}

impl TryFrom<Vec3> for Color {
    type Error = OutOfBoundsError;
    fn try_from(value: Vec3) -> Result<Self, Self::Error> {
        if !value.x().is_sign_positive()
            || !value.y().is_sign_positive()
            || !value.z().is_sign_positive()
            || value.x() > 1.0
            || value.y() > 1.0
            || value.z() > 1.0
        {
            return Err(OutOfBoundsError(format!("{:?}", value)));
        }

        let ir = (255.999 * value.x()) as u8;
        let ig = (255.999 * value.y()) as u8;
        let ib = (255.999 * value.z()) as u8;
        Ok(Self {
            red: ir,
            green: ig,
            blue: ib,
        })
    }
}

pub enum RGBColor {
    White,
    Black,
}

impl From<RGBColor> for Color {
    fn from(value: RGBColor) -> Self {
        match value {
            RGBColor::White => Color {
                red: 255,
                green: 255,
                blue: 255,
            },
            RGBColor::Black => Color {
                red: 0,
                green: 0,
                blue: 0,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::math::vec3::Vec3;

    use super::Color;

    #[test]
    fn test() {
        let color = Color::new(0, 0, 0);
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);

        let color = Color::new(1, 2, 3);
        assert_eq!(color.red, 1);
        assert_eq!(color.green, 2);
        assert_eq!(color.blue, 3);

        let color = Color::new(255, 255, 255);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 255);
    }

    #[test]
    fn from_unit_range_test() {
        let color = Color::try_from(Vec3::new(-1., 0., 0.));
        assert!(color.is_err());
        let color = Color::try_from(Vec3::new(0., -1., 0.));
        assert!(color.is_err());
        let color = Color::try_from(Vec3::new(0., 0., -1.));
        assert!(color.is_err());

        let color = Color::try_from(Vec3::new(1. + f32::EPSILON, 0., 0.));
        assert!(color.is_err());
        let color = Color::try_from(Vec3::new(0., 1. + f32::EPSILON, 0.));
        assert!(color.is_err());
        let color = Color::try_from(Vec3::new(0., 0., 1. + f32::EPSILON));
        assert!(color.is_err());

        let color = Color::try_from(Vec3::new(0., 0., 0.)).unwrap();
        assert_eq!(color.get_red(), 0);
        assert_eq!(color.get_green(), 0);
        assert_eq!(color.get_blue(), 0);

        let color = Color::try_from(Vec3::new(0., 1., 0.5)).unwrap();
        assert_eq!(color.get_red(), 0);
        assert_eq!(color.get_green(), 255);
        assert_eq!(color.get_blue(), 127);
    }
}
