use hecs::Entity;
use hecs::EntityBuilder;
use hecs::World;
use rand::{RngExt};
use raylib::ffi::*;
use raylib::prelude::*;

mod asset_manager;
use asset_manager::*;

mod component;
use component::*;

mod draw;

mod constant;
use constant::*;

impl From<&WorldPosition> for Vector2 {
    fn from(wp: &WorldPosition) -> Self {
        Vector2::new(wp.x, wp.y)
    }
}

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    asset_manager: AssetManager,
    world: hecs::World,
    rng: rand::rngs::ThreadRng,
    ui_state: UiState,
}

struct UiState {
    spell_creator_active: bool,
    transform_grass: bool,
}

impl Game {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(SCREEN_W, SCREEN_H)
            .title("Hello, World")
            .build();
        Game {
            asset_manager: AssetManager::new(&mut rl, &thread),
            rl: rl,
            thread: thread,
            world: World::new(),
            rng: rand::rng(),
            ui_state: UiState {
                spell_creator_active: false,
                transform_grass: false,
            },
        }
    }

    pub fn run(&mut self) {
        self.init();
        while !self.rl.window_should_close() {
            self.update();
            self.draw();
        }
    }

    fn init(&mut self) {
        self.spawn_floor_tiles();
        self.spawn_nature();
    }

    fn update(&mut self) {
        if self.ui_state.transform_grass {
            self.transform::<Grass, Rock>();
            self.ui_state.transform_grass = false;
        }
    }

    fn draw(&mut self) {
        self.rl.draw(&self.thread, |mut d| {
            d.clear_background(Color::GRAY);

            draw::tiles(&mut d, &mut self.world);
            draw::nature(&mut d, &mut self.world);
            draw::textures(&mut d, &self.asset_manager);

            unsafe {
                let pressed = GuiButton(
                    ffi::Rectangle {
                        x: 100.0,
                        y: 100.0,
                        width: 150.0,
                        height: 40.0,
                    },
                    c"Click me".as_ptr(),
                );

                if pressed == 1 {
                    self.ui_state.transform_grass = true;
                }
            }

            d.draw_fps(0, 0);
        });
    }

    fn transform<F, T>(&mut self)
    where
        F: hecs::Component + Bundle,
        T: hecs::Component + Bundle,
    {
        let mut position_bundles = Vec::new();
        let mut del_entities = Vec::new();

        for (_, entity, tile_pos, world_pos) in self
            .world
            .query_mut::<(&F, Entity, &TilePosition, &WorldPosition)>()
            .into_iter()
            .take(5)
        {
            position_bundles.push((
                TilePosition::from(*tile_pos),
                WorldPosition::from(*world_pos),
            ));

            del_entities.push(entity);
        }

        for entity in del_entities {
            if let Err(e) = self.world.despawn(entity) {
                println!("Somehow the entity found in the previous lines doesn't exist now: {e}");
            }
        }
        for position_bundle in position_bundles {
            self.world.spawn(
                EntityBuilder::new()
                    .add_bundle(position_bundle)
                    .add_bundle(T::get_bundle())
                    .build(),
            );
        }
    }

    fn spawn_floor_tiles(&mut self) {
        for x in 0..=30 {
            for y in -15..=18 {
                let tile_pos = &TilePosition::new(x, y);
                if Self::tile_in_bounds(tile_pos) {
                    self.world.spawn((
                        Hexagon {
                            color: Some(Color::PURPLE),
                            texture: None,
                        },
                        TilePosition::new(x, y),
                        WorldPosition::from(Self::tile_to_global(tile_pos)),
                        HexTile {},
                    ));
                }
            }
        }
    }

    fn spawn_nature(&mut self) {
        let mut grass_positions = Vec::new();
        let mut rock_positions = Vec::new();
        for (_, tile_pos, world_pos) in self
            .world
            .query_mut::<(&HexTile, &TilePosition, &WorldPosition)>()
        {
            match self.rng.random::<f32>() {
                x if x < 0.1 => {
                    grass_positions.push((
                        TilePosition::from(*tile_pos),
                        WorldPosition::from(*world_pos),
                    ));
                }
                x if x < 0.2 => {
                    rock_positions.push((
                        TilePosition::from(*tile_pos),
                        WorldPosition::from(*world_pos),
                    ));
                }
                _ => {}
            };
        }

        for position_bundle in rock_positions {
            let mut builder = EntityBuilder::new();
            builder
                .add_bundle(Rock::get_bundle())
                .add_bundle(position_bundle);
            self.world.spawn(builder.build());
        }

        for position_bundle in grass_positions {
            let mut builder = EntityBuilder::new();
            builder
                .add_bundle(Grass::get_bundle())
                .add_bundle(position_bundle);
            self.world.spawn(builder.build());
        }
    }

    fn tile_to_global(tile_pos: &TilePosition) -> Vector2 {
        let x_vec = Vector2::new(1.5, SQRT_3 / 2.0) * tile_pos.x as f32;
        let y_vec = Vector2::new(0.0, SQRT_3) * tile_pos.y as f32;
        (x_vec + y_vec) * TILE_RADIUS
    }

    fn tile_in_bounds(tile_pos: &TilePosition) -> bool {
        let global_pos = Self::tile_to_global(tile_pos);
        let left = global_pos.x - TILE_RADIUS;
        let right = global_pos.x + TILE_RADIUS;
        let top = global_pos.y - TILE_RADIUS;
        let bottom = global_pos.y + TILE_RADIUS;

        left >= 0.0 && right <= SCREEN_W as f32 && top >= 0.0 && bottom <= SCREEN_H as f32
    }
}

// const TILE_COUNT_W: i32 = 12;
// const TILE_COUNT_H: i32 = 8;
// fn draw_grid(d: &mut RaylibDrawHandle) {
//     let x_start = SCREEN_W / 5 / 2;
//     let tile_size = SCREEN_W * 4 / 5 / TILE_COUNT_W;

//     let line_w = SCREEN_W * 4 / 5;
//     let y_start = (SCREEN_H - tile_size * TILE_COUNT_H) / 2;
//     let line_h = SCREEN_H - 2 * y_start;

//     d.draw_rectangle(x_start, y_start, line_w, line_h, Color::MIDNIGHTBLUE);

//     for x_i in 0..=TILE_COUNT_W {
//         d.draw_rectangle(x_start + x_i * tile_size, y_start, 5, line_h, Color::BLACK);
//     }

//     for y_i in 0..=TILE_COUNT_H {
//         d.draw_rectangle(x_start, y_start + y_i * tile_size, line_w, 5, Color::BLACK);
//     }
// }
