use ggez::graphics::Text;
use std::collections::VecDeque;

pub struct TextQueue {
    texts: VecDeque<Text>,
    pub ready: bool,
}

impl TextQueue {
    pub fn new() -> Self {
        TextQueue {
            texts: VecDeque::new(),
            ready: false,
        }
    }

    pub fn add(&mut self, s: &str) {
        self.texts.push_back(Text::new(s));
    }

    pub fn get_current(&mut self) -> Option<Text> {
        Some(self.texts.pop_front()?)
    }
}
