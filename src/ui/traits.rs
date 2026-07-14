use sfml::{graphics::Drawable, system::Vector2f};

pub trait UiElement: CustomUi + Drawable {}

pub trait CustomUi {
    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f);
    fn update(&mut self);
}
