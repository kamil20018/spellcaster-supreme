use hecs::{Entity, EntityBuilder};
use sfml::{
    cpp::FBox,
    graphics::{Drawable, RenderStates, RenderTarget, RenderTexture, Sprite, Transformable},
    system::Vector2f,
};

use crate::game::{
    component::{self, TilePosition, WorldPosition},
    draw,
    spawner::Spawner,
};

pub struct PlayField {
    pub render_texture: FBox<RenderTexture>,
    pub world: hecs::World,
    pub rng: rand::rngs::ThreadRng,
}

impl PlayField {
    pub fn init(&mut self) {
        let mut spawner = self.spawner();
        spawner.spawn_floor_tiles();
        spawner.spawn_nature();
    }

    pub fn update(&mut self) {
        draw::tiles(&mut self.render_texture, &mut self.world);
        draw::nature(&mut self.render_texture, &mut self.world);
    }

    pub fn transform<F, T>(&mut self, count: usize)
    where
        F: hecs::Component + component::Bundle,
        T: hecs::Component + component::Bundle,
    {
        let mut position_bundles = Vec::new();
        let mut del_entities = Vec::new();

        for (_, entity, tile_pos, world_pos) in self
            .world
            .query_mut::<(&F, Entity, &TilePosition, &WorldPosition)>()
            .into_iter()
            .take(count)
        {
            position_bundles.push((TilePosition::from(*tile_pos), WorldPosition::from(*world_pos)));

            del_entities.push(entity);
        }

        for entity in del_entities {
            if let Err(e) = self.world.despawn(entity) {
                println!("Somehow the entity found in the previous lines doesn't exist now: {e}");
            }
        }
        for position_bundle in position_bundles {
            self.world.spawn(
                EntityBuilder::new()
                    .add_bundle(position_bundle)
                    .add_bundle(T::get_bundle())
                    .build(),
            );
        }
    }

    fn spawner(&mut self) -> Spawner<'_> {
        Spawner {
            world: &mut self.world,
            rng: &mut self.rng,
        }
    }
}

impl Drawable for PlayField {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut sprite = Sprite::with_texture(self.render_texture.texture());
        sprite.set_position(Vector2f::new(0.0, 0.0));
        target.draw_with_renderstates(&sprite, states);
    }
}
