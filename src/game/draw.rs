use hecs::World;
use sfml::{
    cpp::FBox,
    graphics::{CircleShape, RenderTarget, RenderWindow, Shape, Sprite, Transformable},
    system::Vector2f,
};

use crate::game::{asset_manager::*, component::*, constant::*};

const CIRCLE_SCALE: f32 = 0.9;

pub fn tiles(window: &mut FBox<RenderWindow>, world: &mut World) {
    for (world_position, hexagon) in world.query_mut::<(&WorldPosition, &Hexagon)>() {
        let mut circle = CircleShape::new(TILE_RADIUS * CIRCLE_SCALE, 6);
        circle.set_fill_color(hexagon.color);
        circle.set_position(world_position);
        circle.set_origin(Vector2f::new(TILE_RADIUS * CIRCLE_SCALE, TILE_RADIUS * CIRCLE_SCALE));
        circle.set_rotation(30.);
        window.draw(&circle);
    }
}

pub fn textures(window: &mut FBox<RenderWindow>, asset_manager: &AssetManager) {
    if let Some(circle) = asset_manager.get(&SpellComponentTypes::SpellStartSingle) {
        let mut sprite = Sprite::with_texture(circle);
        sprite.set_position(Vector2f::new(150.0, 150.0));
        window.draw(&sprite);
    }
    if let Some(circle) = asset_manager.get(&SpellComponentTypes::RuneSelf) {
        let mut sprite = Sprite::with_texture(circle);
        sprite.set_position(Vector2f::new(250.0, 250.0));
        window.draw(&sprite);
    }
}

pub fn nature(window: &mut FBox<RenderWindow>, world: &mut World) {
    for (world_position, circle) in world.query_mut::<(&WorldPosition, &Circle)>() {
        let mut circle_shape = CircleShape::new(circle.radius, 16);
        circle_shape.set_fill_color(circle.color);
        circle_shape.set_position(world_position);
        circle_shape.set_origin(Vector2f::new(circle.radius, circle.radius));
        window.draw(&circle_shape);
    }
}
