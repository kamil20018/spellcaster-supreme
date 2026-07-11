use raylib::prelude::*;
use std::collections::HashMap;

use crate::game::constant::*;

pub struct AssetManager {
    spell_textures: HashMap<SpellComponents, Texture2D>,
}

impl AssetManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut textures: HashMap<SpellComponents, Texture2D> = HashMap::new();
        Self::populate(rl, thread, &mut textures);
        Self {
            spell_textures: textures,
        }
    }

    fn populate(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        textures: &mut HashMap<SpellComponents, Texture2D>,
    ) {
        let mut add_texture = |file_path: &str, component_type: SpellComponents| {
            let image = raylib::core::texture::Image::load_image(file_path)
                .unwrap_or_else(|_| panic!("Failed to load image: {file_path}"));

            let texture = rl
                .load_texture_from_image(thread, &image)
                .unwrap_or_else(|_| panic!("Failed to create texture from image at: {file_path}"));
            textures.insert(component_type, texture);
        };
        const SPELL_COMPONENT_TEXTURES: &[(&str, SpellComponents)] = &[
            (
                "resources/spell_components/apply_transform.png",
                SpellComponents::ApplyTransform,
            ),
            (
                "resources/spell_components/rune_grass.png",
                SpellComponents::RuneGrass,
            ),
            (
                "resources/spell_components/rune_rock.png",
                SpellComponents::RuneRock,
            ),
            (
                "resources/spell_components/rune_self.png",
                SpellComponents::RuneSelf,
            ),
            (
                "resources/spell_components/spell_area_1.png",
                SpellComponents::SpellArea1,
            ),
            (
                "resources/spell_components/spell_area_2.png",
                SpellComponents::SpellArea2,
            ),
            (
                "resources/spell_components/spell_area_3.png",
                SpellComponents::SpellArea3,
            ),
            (
                "resources/spell_components/spell_end.png",
                SpellComponents::SpellEnd,
            ),
            (
                "resources/spell_components/spell_start_single.png",
                SpellComponents::SpellStartSingle,
            ),
        ];

        for &(path, component_type) in SPELL_COMPONENT_TEXTURES {
            add_texture(path, component_type);
        }
    }

    #[allow(unused)]
    fn insert(&mut self, id: SpellComponents, texture: Texture2D) {
        self.spell_textures.insert(id, texture);
    }

    pub fn get(&self, id: &SpellComponents) -> Option<&Texture2D> {
        self.spell_textures.get(id)
    }
}
