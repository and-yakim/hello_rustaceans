mod qtree;
use qtree::*;

use std::time;

use macroquad::prelude::*;

impl<T: Clone + Positioned> QTreeMut<T> {
    fn draw(&self, scale: f32) {
        match self {
            QTreeMut::BlankNode { children, .. } => {
                for node in children.iter() {
                    node.draw(scale);
                }
            }
            QTreeMut::ValueNode { region, depth, .. } => {
                draw_rectangle_lines(region.x, region.y, region.w, region.h, 4.0 / scale, GREEN);
                let font_size: f32 = 40.0 / ((*depth + 2) as f32).log2();
                if *depth < 4 {
                    draw_text(
                        &format!("{}", region.w),
                        region.x + region.w / 2.0 - region.w.log10() * font_size / 4.0,
                        region.y + font_size,
                        font_size,
                        GREEN,
                    );
                }
            }
        }
    }
}

#[derive(Clone)]
struct MyObstacle {
    pos: Vec2,
}

impl Positioned for MyObstacle {
    fn pos(&self) -> Vec2 {
        self.pos
    }
}

enum BuilderMode {
    Tree,
    Obstacles,
}

#[macroquad::main("Map builder")]
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

    // let tools = ();

    loop {
        let click = Vec2::from(mouse_position());
        let world_pos = (click - screen_center) / scale + target;
        if is_mouse_button_pressed(MouseButton::Left) {
            // if tools.contains(world_pos) {} else
            if quadtree.region().contains(world_pos) {
                quadtree = quadtree.split_by_click(world_pos);
            }
        } else if is_mouse_button_pressed(MouseButton::Right) {
            if quadtree.region().contains(world_pos) {
                // quadtree = quadtree.enlarge_by_click(world_pos);
            }
        }

        match get_last_key_pressed() {
            Some(KeyCode::Q) => {
                scale *= 1.2;
            }
            Some(KeyCode::E) => {
                scale /= 1.2;
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

        draw_circle(world_pos.x, world_pos.y, 4.0 / scale, RED);

        set_default_camera();

        // tools

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
