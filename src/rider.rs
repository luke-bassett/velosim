use crate::physics::{Force, Position, Velocity};
use crate::vector2d::Vector2D;
use crate::Wind;
use crate::DENSITY_OF_AIR_AT_SEA_LEVEL;

/// Represents a rider "system" which includes their kit and bicycle.
///
/// x is the direction of the race, y is perpendicular to the race.
pub struct Rider {
    position: Position, // x, y
    velocity: Velocity, // m/s in x and y
    power: f64,         // W
    cda: f64,           // m^2
    mass: f64,          // kg
}

impl Rider {
    /// Create a new [Rider] instance with the given power, drag coefficient,
    /// and mass.
    pub(crate) fn new(power: f64, cda: f64, mass: f64) -> Rider {
        Rider {
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
