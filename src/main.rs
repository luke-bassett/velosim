const DENSITY_OF_AIR: f64  = 1.225; // kg/m^3

struct Rider {
    position: f64,     // x, y
    velocity: f64,     // m/s in x and y
    power: f64,        // W
    cda: f64,          // m^2
    mass: f64,         // kg
}

impl Rider {
    fn update_velocity(&mut self, dt: f64) {
        let f_rider = self.power / self.velocity; // TODO: Handle start-from-0 edge case
        let f_drag = 0.5 * self.cda * DENSITY_OF_AIR * self.velocity.powi(2);
        let f_total = f_rider - f_drag;
        let acceleration = f_total / self.mass;
        self.velocity += acceleration * dt;
    }

    fn update_position(&mut self, dt: f64) {
        self.position += self.velocity * dt;
    }
}

fn main() {
    let mut riders: Vec<Rider> = vec![
        Rider {
            position: 0.0,
            velocity: 1.0, 
            power: 300.0,
            cda: 0.3,
            mass: 80.0,
        },
    ];
    let dt: f64 = 1.0; // seconds

    for t in 0..100 {
        for rider in &mut riders {
            rider.update_velocity(dt);
            rider.update_position(dt);
        }
        for rider in &riders {
            println!("t {} | pos {} | v {}", t, rider.position, rider.velocity)
        }
    }
}
