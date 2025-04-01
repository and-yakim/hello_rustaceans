use hello_rustaceans::qtree::*;
use hello_rustaceans::world::*;

#[macroquad::main("Map maker")]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);

    let mut screen = Screen::new();

    let region = Rect::new(-CELL / 2.0, -CELL / 2.0, CELL, CELL);
    let mut quadtree: QTreeMut<Item> = QTreeMut::new(region.into(), vec![]);

    // let tools = ();

    let mut click_value: Option<Item> = None;

    loop {
        let mouse_pos = Vec2::from(mouse_position());
        let world_click = screen.world_pos(mouse_pos);
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
            screen.target.x += 10.0 / screen.scale;
        }
        if is_key_down(KeyCode::A) {
            screen.target.x -= 10.0 / screen.scale;
        }
        if is_key_down(KeyCode::S) {
            screen.target.y += 10.0 / screen.scale;
        }
        if is_key_down(KeyCode::W) {
            screen.target.y -= 10.0 / screen.scale;
        }

        let camera = Camera2D {
            target: screen.target,
            zoom: screen.zoom(),
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

        if screen.scale > 0.1 {
            screen.draw_grid();
        }

        let world_rect = screen.world_rec_to_render();
        quadtree.draw(screen.scale, world_rect);

        draw_circle(grid_knot.x, grid_knot.y, 8.0 / screen.scale, KNOT_COLOR);

        if let Some(ref item) = click_value {
            let wh = grid_knot - item.rect.point();
            draw_rectangle(item.rect.x, item.rect.y, wh.x, wh.y, RECT_COLOR);
        }

        set_default_camera();

        // tools

        match get_last_key_pressed() {
            Some(KeyCode::Q) => {
                screen.scale *= 1.2;
            }
            Some(KeyCode::E) => {
                screen.scale /= 1.2;
            }
            _ => {}
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
