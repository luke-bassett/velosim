use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::f64::EPSILON; // very small number

/// Density of air at sea level [kg / m^3].
const DENSITY_OF_AIR_AT_SEA_LEVEL: f64 = 1.225; // kg/m^3

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
struct Wind {
    velocity: Velocity, // m/s in x and y
}
struct Position {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone)]
struct Velocity {
    x: f64,
    y: f64,
}

impl Velocity {
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// unit vector (direction) of velocity
    fn unit(&self) -> Velocity {
        let mag = self.magnitude();
        if mag < EPSILON { // Avoid divid-by-zero or floating point errors
            Velocity { x: 0.0, y: 0.0 }
        } else {
            Velocity {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }
}

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

impl Rider {
    fn calculate_force_drag(&self, wind: &Wind) -> Force {
        let velocity_relative_to_air = self.velocity - wind.velocity;
        let vel_mag = velocity_relative_to_air.magnitude();
        let drag_mag = 0.5 * self.cda * DENSITY_OF_AIR_AT_SEA_LEVEL * vel_mag.powi(2);
        let direction = velocity_relative_to_air.unit();
        Force {
            x: -1.0 * drag_mag * direction.x,
            y: -1.0 * drag_mag * direction.y,
        }
    }

    /// Calculates the force created by the rider.
    ///
    /// Handles unrealistic accelleration at low velocity and divide-by-zero at
    /// velocity = 0.
    fn calculate_force_rider(&self) -> Force {
        let min_velocity: f64 = 1.0;
        let v_x = self.velocity.x.max(min_velocity);
        Force { x: self.power / v_x, y: 0.0}
    }

    fn update_velocity(&mut self, dt: f64, wind: &Wind) {
        let force_rider = self.calculate_force_rider();
        let force_drag = self.calculate_force_drag(wind);
        let force_total = force_rider + force_drag;
        self.velocity = Velocity {
            x: self.velocity.x + (force_total.x / self.mass) * dt,
            y: self.velocity.y + (force_total.y / self.mass) * dt,
        }
    }

    fn update_position(&mut self, dt: f64) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }
}

fn main() {
    let mut riders: Vec<Rider> = vec![Rider {
        position: Position { x: 0.0, y: 0.0 },
        velocity: Velocity { x: 1.0, y: 0.0 },
        power: 300.0,
        cda: 0.3,
        mass: 80.0,
    }];
    let dt: f64 = 1.0; // seconds
    let wind = Wind {
        velocity: Velocity { x: -5.0, y: 0.0 },
    };

    for t in 0..100 {
        for rider in &mut riders {
            rider.update_velocity(dt, &wind);
            rider.update_position(dt);
        }
        for rider in &riders {
            println!(
                "t {:<5} | v {:>8.2}",
                t, rider.velocity.magnitude()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_increases_monotonically_from_zero() {
        let mut rider = Rider {
            position: Position { x: 0.0, y: 0.0 },
            velocity: Velocity { x: 0.0, y: 0.0 }, // Start at zero
            power: 300.0,                          // Assume some reasonable power
            cda: 0.3,
            mass: 80.0,
        };
        let wind = Wind {
            velocity: Velocity { x: 1.0, y: 0.0 },
        };

        let dt: f64 = 1.0;
        let mut previous_velocity = rider.velocity.x;

        for _ in 0..100 {
            rider.update_velocity(dt, &wind);
            assert!(
                rider.velocity.x >= previous_velocity,
                "Velocity decreased! prev: {}, current: {}",
                previous_velocity,
                rider.velocity.x
            );
            previous_velocity = rider.velocity.x;
        }
    }
}
