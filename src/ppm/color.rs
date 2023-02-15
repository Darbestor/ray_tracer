use thiserror::Error;

#[derive(Error, Debug)]
#[error("Value must be between 0 and 1.0: {0:?}")]
pub struct OutOfBoundsError([f32; 3]);

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

    pub fn from_unit_range(red: f32, green: f32, blue: f32) -> Result<Self, OutOfBoundsError> {
        if !red.is_sign_positive()
            || !green.is_sign_positive()
            || !blue.is_sign_positive()
            || red > 1.0
            || green > 1.0
            || blue > 1.0
        {
            return Err(OutOfBoundsError([red, green, blue]));
        }

        let ir = (255.999 * red) as u8;
        let ig = (255.999 * green) as u8;
        let ib = (255.999 * blue) as u8;
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
        let color = Color::from_unit_range(-1., 0., 0.);
        assert!(color.is_err());
        let color = Color::from_unit_range(0., -1., 0.);
        assert!(color.is_err());
        let color = Color::from_unit_range(0., 0., -1.);
        assert!(color.is_err());

        let color = Color::from_unit_range(1. + f32::EPSILON, 0., 0.);
        assert!(color.is_err());
        let color = Color::from_unit_range(0., 1. + f32::EPSILON, 0.);
        assert!(color.is_err());
        let color = Color::from_unit_range(0., 0., 1. + f32::EPSILON);
        assert!(color.is_err());

        let color = Color::from_unit_range(0., 0., 0.).unwrap();
        assert_eq!(color.get_red(), 0);
        assert_eq!(color.get_green(), 0);
        assert_eq!(color.get_blue(), 0);

        let color = Color::from_unit_range(0., 1., 0.5).unwrap();
        assert_eq!(color.get_red(), 0);
        assert_eq!(color.get_green(), 255);
        assert_eq!(color.get_blue(), 127);
    }
}
