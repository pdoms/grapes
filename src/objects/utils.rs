use crate::linal::vertx2::VX2;

pub fn top_left_line(p1: &mut VX2, p2: &mut VX2) {
    if p1.x > p2.x {
        std::mem::swap(&mut p1.x, &mut p2.x);
        std::mem::swap(&mut p1.y, &mut p2.y);
    }
}

pub trait BBox2d {
    fn bbox(&self) -> Bounds;
}

pub fn min_of_n(d: &[f32]) -> Option<f32> {
    d.iter().fold(None, |min, x| match min {
        None => Some(*x),
        Some(y) => Some(if &x < &&y { *x } else { y }),
    })
}
pub fn max_of_n(d: &[f32]) -> Option<f32> {
    d.iter().fold(None, |max, x| match max {
        None => Some(*x),
        Some(y) => Some(if &x > &&y { *x } else { y }),
    })
}

pub struct Bounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

pub fn edge_2d(a: &VX2, b: &VX2, c: &VX2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

#[cfg(test)]
mod test {
    use crate::{objects::utils::top_left_line, vx2};

    #[test]
    fn swapping() {
        let mut p1 = vx2!(3.0, 2.0);
        let mut p2 = vx2!(2.0, 1.0);
        println!("{p1:?}, {p2:?}");
        top_left_line(&mut p1, &mut p2);
        println!("{p1:?}, {p2:?}");
    }
}
