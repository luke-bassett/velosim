use crate::Velocity;

pub struct Wind {
    velocity: Velocity,
}

impl Wind {
    /// Create a new instance of [Wind].
    pub(crate) fn new(velocity: Velocity) -> Wind {
        Wind { velocity }
    }

    /// Returns this [Wind]'s [Velocity]
    pub(crate) fn velocity(&self) -> Velocity {
        self.velocity
    }
}
