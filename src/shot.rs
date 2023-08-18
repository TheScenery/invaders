use crate::frame::{Drawable, Frame};

pub struct Shot {
    x: usize,
    y: usize,
    exploding: bool,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
        }
    }

    pub fn update(&mut self) {
        if !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
    }

    pub fn dead(&self) -> bool {
        self.y <= 0
    }
}

impl Drawable for Shot{
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding {"*"} else { "|" };
    }
}