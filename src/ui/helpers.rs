use sfml::system::Vector2f;

use crate::ui::{self, ui_id, widgets::*};

pub fn spawn_button_grid(
    rows: usize,
    cols: usize,
    rel_padding: Vector2f,
    padding_sides: bool,
) -> (Vec<Box<dyn ui::traits::UiElement>>, Vec<Vec<u64>>) {
    let mut buttons: Vec<Box<dyn ui::traits::UiElement>> = Vec::new();
    let mut ids: Vec<Vec<u64>> = Vec::new();

    let mut relative_size = Vector2f::new(
        (1.0 - rel_padding.x * (cols + 1) as f32) / cols as f32,
        (1.0 - rel_padding.y * (rows + 1) as f32) / rows as f32,
    );

    if !padding_sides {
        relative_size.x = (1.0 - rel_padding.x * (cols - 1) as f32) / cols as f32;
    }

    for row in 0..rows {
        ids.push(Vec::new());
        for col in 0..cols {
            let id = ui_id::new_id();
            let mut relative_position = Vector2f::new(
                col as f32 * (relative_size.x + rel_padding.x) + rel_padding.x,
                row as f32 * (relative_size.y + rel_padding.y) + rel_padding.y,
            );

            if !padding_sides {
                relative_position.x = col as f32 * (relative_size.x + rel_padding.x);
            }
            buttons.push(Box::new(Button {
                relative_size: relative_size,
                relative_position: relative_position,
                id: id.clone(),
                ..Default::default()
            }));
            ids[row].push(id);
        }
    }

    (buttons, ids)
}
