use hecs::{EntityBuilder, World};
use rand::RngExt;
use sfml::{graphics::Color, system::Vector2u};

use crate::game::{component::*, constant::*};
pub struct Spawner<'a> {
    pub world: &'a mut World,
    pub rng: &'a mut rand::rngs::ThreadRng,
    pub bounds: Vector2u,
}

impl Spawner<'_> {
    pub fn spawn_floor_tiles(&mut self) -> &mut Self {
        for x in 0..=45 {
            for y in -20..=20 {
                let tile_pos = &TilePosition::new(x, y);
                if self.tile_in_bounds(tile_pos) {
                    self.world.spawn((
                        Hexagon { color: Color::MAGENTA },
                        TilePosition::new(x, y),
                        WorldPosition::from(*tile_pos),
                        HexTile {},
                    ));
                }
            }
        }

        self
    }

    fn tile_in_bounds(&self, tile_pos: &TilePosition) -> bool {
        let global_pos = WorldPosition::from(*tile_pos);
        let left = global_pos.x - TILE_RADIUS;
        let right = global_pos.x + TILE_RADIUS;
        let top = global_pos.y - TILE_RADIUS;
        let bottom = global_pos.y + TILE_RADIUS;

        left >= 0.0 && right <= self.bounds.x as f32 && top >= 0.0 && bottom <= self.bounds.y as f32
    }

    pub fn spawn_nature(&mut self) -> &mut Self {
        let mut grass_positions = Vec::new();
        let mut rock_positions = Vec::new();
        for (_, tile_pos, world_pos) in self.world.query_mut::<(&HexTile, &TilePosition, &WorldPosition)>() {
            match self.rng.random::<f32>() {
                x if x < 0.1 => {
                    grass_positions.push((TilePosition::from(*tile_pos), WorldPosition::from(*world_pos)));
                }
                x if x < 0.2 => {
                    rock_positions.push((TilePosition::from(*tile_pos), WorldPosition::from(*world_pos)));
                }
                _ => {}
            };
        }

        for position_bundle in rock_positions {
            let mut builder = EntityBuilder::new();
            builder.add_bundle(Rock::get_bundle()).add_bundle(position_bundle);
            self.world.spawn(builder.build());
        }

        for position_bundle in grass_positions {
            let mut builder = EntityBuilder::new();
            builder.add_bundle(Grass::get_bundle()).add_bundle(position_bundle);
            self.world.spawn(builder.build());
        }

        self
    }
}
