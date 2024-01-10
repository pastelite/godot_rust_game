use std::f32::consts::E;
mod generator;
use generator::custom_noise::CustomNoiseGenerator;

use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::engine::ISprite2D;
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
