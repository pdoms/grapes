use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[macro_export]
macro_rules! vx2 {
    ($v:expr) => {
        $crate::linal::vx2::VX2::new($v, $v)
    };
    ($x:expr, $y:expr) => {
        $crate::linal::vx2::VX2::new($x, $y)
    };
}

#[derive(Debug, Clone)]
pub struct VX2 {
    pub x: f32,
    pub y: f32,
}

impl Default for VX2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl VX2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y + self.y
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let l = self.length();
        self.x /= l;
        self.y /= l;
    }

    pub fn normalized_clone(&self) -> Self {
        let l = self.length();
        vx2!(self.x / l, self.y / l)
    }

    pub fn ceil(&self) -> Self {
        vx2!(self.x.ceil(), self.y.ceil())
    }
    pub fn floor(&self) -> Self {
        vx2!(self.x.floor(), self.y.floor())
    }
    pub fn abs(&self) -> Self {
        vx2!(self.x.abs(), self.y.abs())
    }
    pub fn rotation(&self, rot_deg: f32) -> Self {
        let theta = rot_deg.to_radians();
        let c = theta.cos();
        let s = theta.sin();
        let x = self.x * c + self.y * s;
        let y = -self.x * s + self.y * c;
        vx2!(x, y)
    }
}

impl Add<&VX2> for VX2 {
    type Output = VX2;

    fn add(self, rhs: &VX2) -> Self::Output {
        vx2!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<&VX2> for VX2 {
    fn add_assign(&mut self, rhs: &VX2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<&VX2> for VX2 {
    type Output = VX2;

    fn sub(self, rhs: &VX2) -> Self::Output {
        vx2!(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<&VX2> for VX2 {
    fn sub_assign(&mut self, rhs: &VX2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<&VX2> for &VX2 {
    type Output = VX2;

    fn sub(self, rhs: &VX2) -> Self::Output {
        vx2!(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<f32> for VX2 {
    type Output = VX2;

    fn add(self, rhs: f32) -> Self::Output {
        vx2!(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<f32> for VX2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub<f32> for VX2 {
    type Output = VX2;

    fn sub(self, rhs: f32) -> Self::Output {
        vx2!(self.x - rhs, self.y - rhs)
    }
}

impl SubAssign<f32> for VX2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul<f32> for VX2 {
    type Output = VX2;

    fn mul(self, rhs: f32) -> Self::Output {
        vx2!(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for VX2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for VX2 {
    type Output = VX2;

    fn div(self, rhs: f32) -> Self::Output {
        vx2!(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for VX2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for VX2 {
    type Output = VX2;

    fn neg(self) -> Self::Output {
        vx2!(-self.x, -self.y)
    }
}
