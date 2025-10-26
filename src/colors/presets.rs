use super::color::Color;


/// u32 as [r, g, b, a];
#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum GrapesColors {
    Black = 0x000000FF,
    White = 0xFFFFFFFF,
    GrapesBlack = 0x222222FF,
    GrapesWhite = 0xFFFFEEFF,
    LightGray = 0x6D6D6DFF,
    Gray = 0x999999FF,
    DarkGray = 0x5E5E5EFF,
    Red = 0xFF0000FF,
    GrapesRed = 0x8c0707FF,
    Green = 0x00FF00FF,
    GrapesGreen = 0x319939FF,
    DBlue = 0x0000FFFF,
    GrapesBlue = 0x272FA3FF,
    Purple = 0x5F1D99FF,
    Teal = 0x008080FF,
    Maroon = 0x550000FF,
    Orange = 0xCF8608FF,
    Yellow = 0xD4C222FF,
    Transparent = 0,
}


impl From<GrapesColors> for Color {
    fn from(value: GrapesColors) -> Self {
        Self::from_rgba(value as u32)
    }
}

impl Into<u32> for GrapesColors {
    fn into(self) -> u32 {
        let clr = Color::from(self);
        clr.into()
    }
}
