use sfml::{
    cpp::FBox,
    graphics::{Color, RenderTarget, RenderWindow},
    system::{Vector2f, Vector2i, Vector2u},
    window::{self, ContextSettings, Event, Key, VideoMode, mouse},
};

pub mod asset_manager;

mod play_field;
use play_field::*;

mod spell_creator;
use spell_creator::*;

mod component;
use component::*;

mod constant;
use constant::*;

mod spawner;
mod style;

use crate::{
    helpers,
    ui::{
        self, Ui,
        event::UiEvent,
        padding::RelativePadding,
        traits::UiElement,
        ui_id::UiId,
        widgets::{Button, Grid},
    },
};

impl From<&WorldPosition> for Vector2f {
    fn from(wp: &WorldPosition) -> Self {
        Vector2f::new(wp.x, wp.y)
    }
}

pub struct Game {
    window: FBox<RenderWindow>,
    play_field: PlayField,
    spell_creator: SpellCreator,
    ui: Ui,
}

#[allow(unused)]
#[derive(Clone, Copy)]
struct ButtonHandles {
    button_1: u64,
    button_2: u64,
    button_3: u64,
}

impl Game {
    pub fn new() -> Self {
        let mut window: FBox<RenderWindow> = RenderWindow::new(
            VideoMode::new(SCREEN_W, SCREEN_H, 32),
            "Spellcaster Supreme",
            window::Style::CLOSE,
            &ContextSettings::default(),
        )
        .expect("Cannot create a new Render Window.");
        window.set_framerate_limit(60);
        window.set_position(Vector2i::new(270, 190));

        let (buttons, _button_handles) = helpers::spawn_button_grid(2, 9, Vector2f::new(0.005, 0.05), false);

        let mut grid_buttons: Vec<Box<dyn UiElement>> = Vec::new();
        for _row in 0..11 {
            for _col in 0..11 {
                grid_buttons.push(Box::new(Button {
                    id: UiId::new(),
                    bg_color: Color::RED,
                    ..Default::default()
                }));
            }
        }

        Game {
            window: window,
            play_field: PlayField::new(Vector2u::new(SCREEN_W / 2, SCREEN_H)),
            spell_creator: SpellCreator::new(
                Vector2u::new(SCREEN_W / 2, SCREEN_H * 8 / 9),
                Vector2f::new(SCREEN_W as f32 / 2.0, 0.0),
            ),
            ui: Ui {
                windows: vec![
                    // spell component choice buttons
                    ui::Window {
                        parent_size: Vector2f::new(SCREEN_W as f32, SCREEN_H as f32),
                        relative_position: Vector2f::new(0.5, 8.0 / 9.0),
                        relative_size: Vector2f::new(0.5, 1.0 / 9.0),
                        bg_color: style::BACKGROUND_DARK_BLUE,
                        children: buttons,
                        ..Default::default()
                    },
                    // spell creator grid
                    ui::Window {
                        parent_size: Vector2f::new(SCREEN_W as f32, SCREEN_H as f32),
                        relative_position: Vector2f::new(0.5, 0.0),
                        relative_size: Vector2f::new(0.5, 8.0 / 9.0),
                        bg_color: style::BACKGROUND_DARK_BLUE,
                        children: vec![Box::new(Grid {
                            grid_size: Vector2i::new(11, 11),
                            relative_size: Vector2f::new(1.0, 1.0),
                            children: grid_buttons,
                            padding: RelativePadding {
                                top: 0.005,
                                botton: 0.005,
                                left: 0.005,
                                right: 0.005,
                                columns: 0.005,
                                rows: 0.005,
                            },
                            ..Default::default()
                        })],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        }
    }

    pub fn run(&mut self) {
        self.init();
        while self.window.is_open() {
            self.process_input();
            self.update();
            self.draw();
        }
    }

    fn init(&mut self) {
        self.ui.init();
        self.play_field.init();
    }

    pub fn process_input(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => self.window.close(),
                    Key::R => self.play_field.transform::<Rock, Grass>(10),
                    Key::G => self.play_field.transform::<Grass, Rock>(10),
                    _ => {}
                },
                Event::MouseButtonPressed { button, x, y } => match button {
                    mouse::Button::Left => self.ui.on_click(Vector2f::new(x as f32, y as f32)),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn update(&mut self) {
        self.ui.update();
        self.play_field.update();
        self.spell_creator.update();

        while let Some(event) = &self.ui.next_event() {
            match event {
                UiEvent::ButtonClicked(button_id) => println!("button_id from event {}", button_id.value()),
            }
        }
    }

    fn draw(&mut self) {
        self.window.clear(Color::rgb(2, 9, 46));
        self.window.draw(&self.play_field);
        // self.window.draw(&self.spell_creator);
        self.window.draw(&self.ui);
        self.window.display();
    }
}
