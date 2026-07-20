use std::collections::HashMap;

use sfml::{graphics::Color, system::Vector2f};
use strum::IntoEnumIterator;

use crate::{
    game::asset_manager::SpellComponentTypes,
    ui::{self, ui_id::UiId, widgets::*},
};

pub fn spawn_spell_component_selector_buttons(
    button_count: usize,
) -> (
    Vec<Box<dyn ui::traits::UiElement>>,
    HashMap<UiId, Option<SpellComponentTypes>>,
) {
    let mut buttons: Vec<Box<dyn ui::traits::UiElement>> = Vec::new();
    let mut ids = HashMap::new();

    let mut spell_component_iter = SpellComponentTypes::iter();

    for _ in 0..button_count {
        let id = UiId::new();
        let mut text = None;

        if let Some(spell_component) = spell_component_iter.next() {
            text = Some(String::from(spell_component.button_name()));
            ids.insert(id.clone(), Some(spell_component));
        } else {
            ids.insert(id.clone(), None);
        }

        //TODO: add constructor for buttons w/0 rel sizing
        let mut button = Button::new(Vector2f::new(100.0, 100.0), Vector2f::new(100.0, 100.0), id)
            .set_bg_color(Color::rgb(100, 100, 100));
        if let Some(text) = text {
            button = button.set_text(text);
        }
        buttons.push(Box::new(button));
    }

    (buttons, ids)
}
