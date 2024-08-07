use std::collections::VecDeque;

pub struct TextQueue {
    texts: VecDeque<String>,
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
        self.texts.push_back(String::from(s));
    }

    pub fn get_current(&mut self) -> Option<String> {
        Some(self.texts.pop_front()?)
    }
}
