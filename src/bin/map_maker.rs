use hello_rustaceans::qtree::*;
use hello_rustaceans::world::*;

#[macroquad::main("Map maker")]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);

    let screen_wh = vec2(screen_width(), screen_height());
    let screen_center = screen_wh / 2.0;
    let region = Rect::new(-CELL / 2.0, -CELL / 2.0, CELL, CELL);

    let mut quadtree: QTreeMut<Item> = QTreeMut::new(region.into(), vec![]);

    let mut target = Vec2::ZERO;
    let mut scale = 1.0;

    // let tools = ();

    let mut click_value: Option<Item> = None;

    loop {
        let click = Vec2::from(mouse_position());
        let world_click = (click - screen_center) / scale + target;
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

        let map_coords = target.coords(CELL);
        println!("target: {target}");
        println!("target: {map_coords}");

        let camera = Camera2D {
            target,
            zoom: vec2(scale, scale) / screen_center,
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(DARKGRAY);

        if scale > 0.1 {
            draw_grid(screen_center, scale, target, screen_wh);
        }

        let world_rect = world_rec_to_render(screen_center, scale, target, screen_wh);
        quadtree.draw(scale, world_rect);

        draw_circle(grid_knot.x, grid_knot.y, 8.0 / scale, KNOT_COLOR);

        if let Some(ref item) = click_value {
            let wh = grid_knot - item.rect.point();
            draw_rectangle(item.rect.x, item.rect.y, wh.x, wh.y, RECT_COLOR);
        }

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
