use crate::qtree::*;
use macroquad::prelude::*;

impl<T: Clone + Positioned> QTreeMut<T> {
    pub fn draw(&self, scale: f32) {
        match self {
            QTreeMut::Node { children, .. } => {
                for node in children {
                    node.draw(scale);
                }
            }
            QTreeMut::Leaf { region, values } => {
                draw_rectangle_lines(region.x, region.y, region.w, region.h, 2.0 / scale, GREEN);
                for v in values {
                    v.draw();
                }
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

    fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            DARKGREEN,
        );
    }
}

impl Item {
    pub fn new(pos: Vec2) -> Self {
        Item {
            pos,
            rect: Rect::new(pos.x, pos.y, 0.0, 0.0),
        }
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

pub struct RingBuffer2D<const N: usize, T: Copy> {
    values: [[T; N]; N],
}

impl<const N: usize, T: Copy> RingBuffer2D<N, T> {
    pub fn new(values: [[T; N]; N]) -> Self {
        Self { values }
    }

    pub fn new_3x3(values: [[T; 3]; 3]) -> RingBuffer2D<3, T> {
        RingBuffer2D::<3, T> { values }
    }

    pub fn center(&self) -> T {
        self.values[N / 2][N / 2]
    }

    fn shift_forward<C: Copy>(arr: &mut [C; N], new: C) {
        for i in (1..N).rev() {
            arr[i] = arr[i - 1];
        }
        arr[0] = new;
    }

    fn shift_backward<C: Copy>(arr: &mut [C; N], new: C) {
        for i in 0..(N - 1) {
            arr[i] = arr[i + 1];
        }
        arr[N - 1] = new;
    }

    pub fn shift_left(&mut self, new_col: [T; N]) {
        for row in 0..N {
            Self::shift_backward(&mut self.values[row], new_col[row]);
        }
    }

    pub fn shift_right(&mut self, new_col: [T; N]) {
        for row in 0..N {
            Self::shift_forward(&mut self.values[row], new_col[row]);
        }
    }

    pub fn shift_up(&mut self, new_row: [T; N]) {
        Self::shift_backward(&mut self.values, new_row);
    }

    pub fn shift_down(&mut self, new_row: [T; N]) {
        Self::shift_forward(&mut self.values, new_row);
    }
}
