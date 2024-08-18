use macroquad::prelude::*;

pub struct EntityStats<'a> {
    health_bar_texture: &'a Texture2D,
    xpos: f32,
    ypos: f32,
}

impl<'a> EntityStats<'a> {
    pub fn new(xpos: f32, ypos: f32, health_bar_texture: &'a Texture2D) -> Self {
        Self {
            health_bar_texture,
            xpos,
            ypos,
        }
    }

    pub fn update(&self, health: u32, max_health: u32, name: &str) {
        let bar_y = self.ypos - 10.0;
        let health_bar_params = DrawTextureParams {
            source: Some(Rect {
                x: self.xpos,
                y: bar_y,
                w: self.health_bar_texture.width() * (health as f32 / max_health as f32),
                h: self.health_bar_texture.height(),
            }),
            ..Default::default()
        };

        draw_text(name, self.xpos, self.ypos, 20.0, WHITE);

        draw_texture_ex(
            self.health_bar_texture,
            self.xpos,
            self.ypos,
            WHITE,
            health_bar_params,
        );
    }
}

pub struct ButtonLink<'a, T> {
    pub link: T,
    pub button: Button<'a>,
}

pub struct Button<'a> {
    texture: &'a Texture2D,
    pub xpos: f32,
    pub ypos: f32,
    width: f32,
    height: f32,
    hover_texture: Option<&'a Texture2D>,
    click_texture: Option<&'a Texture2D>,
    mouse_down: bool,
}

impl<'a> Button<'a> {
    pub fn new(texture: &'a Texture2D, xpos: f32, ypos: f32) -> Self {
        Self {
            texture,
            xpos,
            ypos,
            width: texture.width(),
            height: texture.height(),
            hover_texture: None,
            click_texture: None,
            mouse_down: false,
        }
    }

    pub fn attach_hover_texture(&mut self, texture: &'a Texture2D) {
        self.hover_texture = Some(texture);
    }

    pub fn attach_click_texture(&mut self, texture: &'a Texture2D) {
        self.click_texture = Some(texture);
    }

    pub fn clicked(&mut self) -> bool {
        if self.hovered() {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.mouse_down = true;
                return true;
            }
        }
        false
    }

    fn hovered(&self) -> bool {
        let x = mouse_position().0;
        let y = mouse_position().1;
        if x > self.xpos && x < self.xpos + self.width {
            if y > self.ypos && y < self.ypos + self.height {
                return true;
            }
        }
        false
    }

    pub fn draw(&mut self) {
        let mut texture = self.texture;

        if self.mouse_down {
            texture = match self.click_texture {
                Some(texture) => texture,
                None => self.texture,
            };
        }

        if is_mouse_button_released(MouseButton::Left) {
            texture = self.texture;
            self.mouse_down = true;
        }

        if self.hovered() {
            texture = match self.hover_texture {
                Some(texture) => texture,
                None => self.texture,
            };
        }

        draw_texture(texture, self.xpos, self.ypos, WHITE);
    }
}
