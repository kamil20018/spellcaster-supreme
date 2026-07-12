pub const TILE_RADIUS: f32 = 30.0;
pub const SQRT_3: f32 = 1.7320508;
pub const SCREEN_W: u32 = 1920;
pub const SCREEN_H: u32 = 1080;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum SpellComponentTypes {
    ApplyTransform,
    RuneGrass,
    RuneRock,
    RuneSelf,
    SpellArea1,
    SpellArea2,
    SpellArea3,
    SpellEnd,
    SpellStartSingle,
}
