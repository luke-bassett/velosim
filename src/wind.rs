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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Velocity;

    #[test]
    fn test_wind_creation() {
        let wind = Wind::new(Velocity::new(5.0, 2.0));
        assert_eq!(wind.velocity().x(), 5.0);
        assert_eq!(wind.velocity().y(), 2.0);
    }

    #[test]
    fn test_wind_zero_velocity() {
        let wind = Wind::new(Velocity::new(0.0, 0.0));
        assert_eq!(wind.velocity().x(), 0.0);
        assert_eq!(wind.velocity().y(), 0.0);
    }

    #[test]
    fn test_wind_negative_velocity() {
        let wind = Wind::new(Velocity::new(-3.0, -1.0));
        assert_eq!(wind.velocity().x(), -3.0);
        assert_eq!(wind.velocity().y(), -1.0);
    }
}
