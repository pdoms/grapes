use std::cell::RefCell;
use std::rc::Rc;

use crate::linal::vx2::VX2;
use crate::vx2;

pub type Camera2dRef = Rc<RefCell<Camera2d>>;

#[derive(Debug, Default)] 
pub struct Camera2d {
    position: VX2,
}

impl Camera2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: vx2!(x, y),
        }
    }
    pub fn center(w: f32, h: f32) -> Self {
        Self {
            position: vx2!(w*0.5, h*0.5)
        }
    }
}
