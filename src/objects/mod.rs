use collision::gjk::gjk;

use crate::linal::vertx2::VX2;

pub mod circle;
pub mod collision;
pub mod line;
pub mod rectangle;
pub mod tri;
pub mod utils;

pub trait Vertices {
    fn vertices(&self) -> Vec<VX2>;
}

pub trait SupportV {
    fn support(&self, dir: &VX2) -> VX2;
}

pub trait Collision
where
    Self: Sized + Vertices + SupportV,
{
    fn collides<O: Vertices + SupportV + Sized>(&self, with: &O) -> bool {
        gjk(self, with)
    }
}
