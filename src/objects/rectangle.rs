use std::f32;

use crate::{
    colors::color::Color,
    constants::DEFAULT_FOREGROUND,
    linal::vertx2::VX2,
    renderer::two_d::{Render, Renderer},
    vx2,
};

use super::{
    Collision, SupportV, Vertices,
    collision::{
        epa::{EpaResult, epa},
        gjk::{furthest_polygon, gjk_for_epa},
    },
    line::bresenham,
    tri::Tri2d,
    utils::{BBox2d, max_of_n, min_of_n},
};

#[derive(Default, Clone, Copy)]
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

    ///tl, tr, br, bl
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
    fn fill_clr<C: Into<u32> + Copy>(&self, renderer: &mut Renderer, c: C) {
        let [tl, tr, br, bl] = self.vertices_arr();
        Tri2d::new(tl, tr, bl).fill_clr(renderer, c);
        Tri2d::new(bl, tr, br).fill_clr(renderer, c);
    }
    fn draw_clr<C: Into<u32> + Copy>(&self, renderer: &mut Renderer, c: C) {
        let [tl, tr, br, bl] = self.vertices_arr();
        bresenham(renderer.buffer_mut(), &tl, &tr, c.into());
        bresenham(renderer.buffer_mut(), &tr, &br, c.into());
        bresenham(renderer.buffer_mut(), &br, &bl, c.into());
        bresenham(renderer.buffer_mut(), &bl, &tl, c.into());
    }

    fn with_texture(&self, renderer: &mut Renderer, texture: &crate::textures::Texture) {
        if self.rotation > f32::EPSILON {
            panic!("Texture rendering on rectangle with rotation is not yet implemented");
        }

        let bbox = self.bbox();
        let start_y = bbox.min_y as i32;
        let end_y = bbox.max_y as i32;

        let start_x = bbox.min_x as i32;
        let width = self.size.x as usize;
        if width != texture.size().x as usize {
            panic!(
                "At the moment we need texture to be the same size as the rectangle for this to work"
            );
        }

        let mut texture_y = 0;
        let tex = texture.get_buffer().get_ptr();
        unsafe {
            for y in start_y..end_y {
                let dst = renderer.buffer_mut().get_xy(start_x, y).unwrap();

                let src = tex.add(texture_y as usize * width);
                texture_y += 1;
                std::ptr::copy_nonoverlapping(src, dst, width);
            }
        }
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

impl SupportV for Rectangle {
    fn support(&self, dir: &VX2) -> VX2 {
        let verts = self.vertices_arr();
        let idx = furthest_polygon(&verts, dir);
        verts[idx]
    }
}
impl SupportV for &Rectangle {
    fn support(&self, dir: &VX2) -> VX2 {
        let verts = self.vertices_arr();
        let idx = furthest_polygon(&verts, dir);
        verts[idx]
    }
}

impl Collision for Rectangle {
    fn collides_epa<O: Vertices + SupportV + Sized>(&self, with: &O) -> Option<EpaResult> {
        if let Some(simplex) = gjk_for_epa(self, with) {
            let verts_self = self.vertices();
            let verts_with = with.vertices();
            return epa(
                simplex,
                |dir: &VX2| verts_self[furthest_polygon(&verts_self, dir)],
                |dir: &VX2| verts_with[furthest_polygon(&verts_with, dir)],
            );
        }
        return None;
    }
}
impl Collision for &Rectangle {
    fn collides_epa<O: Vertices + SupportV + Sized>(&self, with: &O) -> Option<EpaResult> {
        if let Some(simplex) = gjk_for_epa(self, with) {
            let verts_self = self.vertices();
            let verts_with = with.vertices();
            return epa(
                simplex,
                |dir: &VX2| verts_self[furthest_polygon(&verts_self, dir)],
                |dir: &VX2| verts_with[furthest_polygon(&verts_with, dir)],
            );
        }
        return None;
    }
}

impl BBox2d for Rectangle {
    fn bbox(&self) -> super::utils::Bounds {
        let verts = self.vertices_arr();
        let xs: Vec<f32> = verts.iter().map(|v| v.x).collect();
        let ys: Vec<f32> = verts.iter().map(|v| v.y).collect();

        super::utils::Bounds {
            min_x: min_of_n(&xs).unwrap(),
            max_x: max_of_n(&xs).unwrap(),
            min_y: min_of_n(&ys).unwrap(),
            max_y: max_of_n(&ys).unwrap(),
        }
    }
}
