use sfml::{
    graphics::{Drawable, RenderStates, RenderTarget},
    system::Vector2f,
};

pub mod event;
pub mod helpers;
pub mod traits;
pub mod ui_id;
pub mod widget;
pub mod widgets;
pub mod window;
pub use event::UiEvent;
pub use window::Window;

pub struct Ui {
    pub windows: Vec<Window>,
    pub event_queue: Vec<UiEvent>,
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

    pub fn on_click(&mut self, click_pos: Vector2f) {
        for window in &self.windows {
            if let Some(events) = window.on_click(click_pos) {
                self.event_queue.extend(events);
            }
        }
    }

    pub fn next_event(&mut self) -> Option<UiEvent> {
        self.event_queue.pop()
    }
}

impl Default for Ui {
    fn default() -> Self {
        Ui {
            windows: Vec::new(),
            event_queue: Vec::new(),
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
