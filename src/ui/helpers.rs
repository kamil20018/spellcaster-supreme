use sfml::system::Vector2f;

use crate::ui::{self, ui_id, widgets::*};

pub fn spawn_button_grid(
    rows: usize,
    cols: usize,
    rel_offset: Vector2f,
) -> (Vec<Box<dyn ui::traits::UiElement>>, Vec<Vec<u64>>) {
    let mut buttons: Vec<Box<dyn ui::traits::UiElement>> = Vec::new();
    let mut ids: Vec<Vec<u64>> = Vec::new();

    for row in 0..rows {
        ids.push(Vec::new());
        for col in 0..cols {
            let id = ui_id::new_id();
            buttons.push(Box::new(Button {
                relative_size: Vector2f::new(0.1, 0.1),
                relative_position: Vector2f::new(0.11 * col as f32, 0.11 * row as f32) + rel_offset,
                id: id.clone(),
                ..Default::default()
            }));
            ids[row].push(id);
        }
    }

    (buttons, ids)
}
