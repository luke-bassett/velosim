use std::f64::EPSILON;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub; // very small number

/// Density of air at sea level [kg / m^3].
const DENSITY_OF_AIR_AT_SEA_LEVEL: f64 = 1.225; // kg/m^3

/// Exactly one instance of wind should ever be created for a given simulation.
struct Wind {
    velocity: Velocity,
}

impl Wind {
    /// Create a new instance of [Wind].
    fn new(velocity: Velocity) -> Wind {
        Wind { velocity }
    }

    /// Returns this [Wind]'s [Velocity]
    fn velocity(&self) -> Velocity {
        self.velocity
    }
}

/// The [Position] of an object in 2d space.
#[derive(Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
}

impl Position {
    /// Create a new [Position] instance with the given x and y components.
    fn new(x: f64, y: f64) -> Position {
        Position { x, y }
    }

    /// Returns the x component of this [Position]
    fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y component of this [Position]
    fn y(&self) -> f64 {
        self.y
    }
}

/// The [Velocity] of an object in 2d space.
/// expressed in m/s.
#[derive(Copy, Clone)]
struct Velocity {
    x: f64,
    y: f64,
}

impl std::fmt::Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Velocity {
    /// Create a new [Velocity] instance with the given x and y components.
    fn new(x: f64, y: f64) -> Velocity {
        Velocity { x, y }
    }

    /// Returns the x component of this [Velocity]
    fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y component of this [Velocity]
    fn y(&self) -> f64 {
        self.y
    }

    /// Returns the magnitude of this [Velocity].
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// unit vector (direction) of velocity
    fn unit(&self) -> Velocity {
        let mag = self.magnitude();
        if mag < EPSILON {
            // Avoid divid-by-zero or floating point errors
            Velocity::new(0.0, 0.0)
        } else {
            let x = self.x / mag;
            let y = self.y / mag;
            Velocity::new(x, y)
        }
    }
}

/// A [Force] in 2d space.
struct Force {
    x: f64,
    y: f64,
}

impl Mul<f64> for Force {
    type Output = Force;

    fn mul(self, scalar: f64) -> Self::Output {
        Force {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Add<Force> for Force {
    type Output = Force;

    fn add(self, other: Force) -> Force {
        Force {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Velocity> for Velocity {
    type Output = Velocity;

    fn sub(self, other: Velocity) -> Velocity {
        Velocity {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Represents a rider "system" which includes their kit and bicycle.
///
/// x is the direction of the race, y is perpendicular to the race.
struct Rider {
    position: Position, // x, y
    velocity: Velocity, // m/s in x and y
    power: f64,         // W
    cda: f64,           // m^2
    mass: f64,          // kg
}

impl Rider {
    /// Create a new [Rider] instance with the given power, drag coefficient,
    /// and mass.
    fn new(power: f64, cda: f64, mass: f64) -> Rider {
        Rider {
            position: Position { x: 0.0, y: 0.0 },
            velocity: Velocity { x: 0.0, y: 0.0 },
            power,
            cda,
            mass,
        }
    }

    /// Returns the power of the [Rider].
    fn power(&self) -> f64 {
        self.power
    }

    /// Returns the drag coefficient of the [Rider].
    fn cda(&self) -> f64 {
        self.cda
    }

    /// Returns the mass of the [Rider].
    fn mass(&self) -> f64 {
        self.mass
    }

    /// Returns the position of the [Rider].
    fn position(&self) -> Position {
        self.position
    }

    /// Set the [Rider]'s [Position].
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Returns the velocity of the [Rider].
    fn velocity(&self) -> Velocity {
        self.velocity
    }

    /// Set the [Rider]'s [Velocity].
    fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }
}

/// Calculates the drag [Force] acting on the [Rider]. Drag [Force] is
/// a function of the [Rider]'s [Velocity] and the [Wind] [Velocity].
fn calculate_rider_drag(rider: &Rider, wind: &Wind) -> Force {
    let rider_velocity = rider.velocity();

    let velocity_relative_to_air = rider_velocity - wind.velocity();
    let vel_mag = velocity_relative_to_air.magnitude();
    let drag_mag = 0.5 * rider.cda() * DENSITY_OF_AIR_AT_SEA_LEVEL * vel_mag.powi(2);
    let direction = velocity_relative_to_air.unit();
    Force {
        x: -1.0 * drag_mag * direction.x(),
        y: -1.0 * drag_mag * direction.y(),
    }
}

/// Calculates the [Force] created by the [Rider].
///
/// Handles unrealistic accelleration at low velocity and divide-by-zero at
/// velocity = 0.
fn calculate_rider_force(rider: &Rider) -> Force {
    let min_velocity: f64 = 1.0;
    let rider_velocity = rider.velocity();
    let rider_power = rider.power();

    let v_x = rider_velocity.x().max(min_velocity);
    Force {
        x: rider_power / v_x,
        y: 0.0,
    }
}

/// Updates the [Rider]'s velocity based on the forces acting on them, and an
/// arbitrary time delta `dt`. At the moment only wind is considered.
fn update_rider_velocity(rider: &mut Rider, dt: f64, wind: &Wind) {
    let rider_force = calculate_rider_force(rider);
    let drag_force = calculate_rider_drag(rider, wind);
    let total_force = rider_force + drag_force;

    let current_rider_velocity = rider.velocity();

    let new_rider_velocity = Velocity {
        x: current_rider_velocity.x + (total_force.x / rider.mass()) * dt,
        y: current_rider_velocity.y + (total_force.y / rider.mass()) * dt,
    };

    rider.set_velocity(new_rider_velocity);
}

/// Updates the [Rider]'s position based on their velocity and an arbitrary
/// time delta `dt`.
fn update_rider_position(rider: &mut Rider, dt: f64) {
    let current_rider_position = rider.position();

    let new_rider_position = Position::new(
        current_rider_position.x() + rider.velocity().x() * dt,
        current_rider_position.y() + rider.velocity().y() * dt,
    );

    rider.set_position(new_rider_position)
}

fn main() {
    let mut riders: Vec<Rider> = vec![Rider::new(300.0, 0.3, 80.0)];
    let dt: f64 = 1.0; // seconds
    let wind = Wind::new(Velocity::new(-5.0, 1.0));

    for t in 0..100 {
        for rider in &mut riders {
            update_rider_velocity(rider, dt, &wind);
            update_rider_position(rider, dt);
        }
        for rider in &riders {
            let rider_velocity = rider.velocity();

            println!(
                "t {:<5} | vx {:>8.2} | vy {:>8.2} | v {:>8.2}",
                t,
                rider_velocity.x(),
                rider_velocity.y(),
                rider_velocity.magnitude()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_increases_monotonically_from_zero() {
        let mut rider = Rider::new(300.0, 0.3, 80.0);
        let wind = Wind::new(Velocity::new(1.0, 0.0));

        let dt: f64 = 1.0;

        for _ in 0..100 {
            let old_rider_velocity = rider.velocity();
            update_rider_velocity(&mut rider, dt, &wind);
            let new_rider_velocity = rider.velocity();

            assert!(
                new_rider_velocity.x() >= old_rider_velocity.x(),
                "Velocity decreased! prev: {}, current: {}",
                old_rider_velocity.x(),
                new_rider_velocity.x()
            );
        }
    }
}
