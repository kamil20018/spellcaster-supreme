use sfml::{
    cpp::FBox,
    graphics::{Color, Drawable, RenderStates, RenderTarget, RenderTexture, Sprite, Transformable},
    system::{Vector2f, Vector2u},
};

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

pub trait UiElement: CustomUi + Drawable {}

pub struct Window {
    pub parent_size: Vector2f,
    pub relative_size: Vector2f,
    pub parent_position: Vector2f,
    pub relative_position: Vector2f,
    pub bg_color: Color,
    pub render_texture: FBox<RenderTexture>,

    pub children: Vec<Box<dyn UiElement>>,
}

impl Window {
    fn init(&mut self) {
        // let test = self.parent_size * self.relative_size;
        let texture_size = Vector2f::new(
            self.parent_size.x * self.relative_size.x,
            self.parent_size.y * self.relative_size.y,
        );
        self.render_texture = RenderTexture::new(texture_size.x as u32, texture_size.y as u32).unwrap();
        let pos = self.get_real_position();
        for child in &mut self.children {
            child.init(&texture_size, &pos);
        }
    }

    fn update(&mut self) {
        self.render_texture.clear(self.bg_color);
        for child in &mut self.children {
            child.update();
        }

        for child in &self.children {
            self.render_texture.draw(child.as_ref());
        }
    }

    fn get_real_position(&self) -> Vector2f {
        Vector2f {
            x: self.parent_position.x + self.relative_position.x * self.parent_size.x,
            y: self.parent_position.y + self.relative_position.y * self.parent_size.y,
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            parent_size: Vector2f::new(0.0, 0.0),
            relative_size: Vector2f::new(0.0, 0.0),
            parent_position: Vector2f::new(0.0, 0.0),
            relative_position: Vector2f::new(0.0, 0.0),
            bg_color: Color::rgb(100, 100, 100),
            render_texture: RenderTexture::new(1, 1).unwrap(),

            children: Vec::new(),
        }
    }
}

impl Drawable for Window {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut sprite = Sprite::with_texture(self.render_texture.texture());
        sprite.set_position(self.get_real_position());
        target.draw_with_renderstates(&sprite, states);
    }
}

pub trait CustomUi {
    fn init(&mut self, parent_size: &Vector2f, parent_position: &Vector2f);
    fn update(&mut self);
    fn get_real_position(&self, parent_size: &Vector2f, parent_position: &Vector2f) -> Vector2f;
}

pub struct Button {
    //actual user given stuff
    pub relative_size: Vector2f,
    pub relative_position: Vector2f,
    pub bg_color: Color,
    //calculated / processed later
    pub real_size: Vector2f,
    pub real_position: Vector2f,
    pub texture_position: Vector2f,
    pub render_texture: FBox<RenderTexture>,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            relative_size: Vector2f::new(0.0, 0.0),
            relative_position: Vector2f::new(0.0, 0.0),
            bg_color: Color::rgb(100, 100, 100),

            real_size: Vector2f::new(0.0, 0.0),
            real_position: Vector2f::new(0.0, 0.0),
            texture_position: Vector2f::new(0.0, 0.0),
            render_texture: RenderTexture::new(1, 1).unwrap(),
        }
    }
}

impl UiElement for Button {}

impl CustomUi for Button {
    fn init(&mut self, parent_size: &Vector2f, parent_position: &Vector2f) {
        self.real_size = Vector2f::new(
            parent_size.x * self.relative_size.x,
            parent_size.y * self.relative_size.y,
        );
        self.real_position = self.get_real_position(parent_size, parent_position);
        self.render_texture = RenderTexture::new(self.real_size.x as u32, self.real_size.y as u32).unwrap();
        self.texture_position = Vector2f::new(
            parent_size.x * self.relative_position.x,
            parent_size.y * self.relative_position.y,
        );
    }

    fn update(&mut self) {
        self.render_texture.clear(self.bg_color);
    }

    fn get_real_position(&self, parent_size: &Vector2f, parent_position: &Vector2f) -> Vector2f {
        Vector2f {
            x: parent_position.x + self.relative_position.x * parent_size.x,
            y: parent_position.y + self.relative_position.y * parent_size.y,
        }
    }
}

impl Drawable for Button {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut sprite = Sprite::with_texture(self.render_texture.texture());

        sprite.set_position(self.texture_position);
        target.draw_with_renderstates(&sprite, states);
    }
}
