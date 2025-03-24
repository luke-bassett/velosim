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
