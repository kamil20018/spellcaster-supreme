use sfml::{
    graphics::{Color, Drawable, RenderStates, RenderTarget, Sprite, Transformable},
    system::Vector2f,
};

use crate::ui::{event::UiEvent, traits::*, widget::*};

pub struct Window {
    pub widget: WidgetData,

    pub parent_size: Vector2f,
    pub parent_position: Vector2f,
    pub relative_size: Vector2f,
    pub relative_position: Vector2f,
    pub bg_color: Color,

    pub children: Vec<Box<dyn UiElement>>,
}

impl Window {
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

    pub fn on_click(&self, click_pos: Vector2f) -> Option<Vec<UiEvent>> {
        if self.widget.was_clicked(click_pos) {
            let mut events: Vec<UiEvent> = Vec::new();
            for child in &self.children {
                if let Some(child_events) = child.on_click(click_pos) {
                    events.extend(child_events);
                }
            }
            if events.len() > 0 {
                return Some(events);
            }
        }
        None
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            parent_size: Vector2f::new(0.0, 0.0),
            parent_position: Vector2f::new(0.0, 0.0),
            relative_size: Vector2f::new(0.0, 0.0),
            relative_position: Vector2f::new(0.0, 0.0),

            bg_color: Color::rgb(100, 100, 100),

            widget: WidgetData {
                clickable: true,
                ..Default::default()
            },

            children: Vec::new(),
        }
    }
}

impl Drawable for Window {
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
