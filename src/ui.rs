use sfml::{
    graphics::{Drawable, RenderStates, RenderTarget},
    system::Vector2f,
};

pub mod traits;
pub mod widget;
pub mod widgets;
pub mod window;

pub use widgets::Button;
pub use window::Window;

pub struct Ui {
    pub windows: Vec<Window>,
}

impl Ui {
    pub fn init(&mut self) {
        for window in &mut self.windows {
            window.init();
        }
    }

    pub fn update(&mut self) {
        for window in &mut self.windows {
            window.update();
        }
    }

    pub fn on_click(&self, click_pos: Vector2f) {
        for window in &self.windows {
            window.on_click(click_pos);
        }
    }
}

impl Drawable for Ui {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for window in &self.windows {
            target.draw_with_renderstates(window, states);
            // or: target.draw(window);
        }
    }
}
