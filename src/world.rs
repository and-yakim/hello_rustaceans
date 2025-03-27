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
