use crate::{
    buffer::Buffer,
    colors::color::Color,
    linal::vx2::VX2,
    renderer::two_d::{Render, Renderer},
    vx2,
};

use super::utils::top_left_line;

#[derive(Clone, Debug)]
pub struct Line2d {
    pub p1: VX2,
    pub p2: VX2,
    pub clr: Option<Color>,
}

impl Line2d {
    pub fn new(p1: VX2, p2: VX2) -> Self {
        Self { p1, p2, clr: None }
    }

    pub fn from_ccords(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            p1: vx2!(x1, y1),
            p2: vx2!(x2, y2),
            clr: None,
        }
    }

    pub fn set_clr<C: Into<Color> + Copy>(&mut self, c: C) {
        self.clr = Some(c.into())
    }

    pub fn top_left_sort(&mut self) {
        top_left_line(&mut self.p1, &mut self.p2);
    }

    pub fn midpoint(&self) -> VX2 {
        vx2!((self.p1.x + self.p2.x) * 0.5, (self.p1.y + self.p2.y) * 0.5)
    }

    pub fn rotate(&self, rot_deg: f32, privot: &VX2) -> Self {
        let mut new_p1 = &self.p1 - &privot;
        let mut new_p2 = &self.p2 - &privot;
        new_p1 = new_p1.rotation(rot_deg);
        new_p2 = new_p2.rotation(rot_deg);
        Self {
            p1: new_p1 + privot,
            p2: new_p2 + privot,
            clr: self.clr,
        }
    }
}

impl Render for Line2d {
    fn draw(&self, renderer: &mut Renderer) {
        if let Some(clr) = self.clr {
            bresenham(renderer.buffer_mut(), &self.p1, &self.p2, clr.into());
        }
    }
    fn draw_clr<C: Into<u32> + Copy>(&self, renderer: &mut Renderer, c: C) {
        bresenham(renderer.buffer_mut(), &self.p1, &self.p2, c.into());
    }

    fn draw_renderer(&self, renderer: &mut Renderer) {
        let clr: u32 = renderer.stroke().into();
        bresenham(renderer.buffer_mut(), &self.p1, &self.p2, clr);
    }
}

//pub fn bresenham(buffer: &mut Buffer<u32>, p1: &VX2, p2: &VX2, clr: u32) {
//    let dx = (p2.x - p1.x).abs() as i32;
//    let dy = (p2.y - p1.y).abs() as i32;
//
//    //decision parameter
//    let mut p = 2 * dy - dx;
//    let dbl_dy = 2 * dy;
//    let dbl_dy_dx = 2 * (dy - dx);
//    let mut x = p1.x as i32;
//    let mut y = p1.y as i32;
//    let x_inc = if p2.x > p1.x { 1 } else { -1 };
//    let y_inc = if p2.y > p1.y { 1 } else { -1 };
//
//    //starting point
//    buffer.set_xy(x as usize, y as usize, clr);
//
//    for _ in 0..dx {
//        x += x_inc;
//        if p < 0 {
//            //stay in horizontal
//            p += dbl_dy;
//        } else {
//            y += y_inc;
//            //diagonal move
//            p += dbl_dy_dx;
//        }
//        buffer.set_xy(x as usize, y as usize, clr);
//    }
//}

/// Bresenham line algorithm (handles all octants).
pub fn bresenham(buffer: &mut Buffer<u32>, p1: &VX2, p2: &VX2, clr: u32) {
    let dx = (p2.x - p1.x).abs();
    let dy = (p2.y - p1.y).abs();
    // Determine if the line is steep. If so, we'll swap x and y axes.
    let steep = dy > dx;

    // Work on mutable copies
    let (mut x0, mut y0, mut x1, mut y1) = if steep {
        (p1.y, p1.x, p2.y, p2.x) // swap x and y
    } else {
        (p1.x, p1.y, p2.x, p2.y)
    };
    // Ensure we iterate from left to right (increasing x)
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = (y1 - y0).abs();
    let y_step = if y0 < y1 { 1.0 } else { -1.0 };

    let mut err = dx * 0.5;
    let mut y = y0;

    for x in x0.round() as usize..=x1.round() as usize {
        let i_y = y.round() as i32;

        if steep {
            buffer.set_xy(i_y, x as i32, clr); //swapped back
        } else {
            buffer.set_xy(x as i32, i_y, clr);
        }
        err -= dy;
        if err < 0.0 {
            y += y_step;
            err += dx;
        }
    }
}
