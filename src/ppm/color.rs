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
}
