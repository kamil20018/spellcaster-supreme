use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, RenderStates, RenderTarget, RenderTexture, Sprite, Transformable},
    system::{Vector2f, Vector2u},
};

pub struct SpellCreator {
    render_texture: FBox<RenderTexture>,
    sprite_position: Vector2f,
    grid_rows: u32,
    grid_cols: u32,
}

impl SpellCreator {
    pub fn new(texture_size: Vector2u, sprite_position: Vector2f) -> Self {
        SpellCreator {
            render_texture: RenderTexture::new(texture_size.x, texture_size.y).unwrap(),
            sprite_position: sprite_position,
            grid_rows: 11,
            grid_cols: 11,
        }
    }

    pub fn update(&mut self) {
        self.render_texture.clear(Color::WHITE);
    }
}

impl Drawable for SpellCreator {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut sprite = Sprite::with_texture(self.render_texture.texture());
        sprite.set_position(self.sprite_position);
        target.draw_with_renderstates(&sprite, states);
    }
}
