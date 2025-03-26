pub trait Vector2D {
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn magnitude(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }

    fn unit(&self) -> Self
    where
        Self: Sized,
    {
        let mag = (self.x().powi(2) + self.y().powi(2)).sqrt();
        if mag < f64::EPSILON {
            Self::from_components(0.0, 0.0)
        } else {
            Self::scale(self, 1.0 / mag)
        }
    }

    fn scale(&self, scalar: f64) -> Self
    where
        Self: Sized,
    {
        Self::from_components(self.x() * scalar, self.y() * scalar)
    }

    fn add(&self, other: &Self) -> Self
    where
        Self: Sized,
    {
        Self::from_components(self.x() + other.x(), self.y() + other.y())
    }

    fn sub(&self, other: &Self) -> Self
    where
        Self: Sized,
    {
        Self::from_components(self.x() - other.x(), self.y() - other.y())
    }

    fn from_components(x: f64, y: f64) -> Self;
}

/// Macro to implement `Vector2D` for any struct with `x` and `y` fields.
#[macro_export]
macro_rules! impl_vector2d {
    ($t:ty) => {
        impl Vector2D for $t {
            fn x(&self) -> f64 {
                self.x
            }
            fn y(&self) -> f64 {
                self.y
            }
            fn from_components(x: f64, y: f64) -> Self {
                Self { x, y }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestVector {
        x: f64,
        y: f64,
    }

    impl_vector2d!(TestVector);

    #[test]
    fn test_magnitude() {
        let v = TestVector { x: 3.0, y: 4.0 };
        assert_eq!(v.magnitude(), 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_magnitude_negative() {
        let v = TestVector { x: -3.0, y: -4.0 };
        assert_eq!(v.magnitude(), 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_unit() {
        let v = TestVector { x: 3.0, y: 3.0 };
        let unit = v.unit();
        assert_eq!(unit.magnitude(), 1.0);
        assert!((unit.x() - unit.y()).abs() < f64::EPSILON);
        assert!((unit.x() - (2.0_f64.sqrt() / 2.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_unit_of_zero() {
        let v = TestVector { x: 0.0, y: 0.0 };
        let unit = v.unit();
        assert_eq!(unit.magnitude(), 0.0);
    }

    #[test]
    fn test_scale() {
        let v = TestVector { x: 3.0, y: 4.0 };
        let scaled = v.scale(2.0);
        assert_eq!(scaled.magnitude(), 10.0);
        assert_eq!(scaled.x(), 6.0);
        assert_eq!(scaled.y(), 8.0);
    }

    #[test]
    fn test_add() {
        let v1 = TestVector { x: 3.0, y: 4.0 };
        let v2 = TestVector { x: 1.0, y: 2.0 };
        let added = v1.add(&v2);
        assert_eq!(added.x(), 4.0);
        assert_eq!(added.y(), 6.0);
    }

    #[test]
    fn test_sub() {
        let v1 = TestVector { x: 3.0, y: 4.0 };
        let v2 = TestVector { x: 1.0, y: 2.0 };
        let subbed = v1.sub(&v2);
        assert_eq!(subbed.x(), 2.0);
        assert_eq!(subbed.y(), 2.0);
    }

    #[test]
    fn test_from_components() {
        let v = TestVector::from_components(1.0, 2.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
    }
}
