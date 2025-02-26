/// Density of air at sea level [kg / m^3].
const DENSITY_OF_AIR_AT_SEA_LEVEL: f64 = 1.225; // kg/m^3

/// Represents a rider "system" which includes their kit and bicycle.
struct Rider {
    position: f64, // x, y
    velocity: f64, // m/s in x and y
    power: f64,    // W
    cda: f64,      // m^2
    mass: f64,     // kg
}

impl Rider {
    fn update_velocity(&mut self, dt: f64) {
        let force_rider = self.calculate_force_rider();
        let force_drag = 0.5 * self.cda * DENSITY_OF_AIR_AT_SEA_LEVEL * self.velocity.powi(2);
        let force_total = force_rider - force_drag;
        let acceleration = force_total / self.mass;
        self.velocity += acceleration * dt;
    }

    /// Calculates the force created by the rider.
    ///
    /// Handles unrealistic accelleration at low velocity and divide-by-zero at
    /// velocity = 0.
    fn calculate_force_rider(&mut self) -> f64 {
        let force_rider = if self.velocity > 0.5 {
            self.power / self.velocity
        } else {
            self.power.sqrt()
        };
        force_rider
    }

    fn update_position(&mut self, dt: f64) {
        self.position += self.velocity * dt;
    }
}

fn main() {
    let mut riders: Vec<Rider> = vec![Rider {
        position: 0.0,
        velocity: 0.0,
        power: 300.0,
        cda: 0.3,
        mass: 80.0,
    }];
    let dt: f64 = 1.0; // seconds

    for t in 0..101 {
        for rider in &mut riders {
            rider.update_velocity(dt);
            rider.update_position(dt);
        }
        for rider in &riders {
            println!(
                "t {:<5} | p {:>10.3} | v {:>8.3}",
                t, rider.position, rider.velocity
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
            position: 0.0,
            velocity: 0.0, // Start at zero
            power: 300.0,  // Assume some reasonable power
            cda: 0.3,
            mass: 80.0,
        };

        let dt: f64 = 1.0;
        let mut previous_velocity = rider.velocity;

        for _ in 0..100 {
            rider.update_velocity(dt);
            assert!(
                rider.velocity >= previous_velocity,
                "Velocity decreased! prev: {}, current: {}",
                previous_velocity,
                rider.velocity
            );
            previous_velocity = rider.velocity;
        }
    }
}
