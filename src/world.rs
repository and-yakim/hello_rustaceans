use crate::qtree::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Item {
    pos: Vec2,
    pub rect: Rect,
}

impl Positioned for Item {
    fn pos(&self) -> Vec2 {
        self.pos
    }
}

impl Item {
    pub fn new(pos: Vec2) -> Self {
        Item {
            pos,
            rect: Rect::new(pos.x, pos.y, 0.0, 0.0),
        }
    }

    pub fn draw(&self, scale: f32) {
        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            4.0 / scale,
            BROWN,
        );
    }
}

impl From<Rect> for Item {
    fn from(rect: Rect) -> Self {
        Item {
            pos: rect.center(),
            rect,
        }
    }
}

pub struct RingBuffer3x3<T> {
    values: [[T; 3]; 3],
}

impl<T: Copy> RingBuffer3x3<T> {
    pub fn new(values: [[T; 3]; 3]) -> Self {
        Self { values }
    }

    pub fn center(&self) -> T {
        self.values[1][1]
    }

    pub fn shift_left(&mut self, new_col: [T; 3]) {
        for row in 0..3 {
            self.values[row][0] = self.values[row][1];
            self.values[row][1] = self.values[row][2];
            self.values[row][2] = new_col[row];
        }
    }

    pub fn shift_right(&mut self, new_col: [T; 3]) {
        for row in 0..3 {
            self.values[row][2] = self.values[row][1];
            self.values[row][1] = self.values[row][0];
            self.values[row][0] = new_col[row];
        }
    }

    pub fn shift_up(&mut self, new_row: [T; 3]) {
        self.values[0] = self.values[1];
        self.values[1] = self.values[2];
        self.values[2] = new_row;
    }

    pub fn shift_down(&mut self, new_row: [T; 3]) {
        self.values[2] = self.values[1];
        self.values[1] = self.values[0];
        self.values[0] = new_row;
    }
}
