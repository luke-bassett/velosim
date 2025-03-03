use crate::vector2d::Vector2D;

/// The [Position] of an object in 2d space.
#[derive(Copy, Clone)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    /// Create a new [Position] instance with the given x and y components.
    pub(crate) fn new(x: f64, y: f64) -> Position {
        Position { x, y }
    }

    /// Returns the x component of this [Position]
    pub(crate) fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y component of this [Position]
    pub(crate) fn y(&self) -> f64 {
        self.y
    }
}

/// The [Velocity] of an object in 2d space.
/// expressed in m/s.
#[derive(Copy, Clone)]
pub struct Velocity {
    x: f64,
    y: f64,
}

impl_vector2d!(Velocity);

impl std::fmt::Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Velocity {
    /// Create a new [Velocity] instance with the given x and y components.
    pub(crate) fn new(x: f64, y: f64) -> Velocity {
        Velocity { x, y }
    }

    /// Returns the x component of this [Velocity]
    pub(crate) fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y component of this [Velocity]
    pub(crate) fn y(&self) -> f64 {
        self.y
    }
}

/// A [Force] in 2d space.
pub struct Force {
    x: f64,
    y: f64,
}

impl_vector2d!(Force);

impl Force {
    /// Create a new [Force] instance with the given x and y components.
    pub(crate) fn new(x: f64, y: f64) -> Force {
        Force { x, y }
    }

    /// Returns the x component of this [Force]
    pub(crate) fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y component of this [Force]
    pub(crate) fn y(&self) -> f64 {
        self.y
    }
}
