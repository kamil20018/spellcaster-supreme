use std::f32;

use sfml::{
    cpp::FBox,
    graphics::{
        Color, Drawable, Rect, RectangleShape, RenderStates, RenderTarget, RenderTexture, Shape, Sprite, Transformable,
    },
    system::{Vector2f, Vector2u},
};

use crate::game::style;

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
        let grid_size = self.render_texture.size();
        let cell_width = grid_size.x / self.grid_cols;
        let cell_height = grid_size.y / self.grid_rows;
        for row in 1..self.grid_rows {
            let mut row_line = RectangleShape::from_rect(Rect::<f32> {
                left: 0.0,
                top: cell_height as f32 * row as f32,
                width: grid_size.x as f32,
                height: 5.0,
            });
            row_line.set_fill_color(style::BACKGROUND_DARK_BLUE);
            self.render_texture.draw(&row_line);
        }
        for col in 1..self.grid_cols {
            let mut col_line = RectangleShape::from_rect(Rect::<f32> {
                left: cell_width as f32 * col as f32,
                top: 0.0,
                width: 5.0,
                height: grid_size.y as f32,
            });
            col_line.set_fill_color(Color::rgb(2, 9, 46));
            self.render_texture.draw(&col_line);
        }
        self.render_texture.display();
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
