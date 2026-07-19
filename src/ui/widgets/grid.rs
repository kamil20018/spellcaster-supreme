use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, RenderStates, RenderTarget, Texture},
    system::{Vector2f, Vector2i},
};

use crate::ui::{
    event::EventFromUi,
    padding::RelativePadding,
    traits::*,
    ui_id::{self, UiId},
    widget::*,
};

pub struct Grid {
    //actual user given stuff
    pub relative_size: Vector2f,
    pub relative_position: Vector2f,
    pub bg_color: Color,
    pub id: ui_id::UiId,
    pub grid_size: Vector2i,
    pub padding: RelativePadding,

    pub children: Vec<Box<dyn UiElement>>,
    //calculated / processed later
    pub widget: WidgetData,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            relative_size: Vector2f::new(0.0, 0.0),
            relative_position: Vector2f::new(0.0, 0.0),
            bg_color: Color::rgb(100, 100, 100),
            id: UiId::new_none(),
            grid_size: Vector2i::new(2, 2),
            padding: RelativePadding { ..Default::default() },
            children: Vec::new(),
            widget: WidgetData {
                clickable: true,
                ..Default::default()
            },
        }
    }
}

impl UiElement for Grid {}

impl CustomUi for Grid {
    fn init(&mut self, parent_size: Vector2f, parent_position: Vector2f) {
        self.widget
            .init(parent_size, parent_position, self.relative_size, self.relative_position);

        for row in 0..self.grid_size.y {
            for col in 0..self.grid_size.x {
                let idx = row * self.grid_size.x + col;
                if idx < self.children.len() as i32 {
                    let child_relative_size = Vector2f::new(
                        (1.0 - self.padding.left
                            - self.padding.right
                            - self.padding.columns * (self.grid_size.x - 1) as f32)
                            / self.grid_size.x as f32,
                        (1.0 - self.padding.top
                            - self.padding.botton
                            - self.padding.rows * (self.grid_size.y - 1) as f32)
                            / self.grid_size.y as f32,
                    );
                    let child_relative_position = Vector2f::new(
                        self.padding.left + col as f32 * (self.padding.columns + child_relative_size.x),
                        self.padding.top + row as f32 * (self.padding.rows + child_relative_size.y),
                    );
                    self.children[idx as usize].overwrite_relative(child_relative_size, child_relative_position);
                    self.children[idx as usize].init(self.widget.real_size, self.widget.real_position);
                }
            }
        }
    }

    fn update(&mut self) {
        self.widget.render_texture.clear(self.bg_color);

        for child in &mut self.children {
            child.update();
        }

        for child in &self.children {
            self.widget.render_texture.draw(child.as_ref());
        }

        self.widget.render_texture.display();
    }

    fn on_click(&self, click_pos: Vector2f) -> Option<Vec<EventFromUi>> {
        if self.widget.was_clicked(click_pos) && self.widget.clickable {
            let mut events: Vec<EventFromUi> = Vec::new();
            for child in &self.children {
                if let Some(child_events) = child.on_click(click_pos) {
                    events.extend(child_events);
                }
            }
            if events.len() > 0 {
                return Some(events);
            }
        }
        None
    }

    fn is_id(&self, id: UiId) -> bool {
        self.id == id
    }

    fn contains_id(&self, _id: UiId) -> bool {
        for child in &self.children {
            if child.is_id(_id) {
                return true;
            }
        }
        return false;
    }

    fn set_background_texture(&mut self, id: UiId, texture: FBox<Texture>) {
        for child in &mut self.children {
            if child.is_id(id) || child.contains_id(id) {
                child.set_background_texture(id, texture);
                return;
            }
        }
    }
}

impl Drawable for Grid {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.widget.draw(target, states);
    }
}
