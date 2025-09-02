use std::u32;

use crate::{buffer::Buffer, colors::color::Color, constants::DEFAULT_FOREGROUND};

pub trait Render {
    fn draw(&self, _renderer: &mut Renderer) {
        unimplemented!("Default unimplemented for draw");
    }
    fn fill(&self, _renderer: &mut Renderer) {
        unimplemented!("Default unimplemented for fill");
    }
    fn draw_renderer(&self, _renderer: &mut Renderer) {
        unimplemented!("Default unimplemented for draw_renderer");
    }
    fn fill_rendderer(&self, _renderer: &mut Renderer) {
        unimplemented!("Default unimplemented for fill_renderer");
    }
    fn draw_clr<C: Into<Color> + Copy>(&self, _renderer: &mut Renderer, _c: C) {
        unimplemented!("Default unimplemented for draw_clr");
    }
    fn fill_clr<C: Into<Color> + Copy>(&self, _renderer: &mut Renderer, _c: C) {
        unimplemented!("Default unimplemented for fill_clr");
    }
}

pub struct Renderer {
    buffer: Buffer<u32>,
    fill: Color,
    stroke: Color,
    anti_aliasing: bool,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: Buffer::init(width, height),
            fill: DEFAULT_FOREGROUND.into(),
            stroke: DEFAULT_FOREGROUND.into(),
            anti_aliasing: false,
        }
    }
    pub fn window_width(&self) -> usize {
        self.buffer.width()
    }
    pub fn window_height(&self) -> usize {
        self.buffer.height()
    }

    pub fn set_fill<C: Into<Color> + Copy>(&mut self, fill: C) {
        self.fill = fill.into();
    }
    pub fn fill(&self) -> Color {
        self.fill
    }
    pub fn set_stroke<C: Into<Color> + Copy>(&mut self, stroke: C) {
        self.stroke = stroke.into()
    }
    pub fn stroke(&self) -> Color {
        self.stroke
    }

    pub fn buffer<'r>(&'r self) -> &'r Buffer<u32> {
        &self.buffer
    }

    pub fn buffer_mut<'r>(&'r mut self) -> &'r mut Buffer<u32> {
        &mut self.buffer
    }

    pub fn clear_background<C: Into<u32> + Copy>(&mut self, c: C) {
        for i in 0..self.buffer.len() {
            self.buffer.set(i, c.into());
        }
    }
}
