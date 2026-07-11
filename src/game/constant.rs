pub const TILE_RADIUS: f32 = 30.0;
pub const SQRT_3: f32 = 1.7320508;
pub const SCREEN_W: i32 = 1920;
pub const SCREEN_H: i32 = 1080;

#[derive(Hash, Eq, PartialEq)]
pub enum SpellComponents {
    Circle,
    ManaInput,
}
