mod qtree;
use qtree::*;

use std::time;

use macroquad::prelude::*;

fn draw_region(rect: &Rect, scale: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 4.0 / scale, color);
    let font_size: f32 = 40.0 / scale;
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
    set_default_filter_mode(FilterMode::Nearest);

    let (width, height) = (screen_width(), screen_height());
    let screen_center = vec2(width / 2.0, height / 2.0);
    let region = Rect::new(0.0, 0.0, height, height);

    let mut quadtree: QTreeMut<MyObstacle> = QTreeMut::new(region, vec![]);

    let mut target = screen_center;
    let mut scale = 1.0;

    let mut p = screen_center;

    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            let click = Vec2::from(mouse_position());
            let world_pos = (click - screen_center) / scale + target;
            p = world_pos;
            println!("{} {}", click, world_pos);
            if quadtree.region().contains(world_pos) {
                quadtree = quadtree.split_by_click(world_pos);
                println!("Split done!");
            }
        }

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

        let camera = Camera2D {
            target,
            zoom: vec2(2.0 * scale / width, 2.0 * scale / height),
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

        quadtree.draw(scale);

        draw_circle(p.x, p.y, 4.0 / scale, RED);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
