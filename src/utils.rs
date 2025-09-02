const FULL_CIRCLE_DEG: f32 = 360.0;

#[macro_export]
macro_rules! min {
    ($a:expr, $b:expr) => {
        $a.min($b)
    };
}
#[macro_export]
macro_rules! max {
    ($a:expr, $b:expr) => {
        $a.max($b)
    };
}

pub fn wrap_degrees_360(angle: f32) -> f32 {
    ((angle % FULL_CIRCLE_DEG) + FULL_CIRCLE_DEG) % FULL_CIRCLE_DEG
}

#[cfg(test)]
mod test {
    use crate::utils::wrap_degrees_360;

    #[test]
    fn wrap_360() {
        let angles = vec![
            (0.0, 0.0),
            (45.0, 45.0),
            (180.0, 180.0),
            (359.0, 359.0),
            (360.0, 0.0),
            (361.0, 1.0),
            (400.0, 40.0),
        ];
        let is_same = |a: f32, b: f32| (a - b).abs() < 0.01;
        for (angle, res) in angles {
            assert!(is_same(wrap_degrees_360(angle), res));
        }
    }
}
