use hello_rustaceans::qtree::*;
use hello_rustaceans::world::*;

use std::time;

use macroquad::prelude::*;

const GRID: f32 = 32.0;
const CELL: f32 = GRID * 16.0;

const fn make_transparent(color: Color, a: f32) -> Color {
    Color::new(color.r, color.g, color.b, a)
}
const GRID_COLOR: Color = make_transparent(LIGHTGRAY, 0.20);
const KNOT_COLOR: Color = make_transparent(RED, 0.50);
const RECT_COLOR: Color = make_transparent(GREEN, 0.50);

fn world_pos(screen_point: Vec2, screen_center: Vec2, target: Vec2) -> Vec2 {
    (screen_point - screen_center) + target
}

// fn screen_pos(world_point: Vec2, screen_center: Vec2, target: Vec2) -> Vec2 {
//     (world_point - target) + screen_center
// }

fn world_to_cell(pos: Vec2) -> IVec2 {
    ivec2((pos.x / CELL).round() as i32, (pos.y / CELL).round() as i32)
}

#[macroquad::main("Platformer")]
async fn main() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    set_default_filter_mode(FilterMode::Nearest);

    let screen_wh = vec2(screen_width(), screen_height());
    let screen_center = screen_wh / 2.0;
    let region = Rect::new(-screen_center.x, -screen_center.y, screen_wh.y, screen_wh.y);

    let mut quadtree: QTreeMut<Item> = QTreeMut::new(region.into(), vec![]);

    let mut target = vec2(CELL / 2.0, CELL / 2.0);

    // let tools = ();

    let mut click_value: Option<Item> = None;

    loop {
        let click = Vec2::from(mouse_position());
        let world_click = (click - screen_center) + target;
        let grid_knot = (world_click / GRID).round() * GRID;
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(item) = click_value {
                let wh = grid_knot - item.rect.point();
                let value = Rect::new(item.rect.x, item.rect.y, wh.x, wh.y);
                quadtree.add(value.into());
                click_value = None;
            } else {
                click_value = Some(Item::new(grid_knot))
            }
        }

        if is_key_down(KeyCode::D) {
            target.x += 10.0;
        }
        if is_key_down(KeyCode::A) {
            target.x -= 10.0;
        }
        if is_key_down(KeyCode::S) {
            target.y += 10.0;
        }
        if is_key_down(KeyCode::W) {
            target.y -= 10.0;
        }

        let map_coords = world_to_cell(target);
        println!("target: {target}");
        println!("target: {map_coords}");

        let camera = Camera2D {
            target,
            zoom: Vec2::ONE / screen_center,
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

        let world_zero = world_pos(Vec2::ZERO, screen_center, target);
        let world_corner = world_pos(screen_wh, screen_center, target);

        let start = (world_zero / GRID).floor() * GRID;
        let end = (world_corner / GRID).ceil() * GRID;

        for i in 0..=((world_corner.x - world_zero.x + GRID) / GRID) as usize {
            let x = start.x + GRID * i as f32;
            draw_line(x, start.y, x, end.y, 1.0, GRID_COLOR);
        }
        for j in 0..=((world_corner.y - world_zero.y + GRID) / GRID) as usize {
            let y = start.y + GRID * j as f32;
            draw_line(start.x, y, end.x, y, 1.0, GRID_COLOR);
        }

        quadtree.draw(1.0);

        draw_circle(grid_knot.x, grid_knot.y, 8.0, KNOT_COLOR);

        if let Some(ref item) = click_value {
            let wh = grid_knot - item.rect.point();
            draw_rectangle(item.rect.x, item.rect.y, wh.x, wh.y, RECT_COLOR);
        }

        set_default_camera();

        // tools

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
