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

pub trait Collision
where
    Self: Vertices,
{
    fn collides<V: Vertices>(&self, with: V) -> bool {
        let verts1 = self.vertices();
        let verts2 = with.vertices();
        gjk(&verts1, &verts2)
    }
}
