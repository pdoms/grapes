use crate::{
    colors::color::Color,
    constants::DEFAULT_FOREGROUND,
    linal::vx2::{self, VX2},
    renderer::two_d::{Render, Renderer},
    vx2,
};

use super::{line::bresenham, tri::Tri2d};

pub struct Rectangle {
    /// center
    pub pos: VX2,
    pub size: VX2,
    pub rotation: f32,
    fill: Color,
    stroke: Color,
}
impl Rectangle {
    pub fn new(pos: VX2, size: VX2) -> Self {
        Self {
            pos,
            size,
            fill: DEFAULT_FOREGROUND.into(),
            stroke: DEFAULT_FOREGROUND.into(),
            rotation: 0.0,
        }
    }

    pub fn set_fill<C: Into<Color> + Copy>(&mut self, fill: C) {
        self.fill = fill.into()
    }
    pub fn set_stroke<C: Into<Color> + Copy>(&mut self, stroke: C) {
        self.stroke = stroke.into()
    }

    pub fn set_rotation(&mut self, deg: f32) {
        self.rotation = deg;
    }

    pub fn vertices(&self) -> [VX2; 4] {
        let width_offset = self.size.x * 0.5;
        let height_offset = self.size.y * 0.5;
        let tl = vx2!(-width_offset, height_offset);
        let tr = vx2!(width_offset, height_offset);
        let br = vx2!(width_offset, -height_offset);
        let bl = vx2!(-width_offset, -height_offset);
        let mut verts = [tl, tr, br, bl];
        let rads = self.rotation.to_radians();
        let cos_theta = rads.cos();
        let sin_theta = rads.sin();
        for vert in verts.iter_mut() {
            let temp_x = vert.x * cos_theta - vert.y * sin_theta + self.pos.x;
            let temp_y = vert.x * sin_theta + vert.y * cos_theta + self.pos.y;
            vert.x = temp_x;
            vert.y = temp_y;
        }
        verts
    }
}
impl Render for Rectangle {
    fn draw(&self, renderer: &mut Renderer) {
        let [tl, tr, br, bl] = self.vertices();
        bresenham(renderer.buffer_mut(), &tl, &tr, self.stroke.into());
        bresenham(renderer.buffer_mut(), &tr, &br, self.stroke.into());
        bresenham(renderer.buffer_mut(), &br, &bl, self.stroke.into());
        bresenham(renderer.buffer_mut(), &bl, &tl, self.stroke.into());
    }

    fn fill(&self, renderer: &mut Renderer) {
        let [tl, tr, br, bl] = self.vertices();
        Tri2d::new(tl, tr, bl).fill(renderer);
        Tri2d::new(bl, tr, br).fill(renderer);
        
    }
}
