use sfml::{
    graphics::{Color, Drawable, RenderStates, RenderTarget},
    system::Vector2f,
};

use crate::ui::{traits::*, widget::*};

pub struct Button {
    //actual user given stuff
    pub relative_size: Vector2f,
    pub relative_position: Vector2f,
    pub bg_color: Color,
    //calculated / processed later
    pub widget: WidgetData,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            relative_size: Vector2f::new(0.0, 0.0),
            relative_position: Vector2f::new(0.0, 0.0),
            bg_color: Color::rgb(100, 100, 100),

            widget: WidgetData {
                clickable: true,
                ..Default::default()
            },
        }
    }
}

impl UiElement for Button {}

impl CustomUi for Button {
    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f) {
        self.widget
            .init(parent_size, parent_position, self.relative_size, self.relative_position);
    }

    fn update(&mut self) {
        self.widget.render_texture.clear(self.bg_color);
    }

    fn on_click(&self, click_pos: Vector2f) {
        if self.widget.was_clicked(click_pos) {
            println!("button clicked");
        }
    }
}

impl Drawable for Button {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.widget.draw(target, states);
    }
}
