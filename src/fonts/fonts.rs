use std::path::Path;

use fontdue::FontSettings;

use crate::{
    colors::{
        color::Color,
        modify::{alpha_blend, set_alpha},
    },
    linal::vertx2::VX2,
    textures::Texture,
};

pub struct Font {
    inner: fontdue::Font,
}

impl Font {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let data = std::fs::read(path).unwrap(); //@ERROR
        Self {
            inner: fontdue::Font::from_bytes(data, FontSettings::default()).unwrap(), // @ERROR
        }
    }


    /// Monospace only at the moment
    pub fn width(&self, text: &[u8], fs: f32) -> f32 {
        let rasterized = self.inner.rasterize(text[0] as char, fs);
        let advance_width = rasterized.0.advance_width.ceil();
        text.len() as f32 * advance_width
    }

    pub fn render_into_texture(
        &self,
        text: &[u8],
        pos: VX2,
        fs: f32,
        color: Color,
        texture: &mut Texture,
    ) {
        if text.len() == 0 {
            return;
        }
        let rasterized = self.inner.rasterize(text[0] as char, fs);

        let advance_width = rasterized.0.advance_width.ceil();
        let base_y = pos.y;
        let mut line_x = pos.x;

        for c in text {
            let (metrics, data) = self.inner.rasterize((*c) as char, fs);
            let start_x = line_x as i32 + metrics.xmin;
            let bottom_y = base_y as i32 - metrics.ymin;
            let hi_y = bottom_y - metrics.height as i32;
            let mut data_idx = 0;
            let frame = texture.get_buffer_mut();
            let stride = frame.width();
            for y in hi_y..bottom_y {
                unsafe {
                    if let Some(scan_line) = frame.get(y * stride as i32 + start_x) {
                        for x in 0..metrics.width {
                            let pix = *scan_line.add(x);
                            let coverage = data[data_idx];
                            data_idx += 1;
                            let blend = alpha_blend(set_alpha(color.into(), coverage), pix);
                            scan_line.add(x).write(blend);
                        }
                    }
                }
            }
            line_x += advance_width;
        }
    }
}
