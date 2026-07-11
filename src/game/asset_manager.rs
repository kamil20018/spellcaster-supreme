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

        add_texture(
            "resources/spell_components/circle.png",
            SpellComponents::Circle,
        );
        add_texture(
            "resources/spell_components/mana_input.png",
            SpellComponents::ManaInput,
        );
    }

    #[allow(unused)]
    fn insert(&mut self, id: SpellComponents, texture: Texture2D) {
        self.spell_textures.insert(id, texture);
    }

    pub fn get(&self, id: &SpellComponents) -> Option<&Texture2D> {
        self.spell_textures.get(id)
    }
}
