use hecs::{Entity, EntityBuilder, World};
use sfml::{
    cpp::FBox,
    graphics::{Color, RenderTarget, RenderWindow},
    system::Vector2f,
    window::{self, ContextSettings, Event, Key, VideoMode, mouse},
};

mod asset_manager;
use asset_manager::*;

mod component;
use component::*;

mod draw;

mod constant;
use constant::*;

mod spawner;
use spawner::*;

use crate::{ui, ui::Ui};
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
    ui: Ui,
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
            ui: Ui {
                windows: vec![ui::Window {
                    parent_size: Vector2f::new(SCREEN_W as f32, SCREEN_H as f32),
                    relative_position: Vector2f::new(0.1, 0.1),
                    relative_size: Vector2f::new(0.5, 0.5),
                    bg_color: Color::GREEN,
                    children: vec![
                        Box::new(ui::Button {
                            relative_position: Vector2f::new(0.1, 0.1),
                            relative_size: Vector2f::new(0.1, 0.1),
                            ..Default::default()
                        }),
                        Box::new(ui::Button {
                            relative_position: Vector2f::new(0.2, 0.2),
                            relative_size: Vector2f::new(0.1, 0.1),
                            ..Default::default()
                        }),
                        Box::new(ui::Button {
                            relative_position: Vector2f::new(0.3, 0.3),
                            relative_size: Vector2f::new(0.1, 0.1),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }],
            },
        }
    }

    fn init(&mut self) {
        self.ui.init();

        let mut spawner = self.spawner();
        spawner.spawn_floor_tiles();
        spawner.spawn_nature();
    }

    pub fn run(&mut self) {
        self.init();
        while self.window.is_open() {
            self.update();
            self.draw();
        }
    }

    fn update(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => self.window.close(),
                    Key::R => self.transform::<Rock, Grass>(10),
                    Key::G => self.transform::<Grass, Rock>(10),
                    _ => {}
                },
                Event::MouseButtonPressed { button, x, y } => match button {
                    mouse::Button::Left => self.ui.on_click(Vector2f::new(x as f32, y as f32)),
                    _ => {}
                },
                _ => {}
            }
        }

        self.ui.update();
    }

    fn draw(&mut self) {
        self.window.clear(Color::rgb(2, 9, 46));
        draw::tiles(&mut self.window, &mut self.world);
        draw::nature(&mut self.window, &mut self.world);
        draw::textures(&mut self.window, &mut self.asset_manager);

        self.window.draw(&self.ui);

        self.window.display();
    }

    fn transform<F, T>(&mut self, count: usize)
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
            .take(count)
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

    fn spawner(&mut self) -> Spawner<'_> {
        Spawner {
            world: &mut self.world,
            rng: &mut self.rng,
        }
    }
}
