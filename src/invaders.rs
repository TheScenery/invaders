use std::cmp::max;
use std::time::Duration;
use rusty_time::Timer;
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};

struct Invader {
    x: usize,
    y: usize,
}

pub struct Invaders {
    army: Vec<Invader>,
    direction: i32,
    timer: Timer,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1) && (x < NUM_COLS - 2) && (y > 0) && (y < 9) && (x % 2 == 0) && (y % 2 == 0) {
                    army.push(Invader { x, y })
                }
            }
        }
        Self {
            army,
            direction: 1,
            timer: Timer::from_millis(2000)
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.timer.update(delta);
        if self.timer.ready {
            self.timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true
                }
            }
            if downwards {
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
                self.timer = Timer::from_millis(max(self.timer.duration.as_millis() - 200, 200) as u64)
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self.army.iter().position(|invader| invader.x == x && invader.y == y) {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = "X"
        }
    }
}