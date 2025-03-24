mod qtree;
use qtree::*;

use std::time;

use macroquad::prelude::*;

const GRID: f32 = 32.0;
const GRID_COLOR: Color = Color::new(0.78, 0.78, 0.78, 0.20);

const CELL: f32 = GRID * 16.0;

impl<T: Clone + Positioned> QTreeMut<T> {
    fn draw(&self, scale: f32) {
        match self {
            QTreeMut::BlankNode { children, .. } => {
                for node in children.iter() {
                    node.draw(scale);
                }
            }
            QTreeMut::ValueNode { region, .. } => {
                draw_rectangle_lines(region.x, region.y, region.w, region.h, 4.0 / scale, GREEN);
            }
        }
    }
}

#[derive(Clone)]
struct Item {
    pos: Vec2,
    // hitbox: Rect,
}

impl Item {
    fn new(pos: Vec2) -> Self {
        Item { pos }
    }
}

impl Positioned for Item {
    fn pos(&self) -> Vec2 {
        self.pos
    }
}

fn world_pos(screen_point: Vec2, screen_center: Vec2, scale: f32, target: Vec2) -> Vec2 {
    (screen_point - screen_center) / scale + target
}

// fn screen_pos(world_point: Vec2, screen_center: Vec2, scale: f32, target: Vec2) -> Vec2 {
//     (world_point - target) * scale + screen_center
// }

// save world state
// use custom animations
#[macroquad::main("Platformer")]
async fn main() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    set_default_filter_mode(FilterMode::Nearest);

    let screen_wh = vec2(screen_width(), screen_height());
    let screen_center = vec2(screen_wh.x / 2.0, screen_wh.y / 2.0);
    let region = Rect::new(0.0, 0.0, screen_wh.y, screen_wh.y);

    let mut quadtree: QTreeMut<Item> = QTreeMut::new(region.into(), vec![]);

    let mut target = vec2(CELL / 2.0, CELL / 2.0);
    let mut scale = 1.0;

    // let tools = ();

    loop {
        let click = Vec2::from(mouse_position());
        let world_click = (click - screen_center) / scale + target;
        if is_mouse_button_pressed(MouseButton::Left) {
            let value = Item::new(world_click);
            quadtree.add(value);
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
            zoom: vec2(scale, scale) / screen_center,
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

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

        quadtree.draw(scale);

        set_default_camera();

        // tools

        match get_last_key_pressed() {
            Some(KeyCode::Q) => {
                scale *= 1.2;
            }
            Some(KeyCode::E) => {
                scale /= 1.2;
            }
            _ => {}
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
