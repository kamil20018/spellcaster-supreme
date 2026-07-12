use std::collections::HashMap;

use sfml::{cpp::FBox, graphics::Texture};

use crate::game::constant::*;

pub struct AssetManager {
    spell_components: HashMap<SpellComponentTypes, FBox<Texture>>,
}

impl AssetManager {
    pub fn new() -> Self {
        let mut textures: HashMap<SpellComponentTypes, FBox<Texture>> = HashMap::new();
        Self::populate(&mut textures);
        Self {
            spell_components: textures,
        }
    }

    fn populate(textures: &mut HashMap<SpellComponentTypes, FBox<Texture>>) {
        let mut add_texture = |file_path: &str, component_type: SpellComponentTypes| {
            let texture = Texture::from_file(file_path).unwrap_or_else(|_| panic!("Failed to load image: {file_path}"));

            textures.insert(component_type, texture);
        };
        const SPELL_COMPONENT_TEXTURES: &[(&str, SpellComponentTypes)] = &[
            (
                "resources/spell_components/apply_transform.png",
                SpellComponentTypes::ApplyTransform,
            ),
            (
                "resources/spell_components/rune_grass.png",
                SpellComponentTypes::RuneGrass,
            ),
            (
                "resources/spell_components/rune_rock.png",
                SpellComponentTypes::RuneRock,
            ),
            (
                "resources/spell_components/rune_self.png",
                SpellComponentTypes::RuneSelf,
            ),
            (
                "resources/spell_components/spell_area_1.png",
                SpellComponentTypes::SpellArea1,
            ),
            (
                "resources/spell_components/spell_area_2.png",
                SpellComponentTypes::SpellArea2,
            ),
            (
                "resources/spell_components/spell_area_3.png",
                SpellComponentTypes::SpellArea3,
            ),
            (
                "resources/spell_components/spell_end.png",
                SpellComponentTypes::SpellEnd,
            ),
            (
                "resources/spell_components/spell_start_single.png",
                SpellComponentTypes::SpellStartSingle,
            ),
        ];

        for &(path, component_type) in SPELL_COMPONENT_TEXTURES {
            add_texture(path, component_type);
        }
    }

    pub fn get(&self, id: &SpellComponentTypes) -> Option<&FBox<Texture>> {
        self.spell_components.get(id)
    }
}
