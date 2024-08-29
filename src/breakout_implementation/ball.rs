pub struct Ball {
    x: u16,
    y: u16,
    direction: DirectionVector,
}

struct DirectionVector {
    x: f64,
    y: f64,
}

impl DirectionVector {
    fn new(x: f64, y: f64) -> Self {
        let magnitude = (x * x + y * y).sqrt() as f64;
        DirectionVector {
            x: x / magnitude,
            y: y / magnitude,
        }
    }
}

use rand::Rng;

fn random_upwards_direction() -> DirectionVector {
    let mut rng = rand::thread_rng();
    DirectionVector {
        x: rng.gen_range(-20..=20),
        y: rng.gen_range(0..=20),
    }
}
