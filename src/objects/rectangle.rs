use crate::{
    colors::color::Color,
    constants::DEFAULT_FOREGROUND,
    linal::vertx2::VX2,
    renderer::two_d::{Render, Renderer},
    vx2,
};

use super::{line::bresenham, tri::Tri2d, Collision, Vertices};

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

    pub fn vertices_arr(&self) -> [VX2; 4] {
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
        let [tl, tr, br, bl] = self.vertices_arr();
        bresenham(renderer.buffer_mut(), &tl, &tr, self.stroke.into());
        bresenham(renderer.buffer_mut(), &tr, &br, self.stroke.into());
        bresenham(renderer.buffer_mut(), &br, &bl, self.stroke.into());
        bresenham(renderer.buffer_mut(), &bl, &tl, self.stroke.into());
    }

    fn fill(&self, renderer: &mut Renderer) {
        let [tl, tr, br, bl] = self.vertices_arr();
        Tri2d::new(tl, tr, bl).fill(renderer);
        Tri2d::new(bl, tr, br).fill(renderer);
    }
    fn draw_clr<C: Into<u32> + Copy>(&self, renderer: &mut Renderer, c: C) {
        
        let [tl, tr, br, bl] = self.vertices_arr();
        bresenham(renderer.buffer_mut(), &tl, &tr, c.into());
        bresenham(renderer.buffer_mut(), &tr, &br, c.into());
        bresenham(renderer.buffer_mut(), &br, &bl, c.into());
        bresenham(renderer.buffer_mut(), &bl, &tl, c.into());
    }


}


impl Vertices for Rectangle {
    fn vertices(&self) -> Vec<VX2> {
        self.vertices_arr().to_vec()
    }
}
impl Vertices for &Rectangle {
    fn vertices(&self) -> Vec<VX2> {
        self.vertices_arr().to_vec()
    }
}

impl Collision for Rectangle {}
