use hello_rustaceans::qtree::*;
use hello_rustaceans::world::*;

use std::time;

#[macroquad::main("Platformer")]
async fn main() {
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    set_default_filter_mode(FilterMode::Nearest);

    let mut screen = Screen::new();
    let zoom = Vec2::ONE / screen.center;

    loop {
        if is_key_down(KeyCode::D) {
            screen.target.x += 10.0;
        }
        if is_key_down(KeyCode::A) {
            screen.target.x -= 10.0;
        }
        if is_key_down(KeyCode::S) {
            screen.target.y += 10.0;
        }
        if is_key_down(KeyCode::W) {
            screen.target.y -= 10.0;
        }

        // let map_coords = target.coords(CELL);

        let camera = Camera2D {
            target: screen.target,
            zoom,
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

        screen.draw_grid();

        set_default_camera();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
