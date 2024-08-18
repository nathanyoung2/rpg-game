use macroquad::prelude::*;

pub struct EntityImageParams<'a> {
    pub texture: &'a Option<Texture2D>,
    pub x: f32,
    pub y: f32,
}

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

    pub fn update(
        &self,
        health: u32,
        max_health: u32,
        image_params: EntityImageParams,
        name: &str,
    ) {
        let bar_y = self.ypos + 6.0;
        let health_bar_params = DrawTextureParams {
            source: Some(Rect {
                x: 0.0,
                y: 0.0,
                w: self.health_bar_texture.width() * (health as f32 / max_health as f32),
                h: self.health_bar_texture.height(),
            }),
            ..Default::default()
        };

        draw_text(
            format!("{}  {}/{}", name, health, max_health).as_str(),
            self.xpos,
            self.ypos,
            35.0,
            WHITE,
        );

        if let Some(ref texture) = image_params.texture {
            draw_texture(texture, image_params.x, image_params.y, WHITE);
        }

        draw_texture_ex(
            self.health_bar_texture,
            self.xpos,
            bar_y,
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
