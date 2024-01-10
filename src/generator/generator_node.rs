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

        // generate the map
        let config = TileMapConfig {
            edge_width: 16.0,
            seed: self.seed.clone(),
            offset: 0.45,
            scale_factor: 3,
        };
        let tile_map_data = TileMapData::generate(100, 100, config);

        // draw the map
        for y in 0..tile_map_data.y_size {
            let row_data = &tile_map_data.tiles[y];

            let mut draw_tile = |tile_data: &TileData| {
                let mut ground_clone = ground
                    .duplicate()
                    .expect("Failed to clone ground sprite")
                    .cast::<Node2D>();
                ground_clone.set_name("GroundClone".into());

                let color = if tile_data.height < 0.5 {
                    Color::from_rgb(0.1, 0.1, tile_data.height * 2.0)
                } else {
                    Color::from_rgb(tile_data.height, tile_data.height, tile_data.height)
                };

                let new_coordinates =
                    tile_data.coordinates - Vector2::new(0.0, 50.0 * tile_data.height);

                ground_clone.set_modulate(color);
                ground_clone.set_position(new_coordinates);

                self.sprite.add_child(ground_clone.upcast::<Node>());
            };

            // -_-_-_- <- how tile in each row placed currently
            // render the tiles above first, then render the below ones

            for x in (0..tile_map_data.x_size).step_by(2) {
                draw_tile(&row_data[x]);
            }

            for x in (1..tile_map_data.x_size).step_by(2) {
                draw_tile(&row_data[x]);
            }
        }
    }

    #[func]
    fn _generate_map(&mut self) {}
}

struct TileData {
    height: f32,
    coordinates: Vector2,
}

struct TileMapData {
    tiles: Vec<Vec<TileData>>,
    x_size: usize,
    y_size: usize,
    config: TileMapConfig,
}

struct TileMapConfig {
    edge_width: f32,
    seed: u64,
    offset: f32,
    scale_factor: u32,
}

impl TileMapData {
    fn generate(x_size: usize, y_size: usize, config: TileMapConfig) -> Self {
        // const
        let edge_width = config.edge_width;
        let seed = config.seed;
        let offset = config.offset;
        let scale_factor = config.scale_factor;

        // precalcuate number
        let big_r = edge_width as f32;
        let small_r = (edge_width.powi(2) - (edge_width / 2.0).powi(2)).sqrt();
        let x_offset = big_r + big_r / 2.0;
        let y_offset = small_r * 2f32;

        let scale = scale_factor as f32 / edge_width as f32;
        let noise_generator = CustomNoiseGenerator::new(offset, scale);

        // generate tiles
        let mut tiles = Vec::new();

        for y in 0..y_size {
            let mut row = Vec::new();
            let y_coord = y as f32 * y_offset;

            for x in 0..x_size {
                let x_coord = x as f32 * x_offset;
                let real_y_coord = if x % 2 == 1 {
                    y_coord + small_r
                } else {
                    y_coord
                };

                let noise = noise_generator.get_noise(x_coord, real_y_coord);

                row.push(TileData {
                    height: noise,
                    coordinates: Vector2::new(x_coord, real_y_coord),
                });
            }

            tiles.push(row);
        }

        Self {
            tiles,
            x_size,
            y_size,
            config,
        }
    }
}

fn stepify(x: f32) -> f32 {
    let x = x * 20.0;
    let x = x.floor();
    x / 20.0
}
