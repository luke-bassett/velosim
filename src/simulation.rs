use crate::rider::{update_rider_position, update_rider_velocity, Rider};
use crate::wind::Wind;

pub struct Simulation {
    pub riders: Vec<Rider>,
    pub wind: Wind,
    pub dt: f64, // tick duration in seconds
    pub time: f64,
}

impl Simulation {
    pub fn new(riders: Vec<Rider>, wind: Wind, dt: f64) -> Self {
        Simulation {
            riders,
            wind,
            dt,
            time: 0.0,
        }
    }

    pub fn tick(&mut self) {
        // for each tick we need to do a few things:
        // 1. update the position of each rider
        // 2. update the velocity of each rider
        // 3. increment the time
        for rider in &mut self.riders {
            update_rider_velocity(rider, self.dt, &self.wind);
            update_rider_position(rider, self.dt);
        }
        self.time += self.dt;
    }

    pub fn print_rider_positions(&self) {
        for rider in &self.riders {
            println!(
                "Rider {} is at ({}, {})",
                rider.id,
                rider.position.x(),
                rider.position.y()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::Velocity;

    #[test]
    fn test_time_increments() {
        let riders = vec![Rider::new(100.0, 0.5, 70.0)];
        let wind = Wind::new(Velocity::new(0.0, 0.0));
        let dt = 1.0;
        let mut sim = Simulation::new(riders, wind, dt);
        sim.tick();
        assert_eq!(sim.time, 1.0);
    }

    #[test]
    fn test_rider_moves() {
        let mut rider = Rider::new(100.0, 0.5, 70.0);
        rider.set_velocity(Velocity::new(10.0, 0.0));
        let riders = vec![rider];
        let wind = Wind::new(Velocity::new(0.0, 0.0));
        let dt = 1.0;
        let mut sim = Simulation::new(riders, wind, dt);
        sim.tick();
        assert!(sim.riders[0].position().x() > 0.0);
    }

    #[test]
    fn test_multiple_riders() {
        let riders = vec![
            Rider::new(100.0, 0.5, 70.0),
            Rider::new(120.0, 0.4, 75.0),
        ];
        let wind = Wind::new(Velocity::new(0.0, 0.0));
        let dt = 1.0;
        let mut sim = Simulation::new(riders, wind, dt);
        sim.tick();
        assert_eq!(sim.riders.len(), 2);
    }

    #[test]
    fn test_wind_affects_movement() {
        let mut rider = Rider::new(100.0, 0.5, 70.0);
        rider.set_velocity(Velocity::new(10.0, 0.0));
        let riders = vec![rider];
        let wind = Wind::new(Velocity::new(-5.0, 0.0)); // Headwind
        let dt = 1.0;
        let mut sim = Simulation::new(riders, wind, dt);
        sim.tick();
        let velocity_with_wind = sim.riders[0].velocity().x();

        // Now test without wind
        let mut rider = Rider::new(100.0, 0.5, 70.0);
        rider.set_velocity(Velocity::new(10.0, 0.0));
        let riders = vec![rider];
        let wind = Wind::new(Velocity::new(0.0, 0.0));
        let mut sim = Simulation::new(riders, wind, dt);
        sim.tick();
        let velocity_without_wind = sim.riders[0].velocity().x();

        // With headwind, rider should move slower than without wind
        assert!(velocity_with_wind < velocity_without_wind);
    }
}
