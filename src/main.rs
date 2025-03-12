mod qtree;
use qtree::*;

use std::time;

use macroquad::prelude::*;

fn draw_region(rect: &Rect, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, color);
}

impl<T: Clone + Positioned> QTreeMut<T> {
    fn draw(&self) {
        match self {
            QTreeMut::BlankNode {
                region, children, ..
            } => {
                draw_region(region, BLUE);
                for node in children.iter() {
                    node.draw();
                }
            }
            QTreeMut::ValueNode { region, .. } => {
                draw_region(region, GREEN);
            }
        }
    }
}

#[derive(Clone)]
struct MyObstacle {
    value: Vec2,
}

impl Positioned for MyObstacle {
    fn pos(&self) -> Vec2 {
        self.value
    }
}

#[macroquad::main("Platformer")]
async fn main() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }

    let (width, height) = (screen_width(), screen_height());
    let win_rect = Rect::new(0.0, 0.0, width, height);

    let mut quadtree: QTreeMut<MyObstacle> = QTreeMut::new(win_rect, vec![]);

    loop {
        clear_background(DARKGRAY);

        quadtree.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
