use crate::qtree::*;
use macroquad::prelude::*;

impl<T: Clone + Positioned> QTreeMut<T> {
    pub fn draw(&self, scale: f32) {
        match self {
            QTreeMut::Node { children, .. } => {
                for node in children.iter() {
                    node.draw(scale);
                }
            }
            QTreeMut::Leaf { region, .. } => {
                draw_rectangle_lines(region.x, region.y, region.w, region.h, 2.0 / scale, GREEN);
            }
        }
    }
}

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

pub struct RingBuffer3x3<T: Copy> {
    values: [[T; 3]; 3],
}

impl<T: Copy> RingBuffer3x3<T> {
    pub fn new(values: [[T; 3]; 3]) -> Self {
        Self { values }
    }

    pub fn center(&self) -> T {
        self.values[1][1]
    }

    fn shift_forward<C: Copy>(arr: &mut [C; 3], new: C) {
        arr[2] = arr[1];
        arr[1] = arr[0];
        arr[0] = new;
    }

    fn shift_backward<C: Copy>(arr: &mut [C; 3], new: C) {
        arr[0] = arr[1];
        arr[1] = arr[2];
        arr[2] = new;
    }

    pub fn shift_left(&mut self, new_col: [T; 3]) {
        for row in 0..3 {
            Self::shift_backward(&mut self.values[row], new_col[row]);
        }
    }

    pub fn shift_right(&mut self, new_col: [T; 3]) {
        for row in 0..3 {
            Self::shift_forward(&mut self.values[row], new_col[row]);
        }
    }

    pub fn shift_up(&mut self, new_row: [T; 3]) {
        Self::shift_backward(&mut self.values, new_row);
    }

    pub fn shift_down(&mut self, new_row: [T; 3]) {
        Self::shift_forward(&mut self.values, new_row);
    }
}
