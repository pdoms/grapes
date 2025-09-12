use crate::{
    buffer::Buffer,
    colors::color::Color,
    constants::DEFAULT_FOREGROUND,
    linal::vx2::VX2,
    renderer::two_d::{Render, Renderer},
    vx2,
};

pub struct Circle {
    pub pos: VX2,
    pub r: f32,
    pub fill: Color,
    pub stroke: Color,
}

impl Circle {
    pub fn new(pos: VX2, r: f32) -> Self {
        Self {
            pos,
            r,
            fill: DEFAULT_FOREGROUND.into(),
            stroke: DEFAULT_FOREGROUND.into(),
        }
    }
    pub fn set_fill<C: Into<Color> + Copy>(&mut self, fill: C) {
        self.fill = fill.into()
    }
    pub fn set_stroke<C: Into<Color> + Copy>(&mut self, stroke: C) {
        self.stroke = stroke.into()
    }
}

impl Render for Circle {
    fn draw(&self, renderer: &mut Renderer) {
        mid_point_circle(renderer.buffer_mut(), self, self.stroke);
    }

    fn fill(&self, renderer: &mut Renderer) {
        fill_circle_brute_force(renderer.buffer_mut(), &self, self.fill);
    }
    fn fill_clr<C: Into<u32> + Copy>(&self, renderer: &mut Renderer, c: C) {
        fill_circle_brute_force(renderer.buffer_mut(), &self, c.into());
    }
}

fn draw_circle(buffer: &mut Buffer<u32>, center: &VX2, pos: &VX2, color: u32) {
    buffer.set_xy((center.x + pos.x) as i32, (center.y + pos.y) as i32, color);
    buffer.set_xy((center.x - pos.x) as i32, (center.y + pos.y) as i32, color);
    buffer.set_xy((center.x + pos.x) as i32, (center.y - pos.y) as i32, color);
    buffer.set_xy((center.x - pos.x) as i32, (center.y - pos.y) as i32, color);
    buffer.set_xy((center.x + pos.y) as i32, (center.y + pos.x) as i32, color);
    buffer.set_xy((center.x - pos.y) as i32, (center.y + pos.x) as i32, color);
    buffer.set_xy((center.x + pos.y) as i32, (center.y - pos.x) as i32, color);
    buffer.set_xy((center.x - pos.y) as i32, (center.y - pos.x) as i32, color);
}
pub fn mid_point_circle<C: Into<Color> + Copy>(
    buffer: &mut Buffer<u32>,
    circle: &Circle,
    color: C,
) {
    let radius = circle.r as i32;
    let mut f = 1 - radius;
    let mut x = 0;
    let mut y = radius;

    let mut ddf_x = 1;
    let mut ddf_y = -2 * radius;

    let clr: u32 = color.into().into();
    buffer.set_xy(circle.pos.x as i32, (circle.pos.y as i32) + radius, clr);
    buffer.set_xy(circle.pos.x as i32, (circle.pos.y as i32) - radius, clr);
    buffer.set_xy((circle.pos.x as i32) + radius, circle.pos.y as i32, clr);
    buffer.set_xy((circle.pos.x as i32) - radius, circle.pos.y as i32, clr);

    while x < y {
        if f >= 0 {
            y -= 1;
            ddf_y += 2;
            f += ddf_y;
        }
        x += 1;
        ddf_x += 2;
        f += ddf_x;
        draw_circle(buffer, &circle.pos, &vx2!(x as f32, y as f32), clr);
    }
}

pub fn fill_circle_brute_force<C: Into<Color> + Copy>(
    buffer: &mut Buffer<u32>,
    circle: &Circle,
    color: C,
) {
    let radius = circle.r as i32;
    let clr: u32 = color.into().into();

    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                buffer.set_xy(circle.pos.x as i32 + x, circle.pos.y as i32 + y, clr);
            }
        }
    }
}
