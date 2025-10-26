
use crate::{buffer::Buffer, colors::color::Color, linal::vertx2::VX2, vx2};

#[derive(Clone)]
pub struct Texture {
    buffer: Buffer<u32>,
    size: VX2,
}

impl Texture {
    pub fn init(size: VX2) -> Self {
        Self {
            buffer: Buffer::init(size.x as i32, size.y as i32),
            size,
        }
    }

    pub fn init_with_background_color(size: VX2, color: Color) -> Self {
        let buffer = Buffer::init_with_value(size.x as i32, size.y as i32, color.into());
        Self { buffer, size }
    }

    pub fn size<'t>(&'t self) -> &'t VX2 {
        &self.size
    }

    pub fn get_buffer_mut<'t>(&'t mut self) -> &'t mut Buffer<u32> {
        &mut self.buffer
    }
    pub fn get_buffer<'t>(&'t self) -> &'t Buffer<u32> {
        &self.buffer
    }

    pub fn clear(&mut self, c: Color) {
        self.buffer.fill(c.into());
    }
}
