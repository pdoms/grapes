use crate::linal::vx2::VX2;

pub fn top_left_line(p1: &mut VX2, p2: &mut VX2) {
    if p1.x > p2.x {
        std::mem::swap(&mut p1.x, &mut p2.x);
        std::mem::swap(&mut p1.y, &mut p2.y);
    }
}

pub trait BBox2d {
    fn bbox(&self) -> Bounds;
}

pub struct Bounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
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
