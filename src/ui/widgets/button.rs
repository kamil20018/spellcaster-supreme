use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, Font, RenderStates, RenderTarget, Sprite, Text, Texture, Transformable},
    system::Vector2f,
};

use crate::ui::{event::UiEvent, traits::*, ui_id::UiId, widget::*};

pub struct Button {
    //actual user given stuff
    pub relative_size: Vector2f,
    pub relative_position: Vector2f,
    pub bg_color: Color,
    pub id: UiId,
    pub texture: Option<FBox<Texture>>,
    pub text: Option<String>,
    //calculated / processed later
    pub widget: WidgetData,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            relative_size: Vector2f::new(0.0, 0.0),
            relative_position: Vector2f::new(0.0, 0.0),
            bg_color: Color::rgb(100, 100, 100),
            id: UiId::new_none(),
            texture: None,
            text: None,
            widget: WidgetData {
                clickable: true,
                ..Default::default()
            },
        }
    }
}

impl UiElement for Button {}

impl CustomUi for Button {
    fn overwrite_relative(&mut self, relative_size: Vector2f, relative_position: Vector2f) {
        self.relative_size = relative_size;
        self.relative_position = relative_position;
    }

    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f) {
        eprintln!("parent_size = {:?}", parent_size);
        eprintln!("parent_position = {:?}", parent_position);
        self.widget
            .init(parent_size, parent_position, self.relative_size, self.relative_position);
        eprintln!("self.widget.real_size = {:?}", self.widget.real_size);
    }

    fn update(&mut self) {
        self.widget.render_texture.clear(self.bg_color);

        if let Some(texture) = &mut self.texture {
            let tex_size = texture.size();
            let mut sprite = Sprite::with_texture(texture);
            sprite.scale((
                self.widget.real_size.x / tex_size.x as f32,
                self.widget.real_size.y / tex_size.y as f32,
            ));
            self.widget.render_texture.draw(&sprite);
        }

        if let Some(button_text) = &self.text {
            let font = Font::from_file("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf").unwrap();
            let mut text = Text::new(button_text, &font, 15);
            text.set_position(self.widget.center_text(text.local_bounds()));
            self.widget.render_texture.draw(&text);
        }
        self.widget.render_texture.display();
    }

    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<UiEvent>> {
        if self.widget.was_clicked(click_pos) {
            return Some(vec![UiEvent::ButtonClicked(self.id)]);
        }
        None
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
