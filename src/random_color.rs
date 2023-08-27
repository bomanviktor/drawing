use rand::Rng;
use raster::Color;

pub trait RandomColor {
    fn random() -> Self;
}

impl RandomColor for Color {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
            a: 255,
        }
    }
}
