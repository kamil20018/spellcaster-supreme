use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, RenderStates, RenderTarget, RenderTexture, Sprite},
};

use crate::game::constant::*;

// pub trait SpellComponent {}

// pub struct Grid {}

pub struct SpellCreator {
    render_texture: FBox<RenderTexture>,
}

impl SpellCreator {
    pub fn new() -> Self {
        let mut render_texture = RenderTexture::new(SCREEN_W / 4 * 3, SCREEN_H / 4 * 3).unwrap();
        render_texture.clear(Color::RED);
        SpellCreator {
            render_texture: render_texture,
        }
    }
}

impl Drawable for SpellCreator {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let sprite = Sprite::with_texture(self.render_texture.texture());
        target.draw_with_renderstates(&sprite, states);
    }
}
