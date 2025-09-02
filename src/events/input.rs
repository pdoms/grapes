use crate::{internal::window::Window, linal::vx2::VX2, vx2};

use super::{keyboard::K, mouse::MouseButton};

pub struct Events<'w> {
    inner: &'w Window,
}

impl<'w> Events<'w> {
    pub fn new(w: &'w Window) -> Self {
        Self { inner: w }
    }

    pub fn get_window_size(&self) -> VX2 {
        let (width, height) = self.inner.get_size();
        vx2!(width as f32, height as f32)
    }
    pub fn get_screen_size(&self) -> VX2 {
        let (width, height) = self.inner.get_screen_size();
        vx2!(width as f32, height as f32)
    }
    pub fn get_mouse_position(&self) -> Option<VX2> {
        self.inner
            .get_mouse_position()
            .map(|pos| vx2!(pos.0, pos.1))
    }

    pub fn key_pressed(&self, k: K) -> bool {
        self.inner.key_pressed(k)
    }
    pub fn key_down(&self, k: K) -> bool {
        self.inner.key_down(k)
    }
    pub fn key_released(&self, k: K) -> bool {
        self.inner.key_released(k)
    }
    pub fn button_is_down(&self, b: MouseButton) -> bool {
        self.inner.button_is_down(b as usize)
    }
    //TODO scroll!
}
