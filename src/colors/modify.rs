
///x11 order is expected
pub fn set_alpha(v: u32, a: u8) -> u32 {
    let r = (v >> (8 * 2) & 0xFF) as u32;
    let g = (v >> (8 * 1) & 0xFF) as u32;
    let b = (v >> (8 * 0) & 0xFF) as u32;
    b | g << 8 | r << 16 | (a as u32) << 24
}

///will create x11 order
pub fn swap_alpha(v: u32) -> u32 {
    let a = (v >> (8 * 0) & 0xFF) as u32;
    let value = v >> 8;
    value | a << 24
}

/// assumes x11 order
pub fn is_transparent(c: u32) -> bool {
    (c >> (8 * 3) & 0xFF) as u8 == 0
}

///x11 order is expected for both arguments
pub fn alpha_blend(fg: u32, bg: u32) -> u32 {
    let fg_a = (fg >> (8 * 3) & 0xFF) as u32;
    let alpha = fg_a + 1;
    let inv_alpha = 256 - fg_a;

    let fg_r = (fg >> (8 * 2) & 0xFF) as u32;
    let fg_g = (fg >> (8 * 1) & 0xFF) as u32;
    let fg_b = (fg >> (8 * 0) & 0xFF) as u32;

    let bg_r = (bg >> (8 * 2) & 0xFF) as u32;
    let bg_g = (bg >> (8 * 1) & 0xFF) as u32;
    let bg_b = (bg >> (8 * 0) & 0xFF) as u32;

    let a = 0xFF;
    let r = (alpha * fg_r + inv_alpha * bg_r) >> 8;
    let g = (alpha * fg_g + inv_alpha * bg_g) >> 8;
    let b = (alpha * fg_b + inv_alpha * bg_b) >> 8;

    b | g << 8 | r << 16 | a << 24
}
