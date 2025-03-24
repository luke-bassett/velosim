#[macro_use]
mod vector2d;
mod physics;
mod rider;
mod wind;
mod simulation;
use physics::Velocity;
use rider::{update_rider_position, update_rider_velocity, Rider};
use vector2d::Vector2D;
use wind::Wind;

/// Density of air at sea level [kg / m^3].
const DENSITY_OF_AIR_AT_SEA_LEVEL: f64 = 1.225; // kg/m^3

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
