#[macro_use]
mod vector2d;
mod physics;
mod rider;
mod simulation;
mod wind;
use physics::Velocity;
use rider::Rider;
use simulation::Simulation;
use wind::Wind;

/// Density of air at sea level [kg / m^3].
const DENSITY_OF_AIR_AT_SEA_LEVEL: f64 = 1.225; // kg/m^3

fn main() {
    let riders: Vec<Rider> = vec![
        Rider::new(300.0, 0.3, 80.0),
        Rider::new(280.0, 0.28, 75.0),
        Rider::new(320.0, 0.32, 85.0),
    ];
    let dt: f64 = 1.0; // seconds
    let wind = Wind::new(Velocity::new(-5.0, 1.0));
    let mut simulation = Simulation::new(riders, wind, dt);

    for _ in 0..10 {
        simulation.tick();
        simulation.print_rider_positions();
    }
}
