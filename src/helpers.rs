use std::collections::HashMap;

use sfml::system::Vector2f;
use strum::IntoEnumIterator;

use crate::{
    game::asset_manager::SpellComponentTypes,
    ui::{self, ui_id::UiId, widgets::*},
};

pub fn spawn_button_grid(
    rows: usize,
    cols: usize,
    rel_padding: Vector2f,
    padding_sides: bool,
) -> (
    Vec<Box<dyn ui::traits::UiElement>>,
    HashMap<UiId, Option<SpellComponentTypes>>,
) {
    let mut buttons: Vec<Box<dyn ui::traits::UiElement>> = Vec::new();
    let mut ids = HashMap::new();

    let mut relative_size = Vector2f::new(
        (1.0 - rel_padding.x * (cols + 1) as f32) / cols as f32,
        (1.0 - rel_padding.y * (rows + 1) as f32) / rows as f32,
    );

    if !padding_sides {
        relative_size.x = (1.0 - rel_padding.x * (cols - 1) as f32) / cols as f32;
    }

    let mut spell_component_iter = SpellComponentTypes::iter();

    for row in 0..rows {
        for col in 0..cols {
            let id = UiId::new();
            let mut relative_position = Vector2f::new(
                col as f32 * (relative_size.x + rel_padding.x) + rel_padding.x,
                row as f32 * (relative_size.y + rel_padding.y) + rel_padding.y,
            );

            if !padding_sides {
                relative_position.x = col as f32 * (relative_size.x + rel_padding.x);
            }

            let mut text = None;

            if let Some(spell_component) = spell_component_iter.next() {
                text = Some(String::from(spell_component.button_name()));
                ids.insert(id.clone(), Some(spell_component));
            } else {
                ids.insert(id.clone(), None);
            }

            buttons.push(Box::new(Button {
                relative_size: relative_size,
                relative_position: relative_position,
                id: id,
                text: text,
                ..Default::default()
            }));
        }
    }

    (buttons, ids)
}
