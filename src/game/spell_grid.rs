use sfml::system::Vector2i;

use crate::game::asset_manager::SpellComponentTypes;
pub struct SpellGrid<const W: usize, const H: usize> {
    grid: [[Option<SpellComponentTypes>; W]; H],
}

impl<const W: usize, const H: usize> SpellGrid<W, H> {
    pub fn new() -> Self {
        Self { grid: [[None; W]; H] }
    }

    pub fn set_component(&mut self, position: Vector2i, component: SpellComponentTypes) {
        self.grid[position.y as usize][position.x as usize] = Some(component);
    }

    #[allow(unused)]
    pub fn print_self(&self) {
        let mut text: String = "".to_string();
        for row in &self.grid {
            for cell in row {
                match cell {
                    Some(_) => text.push_str("S"),
                    None => text.push_str("N"),
                }
            }
            text.push_str("\n");
        }
        print!("{text}");
    }
}
