mod qtree;
use qtree::*;

use std::time;

use macroquad::prelude::*;

fn draw_region(rect: &Rect) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLUE);
}

impl<T> QTree<T> {
    fn draw(&self) {
        match self {
            QTree::BlankNode {
                region, children, ..
            } => {
                draw_region(region);
                for node in children.iter() {
                    node.draw();
                }
            }
            QTree::ValueNode { region, .. } => {
                draw_region(region);
            }
        }
    }
}

#[macroquad::main("Platformer")]
async fn main() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }

    let (width, height) = (screen_width(), screen_height());
    let win_rect = Rect::new(0.0, 0.0, width, height);

    let mut quadtree: QTree<Vec2> = QTree::new(win_rect);
    println!("{:#?}", quadtree);

    loop {
        clear_background(DARKGRAY);

        quadtree.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
