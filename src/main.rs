use hello_rustaceans::animation::*;
use hello_rustaceans::qtree::*;
use hello_rustaceans::world::*;

use std::time;

#[macroquad::main("Platformer")]
async fn main() {
    let mut instant = time::Instant::now();
    if let Ok(n) = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
        rand::srand(n.as_secs());
    }
    set_default_filter_mode(FilterMode::Nearest);

    let mut screen = Screen::new();
    let zoom = Vec2::ONE / screen.center;

    let sprite =
        Texture2D::from_file_with_format(include_bytes!("../resources/player_sprite.png"), None);
    sprite.set_filter(FilterMode::Nearest);

    let mut position = screen.target;
    let mut direction = Dir::Right;
    let mut player_move = PlayerState::Idle;

    let mut animation = Animation::new(player_move, direction);

    loop {
        if is_key_down(KeyCode::D) {
            direction = Dir::Right;
        }
        if is_key_down(KeyCode::A) {
            direction = Dir::Left;
        }
        if is_key_down(KeyCode::S) {
            direction = Dir::Down;
        }
        if is_key_down(KeyCode::W) {
            direction = Dir::Up;
        }

        // let map_coords = target.coords(CELL);

        if instant.elapsed().as_millis() > TIMESTEP {
            instant = time::Instant::now();

            animation.update(player_move, direction);

            animation.step();
        }

        let camera = Camera2D {
            target: screen.target,
            zoom,
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

        animation.draw(&sprite, position);

        screen.draw_grid();

        set_default_camera();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
