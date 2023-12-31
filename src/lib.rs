use godot::engine::utilities::pow;
use godot::engine::utilities::sqrt;
use godot::engine::FastNoiseLite;
use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::engine::ISprite2D;
use godot::engine::Noise;
use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    pub speed: f64,
    angular_speed: f64,

    #[base]
    sprite: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world! 2"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        self.sprite.rotate((self.angular_speed * delta) as f32);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32

        let rotation = self.sprite.get_rotation();
        let velocity = Vector2::LEFT.rotated(rotation) * self.speed as f32;
        self.sprite.translate(velocity * delta as f32);
    }

    // #[func]
    // fn increase_speed(&mut self, amount: f64) {
    //     self.speed += amount;
    //     self.sprite.emit_signal("speed_increased".into(), &[]);
    // }

    // #[signal]
    // fn speed_increased();
}

#[godot_api]
impl Player {
    #[func]
    fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    #[signal]
    fn speed_increased();
}

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

        for x in 0..50 {
            for y in 0..20 {
                let x_coord = x as f32 * x_vec.x;
                let mut y_coord = y as f32 * triangle_height as f32;

                if x % 2 == 1 {
                    y_coord += x_vec.y;
                }
                // if y % 2 == 1 {
                //     x_coord += x_vec.x;
                // }

                let mut ground_clone = ground
                    .duplicate()
                    .expect("Failed to clone ground sprite")
                    .cast::<Node2D>();
                ground_clone.set_name("GroundClone".into());
                ground_clone.set_position(Vector2::new(x_coord, y_coord));

                let noise =
                    noise.get_noise_2d(x_coord * scale + offset, y_coord * scale + offset) + 0.5;

                godot_print!("Noise: {}", noise);

                let color = Color::from_rgb(noise, noise, noise);
                ground_clone.set_modulate(color);

                // godot_print!("Ground sprite cloned {:?}", ground_clone);

                let asds = ground_clone.upcast::<Node>();

                self.sprite.add_child(asds);
            }
        }

        // clone the ground sprite
        let mut ground_clone = ground
            .duplicate()
            .expect("Failed to clone ground sprite")
            .cast::<Node2D>();
        ground_clone.set_name("GroundClone".into());
        ground_clone.set_position(Vector2::new(0 as f32, 0 as f32));
        ground_clone.set_modulate(Color::from_rgb(0 as f32, 0 as f32, 1 as f32));

        // godot_print!("Ground sprite cloned {:?}", ground_clone);

        let asds = ground_clone.upcast::<Node>();

        self.sprite.add_child(asds);
    }
}
