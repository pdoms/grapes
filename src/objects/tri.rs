use crate::{
    colors::color::Color,
    constants::DEFAULT_FOREGROUND,
    linal::vx2::VX2,
    max, min,
    objects::line::bresenham,
    renderer::two_d::{Render, Renderer},
    vx2,
};

use super::utils::{BBox2d, Bounds};

pub struct Tri2d {
    pub p0: VX2,
    pub p1: VX2,
    pub p2: VX2,
    stroke: Color,
    fill: Color,
}

impl Tri2d {
    pub fn new(p0: VX2, p1: VX2, p2: VX2) -> Self {
        Tri2d {
            p0,
            p1,
            p2,
            stroke: DEFAULT_FOREGROUND.into(),
            fill: DEFAULT_FOREGROUND.into(),
        }
    }

    pub fn set_stroke<C: Into<Color> + Copy>(&mut self, c: C) {
        self.stroke = c.into();
    }
    pub fn translate(&mut self, offset: &VX2) {
        self.p0 += offset;
        self.p1 += offset;
        self.p2 += offset;
    }

    pub fn centroid(&self) -> VX2 {
        vx2!(
            (self.p0.x + self.p1.x + self.p2.x) / 3.0,
            (self.p0.y + self.p1.y + self.p2.y) / 3.0
        )
    }
    pub fn rotate(&self, rot_deg: f32, privot: &VX2) -> Self {
        let mut new_p0 = &self.p0 - &privot;
        let mut new_p1 = &self.p1 - &privot;
        let mut new_p2 = &self.p2 - &privot;
        new_p0 = new_p0.rotation(rot_deg);
        new_p1 = new_p1.rotation(rot_deg);
        new_p2 = new_p2.rotation(rot_deg);
        Self {
            p0: new_p0 + privot,
            p1: new_p1 + privot,
            p2: new_p2 + privot,
            stroke: self.stroke,
            fill: self.fill,
        }
    }
}

impl BBox2d for Tri2d {
    fn bbox(&self) -> Bounds {
        Bounds {
            min_x: min!(min!(self.p0.x, self.p1.x), self.p2.x),
            max_x: max!(max!(self.p0.x, self.p1.x), self.p2.x),
            min_y: min!(min!(self.p0.y, self.p1.y), self.p2.y),
            max_y: max!(max!(self.p0.y, self.p1.y), self.p2.y),
        }
    }
}

impl Render for Tri2d {
    fn draw(&self, renderer: &mut Renderer) {
        bresenham(
            &mut renderer.buffer_mut(),
            &self.p0,
            &self.p1,
            self.stroke.into(),
        );
        bresenham(
            &mut renderer.buffer_mut(),
            &self.p0,
            &self.p2,
            self.stroke.into(),
        );
        bresenham(
            &mut renderer.buffer_mut(),
            &self.p1,
            &self.p2,
            self.stroke.into(),
        );
    }
    fn fill(&self, renderer: &mut Renderer) {
        unimplemented!("Default unimplemented for fill");
    }
    fn draw_renderer(&self, renderer: &mut Renderer) {
        unimplemented!("Default unimplemented for draw_renderer");
    }
    fn fill_rendderer(&self, renderer: &mut Renderer) {
        unimplemented!("Default unimplemented for fill_renderer");
    }
    fn draw_clr<C: Into<Color> + Copy>(&self, renderer: &mut Renderer, c: C) {
        unimplemented!("Default unimplemented for draw_clr");
    }
    fn fill_clr<C: Into<Color> + Copy>(&self, renderer: &mut Renderer, c: C) {
        unimplemented!("Default unimplemented for fill_clr");
    }
}
