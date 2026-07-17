use hecs::{Entity, EntityBuilder, World};
use sfml::{
    cpp::FBox,
    graphics::{CircleShape, Drawable, RenderStates, RenderTarget, RenderTexture, Shape, Sprite, Transformable},
    system::{Vector2f, Vector2u},
};

use crate::game::{
    component::{self, Circle, Hexagon, TilePosition, WorldPosition},
    constant::*,
    spawner::Spawner,
};

pub struct PlayField {
    render_texture: FBox<RenderTexture>,
    world: hecs::World,
    rng: rand::rngs::ThreadRng,
}

impl PlayField {
    pub fn new(texture_size: Vector2u) -> Self {
        PlayField {
            render_texture: RenderTexture::new(texture_size.x, texture_size.y).unwrap(),
            world: World::new(),
            rng: rand::rng(),
        }
    }
    pub fn init(&mut self) {
        let mut spawner = self.spawner();
        spawner.spawn_floor_tiles();
        spawner.spawn_nature();
    }

    pub fn update(&mut self) {
        // draw::tiles(&mut self.render_texture, &mut self.world);
        for (world_position, hexagon) in self.world.query_mut::<(&WorldPosition, &Hexagon)>() {
            let mut circle = CircleShape::new(TILE_RADIUS * CIRCLE_SCALE, 6);
            circle.set_fill_color(hexagon.color);
            circle.set_position(world_position);
            circle.set_origin(Vector2f::new(TILE_RADIUS * CIRCLE_SCALE, TILE_RADIUS * CIRCLE_SCALE));
            circle.set_rotation(30.);
            self.render_texture.draw(&circle);
        }
        // draw::nature(&mut self.render_texture, &mut self.world);
        for (world_position, circle) in self.world.query_mut::<(&WorldPosition, &Circle)>() {
            let mut circle_shape = CircleShape::new(circle.radius, 16);
            circle_shape.set_fill_color(circle.color);
            circle_shape.set_position(world_position);
            circle_shape.set_origin(Vector2f::new(circle.radius, circle.radius));
            self.render_texture.draw(&circle_shape);
        }
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
            bounds: self.render_texture.size(),
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
