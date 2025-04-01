use std::sync::atomic::{AtomicU32, Ordering};

use crate::physics::{Force, Position, Velocity};
use crate::vector2d::Vector2D;
use crate::Wind;
use crate::DENSITY_OF_AIR_AT_SEA_LEVEL;

/// Represents a rider "system" which includes their kit and bicycle.
///
/// x is the direction of the race, y is perpendicular to the race.
pub struct Rider {
    pub id: u32,
    pub position: Position, // x, y
    velocity: Velocity,     // m/s in x and y
    power: f64,             // W
    cda: f64,               // m^2
    mass: f64,              // kg
}

static NEXT_RIDER_ID: AtomicU32 = AtomicU32::new(0);

impl Rider {
    /// Create a new [Rider] instance with the given power, drag coefficient,
    /// and mass.
    pub(crate) fn new(power: f64, cda: f64, mass: f64) -> Rider {
        Rider {
            id: NEXT_RIDER_ID.fetch_add(1, Ordering::Relaxed),
            position: Position::new(0.0, 0.0),
            velocity: Velocity::new(0.0, 0.0),
            power,
            cda,
            mass,
        }
    }

    /// Returns the power of the [Rider].
    pub(crate) fn power(&self) -> f64 {
        self.power
    }

    /// Returns the drag coefficient of the [Rider].
    pub(crate) fn cda(&self) -> f64 {
        self.cda
    }

    /// Returns the mass of the [Rider].
    pub(crate) fn mass(&self) -> f64 {
        self.mass
    }

    /// Returns the position of the [Rider].
    pub(crate) fn position(&self) -> Position {
        self.position
    }

    /// Set the [Rider]'s [Position].
    pub(crate) fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Returns the velocity of the [Rider].
    pub(crate) fn velocity(&self) -> Velocity {
        self.velocity
    }

    /// Set the [Rider]'s [Velocity].
    pub(crate) fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }
}

/// Calculates the drag [Force] acting on the [Rider]. Drag [Force] is
/// a function of the [Rider]'s [Velocity] and the [Wind] [Velocity].
pub fn calculate_rider_drag(rider: &Rider, wind: &Wind) -> Force {
    let rider_velocity = rider.velocity();

    let velocity_relative_to_air = rider_velocity.sub(&wind.velocity());
    let vel_mag = velocity_relative_to_air.magnitude();
    let drag_mag = 0.5 * rider.cda() * DENSITY_OF_AIR_AT_SEA_LEVEL * vel_mag.powi(2);
    let direction = velocity_relative_to_air.unit();
    Force::new(
        -1.0 * drag_mag * direction.x(),
        -1.0 * drag_mag * direction.y(),
    )
}

/// Calculates the [Force] created by the [Rider].
///
/// Handles unrealistic accelleration at low velocity and divide-by-zero at
/// velocity = 0.
pub fn calculate_rider_force(rider: &Rider) -> Force {
    let min_velocity: f64 = 1.0;
    let rider_velocity = rider.velocity();
    let rider_power = rider.power();

    let v_x = rider_velocity.x().max(min_velocity);
    Force::new(rider_power / v_x, 0.0)
}

/// Updates the [Rider]'s velocity based on the forces acting on them, and an
/// arbitrary time delta `dt`. At the moment only wind is considered.
pub fn update_rider_velocity(rider: &mut Rider, dt: f64, wind: &Wind) {
    let rider_force = calculate_rider_force(rider);
    let drag_force = calculate_rider_drag(rider, wind);
    let total_force = rider_force.add(&drag_force);

    let current_rider_velocity = rider.velocity();

    let new_rider_velocity = Velocity::new(
        current_rider_velocity.x() + (total_force.x() / rider.mass()) * dt,
        current_rider_velocity.y() + (total_force.y() / rider.mass()) * dt,
    );

    rider.set_velocity(new_rider_velocity);
}

/// Updates the [Rider]'s position based on their velocity and an arbitrary
/// time delta `dt`.
pub fn update_rider_position(rider: &mut Rider, dt: f64) {
    let current_rider_position = rider.position();

    let new_rider_position = Position::new(
        current_rider_position.x() + rider.velocity().x() * dt,
        current_rider_position.y() + rider.velocity().y() * dt,
    );

    rider.set_position(new_rider_position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rider_creation() {
        let rider = Rider::new(250.0, 0.3, 75.0);
        assert_eq!(rider.power(), 250.0);
        assert_eq!(rider.cda(), 0.3);
        assert_eq!(rider.mass(), 75.0);
        assert_eq!(rider.position().x(), 0.0);
        assert_eq!(rider.position().y(), 0.0);
        assert_eq!(rider.velocity().x(), 0.0);
        assert_eq!(rider.velocity().y(), 0.0);
    }

    #[test]
    fn test_rider_id_increment() {
        let rider1 = Rider::new(250.0, 0.3, 75.0);
        let rider2 = Rider::new(250.0, 0.3, 75.0);
        assert_eq!(rider2.id, rider1.id + 1);
    }

    #[test]
    fn test_rider_position_update() {
        let mut rider = Rider::new(250.0, 0.3, 75.0);
        rider.set_velocity(Velocity::new(10.0, 0.0));
        update_rider_position(&mut rider, 1.0);
        assert_eq!(rider.position().x(), 10.0);
        assert_eq!(rider.position().y(), 0.0);
    }

    #[test]
    fn test_rider_velocity_update() {
        let mut rider = Rider::new(250.0, 0.3, 75.0);
        let wind = Wind::new(Velocity::new(0.0, 0.0));
        update_rider_velocity(&mut rider, 1.0, &wind);
        // With 250W power and 75kg mass, initial acceleration should be ~3.33 m/s²
        assert!(rider.velocity().x() > 0.0);
    }

    #[test]
    fn test_rider_drag_calculation() {
        let mut rider = Rider::new(250.0, 0.3, 75.0);
        rider.set_velocity(Velocity::new(10.0, 0.0));
        let wind = Wind::new(Velocity::new(0.0, 0.0));
        let drag = calculate_rider_drag(&rider, &wind);
        // Drag should be negative (opposing motion)
        assert!(drag.x() < 0.0);
        assert_eq!(drag.y(), 0.0);
    }

    #[test]
    fn test_rider_force_at_zero_velocity() {
        let rider = Rider::new(250.0, 0.3, 75.0);
        let force = calculate_rider_force(&rider);
        // Should use minimum velocity of 1.0 m/s
        assert_eq!(force.x(), 250.0);
        assert_eq!(force.y(), 0.0);
    }
}
