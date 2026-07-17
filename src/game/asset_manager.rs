#![allow(unused)]
use sfml::{cpp::FBox, graphics::Texture};

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

impl SpellComponentTypes {
    fn texture_path(self) -> &'static str {
        match self {
            SpellComponentTypes::ApplyTransform => "resources/spell_components/apply_transform.png",
            SpellComponentTypes::RuneGrass => "resources/spell_components/rune_grass.png",
            SpellComponentTypes::RuneRock => "resources/spell_components/rune_rock.png",
            SpellComponentTypes::RuneSelf => "resources/spell_components/rune_self.png",
            SpellComponentTypes::SpellArea1 => "resources/spell_components/spell_area_1.png",
            SpellComponentTypes::SpellArea2 => "resources/spell_components/spell_area_2.png",
            SpellComponentTypes::SpellArea3 => "resources/spell_components/spell_area_3.png",
            SpellComponentTypes::SpellEnd => "resources/spell_components/spell_end.png",
            SpellComponentTypes::SpellStartSingle => "resources/spell_components/spell_start_single.png",
        }
    }

    pub fn get_texture(self) -> FBox<Texture> {
        if let Ok(texture) = Texture::from_file(self.texture_path()) {
            return texture;
        }
        panic!("failed to load a texture at {}", self.texture_path());
    }
}
