mod qtree;
use qtree::*;

use std::time;

use macroquad::prelude::*;

fn draw_region(rect: &Rect, scale: f32, color: Color) {
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        screen_height() / 100.0,
        color,
    );
    let font_size: f32 = 20.0 * scale;
    draw_text(
        &format!("{}", rect.w),
        rect.x + rect.w / 2.0 - rect.w.log10() * font_size / 4.0,
        rect.y + font_size,
        font_size,
        color,
    );
}

impl<T: Clone + Positioned> QTreeMut<T> {
    fn draw(&self, scale: f32) {
        match self {
            QTreeMut::BlankNode {
                region, children, ..
            } => {
                draw_region(region, scale, BLUE);
                for node in children.iter() {
                    node.draw(scale);
                }
            }
            QTreeMut::ValueNode { region, .. } => {
                draw_region(region, scale, GREEN);
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

#[macroquad::main("Quadtree builder")]
async fn main() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }

    let (width, height) = (screen_width(), screen_height());
    let region = Rect::new(0.0, 0.0, height, height);

    let mut quadtree: QTreeMut<MyObstacle> = QTreeMut::new(region, vec![]);

    let mut target = vec2(width / 2.0, height / 2.0);
    let mut scale = 2.0;
    let mut camera = Camera2D {
        target,
        zoom: vec2(scale / width, scale / height),
        ..Default::default()
    };

    loop {
        match get_last_key_pressed() {
            Some(KeyCode::Q) => {
                scale *= 1.1;
            }
            Some(KeyCode::E) => {
                scale /= 1.1;
            }
            _ => {}
        }

        if is_key_down(KeyCode::D) {
            target.x += 10.0 / scale;
        }
        if is_key_down(KeyCode::A) {
            target.x -= 10.0 / scale;
        }
        if is_key_down(KeyCode::S) {
            target.y += 10.0 / scale;
        }
        if is_key_down(KeyCode::W) {
            target.y -= 10.0 / scale;
        }

        camera = Camera2D {
            target,
            zoom: vec2(scale / width, scale / height),
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

        quadtree.draw(scale);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
