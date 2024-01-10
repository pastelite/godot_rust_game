use godot::{
    engine::{
        utilities::{pow, sqrt},
        FastNoiseLite, Polygon2D,
    },
    prelude::*,
};

use crate::generator::custom_noise::CustomNoiseGenerator;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MapSpawner {
    seed: u64,

    #[base]
    sprite: Base<Node2D>,
}

#[godot_api]
impl INode2D for MapSpawner {
    fn init(_node: Base<Node2D>) -> Self {
        Self {
            seed: 0,
            sprite: _node,
        }
    }

    fn ready(&mut self) {
        godot_print!("Ready to generate map"); // Prints to the Godot console
        self.generate_map();
    }
}

#[godot_api]
impl MapSpawner {
    #[func]
    fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    #[func]
    fn generate_map(&mut self) {
        godot_print!("Generating map with seed {}", self.seed);

        // find the ground sprite
        let ground = self.sprite.get_node("Ground".into());
        if ground.is_none() {
            godot_print!("Ground sprite not found");
            return;
        }

        let ground = ground.unwrap().try_cast::<Node2D>();
        if ground.is_err() {
            godot_print!("Ground sprite is not a Node2D");
            return;
        }

        let ground = ground.unwrap();
        let offset = 0.45;
        let wanted_scale = 3;
        let noise = FastNoiseLite::new();

        let triangle_width: f64 = 32.0;
        let triangle_height = sqrt(pow(triangle_width, 2.0) - pow(triangle_width / 2.0, 2.0));

        let scale = wanted_scale as f32 / triangle_width as f32;

        let x_vec = Vector2::RIGHT.rotated(0.5235) * (triangle_height as f32);

        let noise_generator: CustomNoiseGenerator = CustomNoiseGenerator::new(offset, scale);

        let mut poly = Polygon2D::new_alloc();
        poly.set_name("Poly".into());

        for y in 0..100 {
            for x in 0..100 {
                let x_coord = x as f32 * x_vec.x;
                let mut y_coord = y as f32 * triangle_height as f32;

                if x % 2 == 1 {
                    y_coord += x_vec.y;
                }

                let mut ground_clone = ground
                    .duplicate()
                    .expect("Failed to clone ground sprite")
                    .cast::<Node2D>();
                ground_clone.set_name("GroundClone".into());

                let noise = noise_generator.get_noise(x_coord, y_coord);
                let noise = stepify(noise);

                let elevated = noise * 50.0;
                // let elevated = 0.0f32;

                godot_print!("Noise: {}", noise);

                let color = if noise < 0.5 {
                    Color::from_rgb(0.1, 0.1, noise * 2.0)
                } else {
                    Color::from_rgb(noise, noise, noise)
                };

                ground_clone.set_modulate(color);
                ground_clone.set_position(Vector2::new(x_coord, y_coord - elevated));
                ground_clone.set_z_index((noise * 40.0) as i32 - 40);

                // godot_print!("Ground sprite cloned {:?}", ground_clone);

                let asds = ground_clone.upcast::<Node>();

                self.sprite.add_child(asds);
            }
        }
    }
}

fn stepify(x: f32) -> f32 {
    let x = x * 20.0;
    let x = x.floor();
    x / 20.0
}
