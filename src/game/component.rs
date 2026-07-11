use hecs::DynamicBundle;
use raylib::{ffi::CSSPalette, prelude::*};

pub struct Hexagon {
    pub color: Option<Color>,
    pub texture: Option<Texture2D>,
}

pub struct Circle {
    pub color: Color,
    pub radius: f32,
}

pub trait Bundle {
    fn get_bundle() -> impl DynamicBundle;
}

pub struct HexTile {}
pub struct Rock {}

impl Bundle for Rock {
    fn get_bundle() -> impl DynamicBundle {
        (
            Rock {},
            Circle {
                color: Color::GRAY,
                radius: 10.0,
            },
        )
    }
}

pub struct Grass {}

impl Bundle for Grass {
    fn get_bundle() -> impl DynamicBundle {
        (
            Grass {},
            Circle {
                color: Color::GREEN,
                radius: 10.0,
            },
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}
impl TilePosition {
    pub fn new(x: i32, y: i32) -> Self {
        TilePosition { x: x, y: y }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
}

impl WorldPosition {
    pub fn new(x: f32, y: f32) -> Self {
        WorldPosition { x: x, y: y }
    }
}

impl From<Vector2> for WorldPosition {
    fn from(v: Vector2) -> Self {
        WorldPosition::new(v.x, v.y)
    }
}
