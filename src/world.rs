use crate::qtree::*;
pub use macroquad::prelude::*;

pub const GRID: f32 = 32.0;
pub const CELL: f32 = GRID * 16.0;

const fn make_transparent(color: Color, a: f32) -> Color {
    Color::new(color.r, color.g, color.b, a)
}
pub const GRID_COLOR: Color = make_transparent(LIGHTGRAY, 0.20);
pub const KNOT_COLOR: Color = make_transparent(RED, 0.50);
pub const RECT_COLOR: Color = make_transparent(GREEN, 0.50);

pub fn world_pos(screen_point: Vec2, screen_center: Vec2, scale: f32, target: Vec2) -> Vec2 {
    (screen_point - screen_center) / scale + target
}

pub fn screen_pos(world_point: Vec2, screen_center: Vec2, scale: f32, target: Vec2) -> Vec2 {
    (world_point - target) * scale + screen_center
}

pub fn world_rec_to_render(screen_center: Vec2, scale: f32, target: Vec2, screen_wh: Vec2) -> Rect {
    let world_zero = world_pos(Vec2::ZERO, screen_center, scale, target);
    let world_wh = world_pos(screen_wh, screen_center, scale, target) - world_zero;
    Rect::new(world_zero.x, world_zero.y, world_wh.x, world_wh.y)
}

pub fn draw_grid(screen_center: Vec2, scale: f32, target: Vec2, screen_wh: Vec2) {
    let world_zero = world_pos(Vec2::ZERO, screen_center, scale, target);
    let world_corner = world_pos(screen_wh, screen_center, scale, target);
    let start = (world_zero / GRID).floor() * GRID;
    let end = (world_corner / GRID).ceil() * GRID;

    for i in 0..=((world_corner.x - world_zero.x + GRID) / GRID) as usize {
        let x = start.x + GRID * i as f32;
        draw_line(x, start.y, x, end.y, 1.0 / scale, GRID_COLOR);
    }
    for j in 0..=((world_corner.y - world_zero.y + GRID) / GRID) as usize {
        let y = start.y + GRID * j as f32;
        draw_line(start.x, y, end.x, y, 1.0 / scale, GRID_COLOR);
    }
}

impl<T: Clone + Positioned> QTreeMut<T> {
    pub fn draw(&self, scale: f32, world_rect: Rect) {
        match self {
            QTreeMut::Node { children, .. } => {
                for node in children {
                    node.draw(scale, world_rect);
                }
            }
            QTreeMut::Leaf { region, values } => {
                if region.intersect(world_rect).is_some() {
                    draw_rectangle_lines(
                        region.x,
                        region.y,
                        region.w,
                        region.h,
                        2.0 / scale,
                        GREEN,
                    );
                    for v in values {
                        v.draw();
                    }
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
