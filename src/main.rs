use hello_rustaceans::qtree::*;
use hello_rustaceans::world::*;

use std::time;

#[macroquad::main("Platformer")]
async fn main() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    set_default_filter_mode(FilterMode::Nearest);

    let screen_wh = vec2(screen_width(), screen_height());
    let screen_center = screen_wh / 2.0;

    let mut target = Vec2::ZERO;

    loop {
        let click = Vec2::from(mouse_position());
        let world_click = (click - screen_center) + target;
        let grid_knot = (world_click / GRID).round() * GRID;
        if is_mouse_button_pressed(MouseButton::Left) {}

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

        let map_coords = target.coords(CELL);
        println!("target: {target}");
        println!("target: {map_coords}");

        let camera = Camera2D {
            target,
            zoom: Vec2::ONE / screen_center,
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

        draw_grid(screen_center, 1.0, target, screen_wh);

        draw_circle(grid_knot.x, grid_knot.y, 8.0, KNOT_COLOR);

        set_default_camera();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
