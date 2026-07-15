use sfml::{graphics::Drawable, system::Vector2f};

use crate::ui::event::UiEvent;

pub trait UiElement: CustomUi + Drawable {}

pub trait CustomUi {
    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f);
    fn update(&mut self);
    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<UiEvent>>;
}
