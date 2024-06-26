use ggez::graphics::Text;

pub struct TextQueue {
    texts: Vec<Text>,
}

impl TextQueue {
    pub fn add(&mut self, s: &str) {
        self.texts.append(s);
    }
}
