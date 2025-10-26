#[derive(Debug, PartialEq)]
pub enum ColorU32 {
    Rgba(u32),
    Argb(u32),
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }
    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }

    pub fn set_alpha(&mut self, a: u8) {
        self.a = a;
    }
    fn unpack(&self) -> (u32, u32, u32, u32) {
        (self.r as u32, self.g as u32, self.b as u32, self.a as u32)
    }
    pub fn from_rgba(c: u32) -> Self {
        let v = c.to_le();
        Self {
            r: (v >> (8 * 3) & 0xFF) as u8,
            g: (v >> (8 * 2) & 0xFF) as u8,
            b: (v >> (8 * 1) & 0xFF) as u8,
            a: (v >> (8 * 0) & 0xFF) as u8,
        }
    }

    pub fn from_argb(c: u32) -> Self {
        let v = c.to_le();
        Self {
            a: (v >> (8 * 3) & 0xFF) as u8,
            r: (v >> (8 * 2) & 0xFF) as u8,
            g: (v >> (8 * 1) & 0xFF) as u8,
            b: (v >> (8 * 0) & 0xFF) as u8,
        }
    }

    pub fn saturation(&mut self, factor: f32) {
        self.r = (self.r as f32 * factor).clamp(0.0, 255.0) as u8;
        self.g = (self.g as f32 * factor).clamp(0.0, 255.0) as u8;
        self.b = (self.b as f32 * factor).clamp(0.0, 255.0) as u8;
    }

    pub fn shade(&self, lum: f32) -> Self {
        Self {
            r: (self.r as f32 * (1.0 - lum)) as u8,
            g: (self.g as f32 * (1.0 - lum)) as u8,
            b: (self.b as f32 * (1.0 - lum)) as u8,
            a: self.a,
        }
    }
    pub fn lerp(start: &Self, end: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (start.r as f32 * (1.0 - t) + end.r as f32 * t).round().clamp(0.0, 255.0) as u8,
            g: (start.g as f32 * (1.0 - t) + end.g as f32 * t).round().clamp(0.0, 255.0) as u8,
            b: (start.b as f32 * (1.0 - t) + end.b as f32 * t).round().clamp(0.0, 255.0) as u8,
            a: (start.a as f32 * (1.0 - t) + end.a as f32 * t).round().clamp(0.0, 255.0) as u8,
        }
    }
}

///always x11 order assumed
impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::from_argb(value)
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Self::new(value[0], value[1], value[2], value[3])
    }
}

impl From<ColorU32> for Color {
    fn from(value: ColorU32) -> Self {
        match value {
            ColorU32::Rgba(v) => Self::from_rgba(v),
            ColorU32::Argb(v) => Self::from_argb(v),
        }
    }
}

///always into x11 order
impl Into<u32> for Color {
    fn into(self) -> u32 {
        let (r, g, b, a) = self.unpack();
        b | g << 8 | r << 16 | a << 24
    }
}
