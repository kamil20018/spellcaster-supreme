use sfml::{
    cpp::FBox,
    graphics::{FloatRect, RenderStates, RenderTarget, RenderTexture, Sprite, Transformable},
    system::Vector2f,
};

pub struct WidgetData {
    pub real_size: Vector2f,
    pub real_position: Vector2f,
    pub texture_position: Vector2f,
    pub render_texture: FBox<RenderTexture>,
    pub clickable: bool,
}

impl Default for WidgetData {
    fn default() -> Self {
        Self {
            real_size: Vector2f::new(0.0, 0.0),
            real_position: Vector2f::new(0.0, 0.0),
            texture_position: Vector2f::new(0.0, 0.0),
            render_texture: RenderTexture::new(1, 1).unwrap(),
            clickable: true,
        }
    }
}

impl WidgetData {
    pub fn init(
        &mut self,
        parent_size: Vector2f,
        parent_position: Vector2f,
        relative_size: Vector2f,
        relative_position: Vector2f,
    ) {
        self.real_size = Vector2f::new(parent_size.x * relative_size.x, parent_size.y * relative_size.y);
        self.real_position = Vector2f::new(
            parent_position.x + relative_position.x * parent_size.x,
            parent_position.y + relative_position.y * parent_size.y,
        );
        self.texture_position = Vector2f::new(
            parent_size.x * relative_position.x,
            // parent_size.y * (1.0 - relative_position.y - relative_size.y),
            parent_size.y * relative_position.y,
        );
        self.render_texture = RenderTexture::new(self.real_size.x as u32, self.real_size.y as u32).unwrap();
    }

    pub fn draw(&self, target: &mut dyn RenderTarget, states: &RenderStates) {
        let mut sprite = Sprite::with_texture(self.render_texture.texture());
        sprite.set_position(self.texture_position);
        target.draw_with_renderstates(&sprite, states);
    }

    pub fn center_text(&self, rect: FloatRect) -> Vector2f {
        Vector2f {
            x: (self.real_size.x - rect.width) / 2.0 - rect.left,
            y: (self.real_size.y - rect.height) / 2.0 - rect.top,
        }
    }

    pub fn was_clicked(&self, click_pos: Vector2f) -> bool {
        self.real_position.x < click_pos.x
            && click_pos.x < self.real_position.x + self.real_size.x
            && self.real_position.y < click_pos.y
            && click_pos.y < self.real_position.y + self.real_size.y
    }
}
