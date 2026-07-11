use crate::game::asset_manager::*;
use crate::game::component::*;
use crate::game::constant::*;
use hecs::World;
use raylib::prelude::*;

pub fn tiles(d: &mut RaylibDrawHandle, world: &mut World) {
    for (world_position, hexagon) in world.query_mut::<(&WorldPosition, &Hexagon)>() {
        if let Some(color) = hexagon.color {
            d.draw_poly(world_position, 6, TILE_RADIUS * 0.95, 0.0, color);
        }
    }
}

pub fn textures(d: &mut RaylibDrawHandle, asset_manager: &AssetManager) {
    if let Some(circle) = asset_manager.get(&SpellComponents::Circle) {
        d.draw_texture(circle, 150, 150, Color::WHITE);
    }
    if let Some(circle) = asset_manager.get(&SpellComponents::ManaInput) {
        d.draw_texture(circle, 250, 250, Color::WHITE);
    }
}

pub fn nature(d: &mut RaylibDrawHandle, world: &mut World) {
    for (world_position, circle) in world.query_mut::<(&WorldPosition, &Circle)>() {
        d.draw_circle(
            world_position.x as i32,
            world_position.y as i32,
            circle.radius,
            circle.color,
        );
    }
}
