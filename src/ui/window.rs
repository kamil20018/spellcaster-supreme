use sfml::{
    graphics::{Color, Drawable, RenderStates, RenderTarget, Sprite, Transformable},
    system::Vector2f,
};

use crate::ui::{traits::*, widget::*};

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
        println!("post widget init");

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

    pub fn on_click(&self, click_pos: Vector2f) {
        if self.widget.was_clicked(click_pos) {
            if self.widget.clickable {
                println!("window clicked");
                //todo: add callback when implemented
            } else {
                println!("window not clickable");
            }
            for child in &self.children {
                child.on_click(click_pos);
            }
        }
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
