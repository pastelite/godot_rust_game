use godot::engine::FastNoiseLite;

pub struct CustomNoiseGenerator {
    offset: f32,
    scale: f32,
}

impl CustomNoiseGenerator {
    pub fn new(offset: f32, scale: f32) -> Self {
        Self { offset, scale }
    }

    pub fn get_noise(&self, x: f32, y: f32) -> f32 {
        let noise = FastNoiseLite::new();

        let base_noise = (noise.get_noise_2d(
            x * self.scale * 2.0 + self.offset,
            y * self.scale * 2.0 + self.offset,
        ) + 1.0)
            / 2.0;
        let top_noise = (noise.get_noise_2d(
            x * self.scale + self.offset * 2.0,
            y * self.scale + self.offset * 2.0,
        ) + 1.0)
            / 2.0;

        flat_center(base_noise) * 0.9 + top_noise * 0.15
    }
}

fn flat_center(x: f32) -> f32 {
    3.0 * 0.75 * (1.0 - x).powi(2) * x + 3.0 * 0.25 * (1.0 - x) * x.powi(2) + x.powi(3)
}
