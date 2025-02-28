use std::f64::EPSILON;

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
        if mag < EPSILON {
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
