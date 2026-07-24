use std::collections::{HashMap, HashSet};

use sfml::{
    cpp::FBox,
    graphics::{Color, RenderTarget, RenderWindow},
    system::{Vector2f, Vector2i, Vector2u},
    window::{self, ContextSettings, Event, Key, VideoMode, mouse},
};

pub mod asset_manager;

mod play_field;
use play_field::*;

mod component;
use component::*;

mod constant;
use constant::*;

mod spawner;
mod style;

mod spell_grid;

use crate::{boxed_vec, game::spell_grid::SpellGrid};
use crate::{
    game::asset_manager::SpellComponentTypes,
    helpers,
    ui::{
        // macros,
        Ui,
        event::{EventFromUi, EventToUi},
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

pub struct Game<'a> {
    window: FBox<RenderWindow>,
    play_field: PlayField,
    spell_grid: SpellGrid<SPELL_GRID_WIDTH, SPELL_GRID_HEIGHT>,
    ui: Ui<'a>,
    ui_mappings: UiMappings,
    ui_state: UiState,
}

pub struct UiMappings {
    exit_button: UiId,
    spell_components: HashMap<UiId, Option<SpellComponentTypes>>,
    spell_grid: HashSet<UiId>,
}

impl UiMappings {
    fn button_press(&self, id: UiId) -> Option<UiAction> {
        if id == self.exit_button {
            return Some(UiAction::ExitGame);
        } else if let Some(Some(spell_component)) = self.spell_components.get(&id) {
            return Some(UiAction::SelectedSpellComponent(*spell_component));
        }
        None
    }

    fn grid_button_press(&self, id: UiId) -> Option<UiAction> {
        if self.spell_grid.contains(&id) {
            return Some(UiAction::SpawnGridComponent);
        }
        None
    }
}

enum UiAction {
    ExitGame,
    SelectedSpellComponent(SpellComponentTypes),
    SpawnGridComponent,
}

pub struct UiState {
    selected_spell_component: Option<SpellComponentTypes>,
}

impl<'a> Game<'a> {
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

        let exit_button_id = UiId::new();
        let exit_button = Button::new(Vector2f::new(0.1, 0.1), Vector2f::new(0.0, 0.0), exit_button_id)
            .set_bg_color(Color::rgb(100, 100, 100));

        let (buttons, spawn_spell_component_mappings) = helpers::spawn_spell_component_selector_buttons(10);

        let mut spell_component_grid_mappings = HashSet::new();
        let mut grid_buttons: Vec<Box<dyn UiElement>> = Vec::new();
        for _row in 0..11 {
            for _col in 0..11 {
                let id = UiId::new();
                spell_component_grid_mappings.insert(id);
                grid_buttons.push(Box::new(Button::new_dynamic(id).set_bg_color(Color::WHITE)));
            }
        }

        Game {
            window: window,
            play_field: PlayField::new(Vector2u::new(SCREEN_W / 2, SCREEN_H)),
            spell_grid: SpellGrid::new(),
            ui: Ui::new(
                Vector2f::new(SCREEN_W as f32, SCREEN_H as f32),
                boxed_vec![
                    // exit button
                    exit_button,
                    // spell component choice buttons
                    Grid::new(
                        Vector2f::new(0.5, 1.0 / 9.0),
                        Vector2f::new(0.5, 8.0 / 9.0),
                        UiId::new(),
                        Vector2i::new(5, 2),
                        RelativePadding {
                            top: 0.0,
                            botton: 0.02,
                            left: 0.005,
                            right: 0.005,
                            columns: 0.005,
                            rows: 0.02,
                        },
                        buttons,
                    )
                    .set_bg_color(style::BACKGROUND_DARK_BLUE),
                    // spell creator grid
                    Grid::new(
                        Vector2f::new(0.5, 8.0 / 9.0),
                        Vector2f::new(0.5, 0.0),
                        UiId::new(),
                        Vector2i::new(SPELL_GRID_WIDTH as i32, SPELL_GRID_HEIGHT as i32),
                        RelativePadding {
                            top: 0.005,
                            botton: 0.005,
                            left: 0.005,
                            right: 0.005,
                            columns: 0.005,
                            rows: 0.005,
                        },
                        grid_buttons,
                    )
                    .set_bg_color(style::BACKGROUND_DARK_BLUE),
                ],
            ),
            ui_mappings: UiMappings {
                spell_components: spawn_spell_component_mappings,
                exit_button: exit_button_id,
                spell_grid: spell_component_grid_mappings,
            },
            ui_state: UiState {
                selected_spell_component: None,
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
        while let Some(event) = self.ui.next_event() {
            self.process_ui_event(&event);
        }
        self.ui.update();
        self.play_field.update();
    }

    fn process_ui_event(&mut self, event: &EventFromUi) {
        match event {
            EventFromUi::ButtonClicked(button_id) => {
                if let Some(ui_action) = self.ui_mappings.button_press(*button_id) {
                    match ui_action {
                        UiAction::ExitGame => self.window.close(),
                        UiAction::SelectedSpellComponent(spell_component_types) => {
                            self.ui_state.selected_spell_component = Some(spell_component_types)
                        }
                        _ => {}
                    }
                }
            }
            EventFromUi::GridButtonClicked(button_id, grid_position) => {
                if let Some(ui_action) = self.ui_mappings.grid_button_press(*button_id) {
                    match ui_action {
                        UiAction::SpawnGridComponent => {
                            if let Some(spell_component) = self.ui_state.selected_spell_component {
                                self.ui.process_incoming_event(EventToUi::SetTexture(
                                    *button_id,
                                    spell_component.get_texture(),
                                ));

                                self.spell_grid.set_component(grid_position.0, spell_component);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self) {
        self.window.clear(Color::rgb(2, 9, 46));
        self.window.draw(&self.play_field);
        self.window.draw(&self.ui);
        self.window.display();
    }
}
