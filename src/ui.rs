use sfml::{
    graphics::{Color, Drawable, RenderStates, RenderTarget, Sprite, Transformable},
    system::Vector2f,
};

pub mod event;
pub mod padding;
pub mod traits;
pub mod ui_id;
pub mod widget;
pub mod widgets;
pub use event::EventFromUi;

use crate::ui::{event::EventToUi, traits::UiElement, widget::WidgetData};

pub struct Ui {
    pub event_queue: Vec<EventFromUi>,

    pub widget: WidgetData,

    pub parent_size: Vector2f,
    pub parent_position: Vector2f,
    pub relative_size: Vector2f,
    pub relative_position: Vector2f,
    pub bg_color: Color,

    pub children: Vec<Box<dyn UiElement>>,
}

impl Ui {
    pub fn init(&mut self) {
        self.widget.init(
            self.parent_size,
            self.parent_position,
            self.relative_size,
            self.relative_position,
        );

        for child in &mut self.children {
            child.init(self.widget.real_size, self.widget.real_position);
        }
    }

    pub fn update(&mut self) {
        self.widget.render_texture.clear(self.bg_color);
        for child in &mut self.children {
            child.update();
        }

        for child in &self.children {
            self.widget.render_texture.draw(child.as_ref());
        }
        self.widget.render_texture.display();
    }

    pub fn on_click(&mut self, click_pos: Vector2f) {
        if self.widget.was_clicked(click_pos) && self.widget.clickable {
            for child in &self.children {
                if let Some(child_events) = child.on_click(click_pos) {
                    self.event_queue.extend(child_events);
                }
            }
        }
    }

    pub fn process_incoming_event(&mut self, event: EventToUi) {
        match event {
            EventToUi::SetTexture(ui_id, texture) => {
                for child in &mut self.children {
                    if child.is_id(ui_id) || child.contains_id(ui_id) {
                        child.set_background_texture(ui_id, texture);
                        return;
                    }
                }
            }
        }
    }

    pub fn next_event(&mut self) -> Option<EventFromUi> {
        self.event_queue.pop()
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            event_queue: Vec::new(),
            parent_size: Vector2f::new(0.0, 0.0),
            parent_position: Vector2f::new(0.0, 0.0),
            relative_size: Vector2f::new(1.0, 1.0),
            relative_position: Vector2f::new(0.0, 0.0),

            bg_color: Color::rgba(100, 100, 100, 0),

            widget: WidgetData {
                clickable: true,
                ..Default::default()
            },

            children: Vec::new(),
        }
    }
}

impl Drawable for Ui {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut sprite = Sprite::with_texture(self.widget.render_texture.texture());
        sprite.set_position(self.widget.real_position);
        target.draw_with_renderstates(&sprite, states);
    }
}
