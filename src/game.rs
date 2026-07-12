use hecs::{Entity, EntityBuilder, World};
use rand::RngExt;
use sfml::{
    cpp::FBox,
    graphics::{Color, RenderTarget, RenderWindow},
    system::Vector2f,
    window::{self, ContextSettings, Event, Key, VideoMode},
};

mod asset_manager;
use asset_manager::*;

mod spell_creator;

mod component;
use component::*;

mod draw;

mod constant;
use constant::*;

impl From<&WorldPosition> for Vector2f {
    fn from(wp: &WorldPosition) -> Self {
        Vector2f::new(wp.x, wp.y)
    }
}

pub struct Game {
    window: FBox<RenderWindow>,
    asset_manager: AssetManager,
    world: hecs::World,
    rng: rand::rngs::ThreadRng,
    ui_state: UiState,
}

struct UiState {
    spell_creator_active: bool,
}

impl Game {
    pub fn new() -> Self {
        let window: FBox<RenderWindow> = RenderWindow::new(
            VideoMode::new(SCREEN_W, SCREEN_H, 32),
            "SFML Example",
            window::Style::CLOSE,
            &ContextSettings::default(),
        )
        .expect("Cannot create a new Render Window.");

        Game {
            window: window,
            asset_manager: AssetManager::new(),
            world: World::new(),
            rng: rand::rng(),
            ui_state: UiState {
                spell_creator_active: false,
            },
        }
    }

    pub fn run(&mut self) {
        self.init();
        while self.window.is_open() {
            self.update();
            self.draw();
        }
    }

    fn init(&mut self) {
        self.spawn_floor_tiles();
        self.spawn_nature();
    }

    fn update(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => self.window.close(),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn draw(&mut self) {
        self.window.clear(Color::rgb(2, 9, 46));
        draw::tiles(&mut self.window, &mut self.world);
        draw::nature(&mut self.window, &mut self.world);
        draw::textures(&mut self.window, &mut self.asset_manager);
        self.window.display();
    }

    #[allow(unused)]
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
            position_bundles.push((TilePosition::from(*tile_pos), WorldPosition::from(*world_pos)));

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
        for x in 0..=45 {
            for y in -20..=20 {
                let tile_pos = &TilePosition::new(x, y);
                if Self::tile_in_bounds(tile_pos) {
                    self.world.spawn((
                        Hexagon { color: Color::MAGENTA },
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
        for (_, tile_pos, world_pos) in self.world.query_mut::<(&HexTile, &TilePosition, &WorldPosition)>() {
            match self.rng.random::<f32>() {
                x if x < 0.1 => {
                    grass_positions.push((TilePosition::from(*tile_pos), WorldPosition::from(*world_pos)));
                }
                x if x < 0.2 => {
                    rock_positions.push((TilePosition::from(*tile_pos), WorldPosition::from(*world_pos)));
                }
                _ => {}
            };
        }

        for position_bundle in rock_positions {
            let mut builder = EntityBuilder::new();
            builder.add_bundle(Rock::get_bundle()).add_bundle(position_bundle);
            self.world.spawn(builder.build());
        }

        for position_bundle in grass_positions {
            let mut builder = EntityBuilder::new();
            builder.add_bundle(Grass::get_bundle()).add_bundle(position_bundle);
            self.world.spawn(builder.build());
        }
    }

    fn tile_to_global(tile_pos: &TilePosition) -> Vector2f {
        let x_vec = Vector2f::new(1.5, SQRT_3 / 2.0) * tile_pos.x as f32;
        let y_vec = Vector2f::new(0.0, SQRT_3) * tile_pos.y as f32;
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
