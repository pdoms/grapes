pub mod color;
pub mod modify;
pub mod presets;

#[cfg(test)]
mod test {
    use crate::colors::{
        color::{Color, ColorU32},
        modify::{is_transparent, set_alpha, swap_alpha},
        presets::GrapesColors,
    };

    #[test]
    pub fn color_conversions() {
        let base = 0x272FA3FF;
        let as_clr_rgba = ColorU32::Rgba(base);
        let as_clr_argb = ColorU32::Argb(swap_alpha(base));
        assert_eq!(Color::from(as_clr_rgba), Color::new(0x27, 0x2F, 0xA3, 0xFF));
        assert_eq!(Color::from(as_clr_argb), Color::new(0x27, 0x2F, 0xA3, 0xFF));
        let diff_alpha = set_alpha(swap_alpha(base), 0xAA);
        assert_eq!(
            Color::from_argb(diff_alpha),
            Color::new(0x27, 0x2F, 0xA3, 0xAA)
        );

        let teal: u32 = 4278222976;
        let clr = Color::from_argb(teal);
        assert_eq!(Color::from(GrapesColors::Teal), clr);

        let base_opaque = Color::from_rgba(0x272FA3FF);
        let base_transparent = Color::from_rgba(0x272FA300);
        assert!(!is_transparent(base_opaque.into()));
        assert!(is_transparent(base_transparent.into()));
    }
}
