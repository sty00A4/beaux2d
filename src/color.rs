use sdl2::pixels;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
impl Into<pixels::Color> for Color {
    fn into(self) -> pixels::Color {
        pixels::Color { r: self.r, g: self.g, b: self.b, a: 0 }
    }
}
impl From<pixels::Color> for Color {
    fn from(value: pixels::Color) -> Self {
        Self { r: value.r, g: value.g, b: value.b }
    }
}
impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self { r: value.0, g: value.1, b: value.2 }
    }
}