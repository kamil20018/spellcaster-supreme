mod game;
use game::Game;

struct Number {
    value: i64,
}

fn main() {
    let mut game = Game::new();

    game.run();
}

// use hecs::World;
// use hecs::Entity;
// #[derive(Debug)]
// struct Position {
//     x: i32,
//     y: i32,
// }

// enum SpellType {
//     A,
//     B,
//     C,
// }

// let mut world = World::new();

// let ent1 = world.spawn((Position { x: 32, y: 16 }, SpellType::A));
// let ent2 = world.spawn((Position { x: 16, y: 32 }, SpellType::B));
// let ent3 = world.spawn((Position { x: 16, y: 16 }, SpellType::C));

// for (entity, position) in world.query_mut::<(Entity, &mut Position)>() {
//     position.x += 1;
//     println!("{:?}: ({}, {})", position, position.x, position.y);
// }
