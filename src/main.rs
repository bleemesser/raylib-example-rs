use raylib::prelude::*;

const POINTS_OF_INTEREST: [[f32; 2]; 6] = [
    [-0.348827, 0.607167],
    [-0.786268, 0.169728],
    [-0.8, 0.156],
    [0.285, 0.0],
    [-0.835, -0.2321],
    [-0.70176, -0.3842],
];

const INITIAL_SCREEN_WIDTH: i32 = 800;
const INITIAL_SCREEN_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, mut thread) = raylib::init()
        .size(INITIAL_SCREEN_WIDTH, INITIAL_SCREEN_HEIGHT)
        .title("raylib [shaders] example - julia set")
        .resizable()
        .build();

    let shader_path = "shaders/julia_set.fs";
    let mut shader = rl
        .load_shader(&mut thread, None, Some(&shader_path));

    let c_loc = shader.get_shader_location("c");
    let zoom_loc = shader.get_shader_location("zoom");
    let offset_loc = shader.get_shader_location("offset");
    let screen_size_loc = shader.get_shader_location("screenSize");

    let mut c: Vector2 = (POINTS_OF_INTEREST[0][0], POINTS_OF_INTEREST[0][1]).into();
    let mut zoom = 0.75;
    let mut offset = Vector2::zero();

    shader.set_shader_value(c_loc, c);
    shader.set_shader_value(zoom_loc, zoom);
    shader.set_shader_value(offset_loc, offset);

    let mut target = rl
        .load_render_texture(&thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32)
        .expect("Failed to load render texture");

    let mut increment_speed: f32 = 0.0;
    let mut show_controls = true;

    let mut is_panning = false;
    let mut pan_start_pos = Vector2::zero();
    let mut offset_at_pan_start = Vector2::zero();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let start = std::time::Instant::now();
        if rl.is_window_resized() {
            target = rl.load_render_texture(&thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32)
                .expect("Failed to load render texture after window resize");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) { c = (POINTS_OF_INTEREST[0][0], POINTS_OF_INTEREST[0][1]).into(); }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) { c = (POINTS_OF_INTEREST[1][0], POINTS_OF_INTEREST[1][1]).into(); }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) { c = (POINTS_OF_INTEREST[2][0], POINTS_OF_INTEREST[2][1]).into(); }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) { c = (POINTS_OF_INTEREST[3][0], POINTS_OF_INTEREST[3][1]).into(); }
        if rl.is_key_pressed(KeyboardKey::KEY_FIVE) { c = (POINTS_OF_INTEREST[4][0], POINTS_OF_INTEREST[4][1]).into(); }
        if rl.is_key_pressed(KeyboardKey::KEY_SIX) { c = (POINTS_OF_INTEREST[5][0], POINTS_OF_INTEREST[5][1]).into(); }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            zoom = 0.75;
            offset = Vector2::zero();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_F1) { show_controls = !show_controls; }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) { increment_speed = 0.0; }

        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) { increment_speed += 1.0; }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) { increment_speed -= 1.0; }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            is_panning = true;
            pan_start_pos = rl.get_mouse_position();
            offset_at_pan_start = offset;
        }

        if is_panning {
            let mouse_delta = rl.get_mouse_position() - pan_start_pos;
            let scale = 2.5 / (zoom * rl.get_screen_height() as f32);

            let offset_delta = mouse_delta * scale;
            offset.x = offset_at_pan_start.x - offset_delta.x;
            offset.y = offset_at_pan_start.y + offset_delta.y; // y is inverted
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            is_panning = false;
        }

        let wheel_move = rl.get_mouse_wheel_move();
        if wheel_move != 0.0 {
            let mouse_pos = rl.get_mouse_position();
            let aspect_ratio = rl.get_screen_width() as f32 / rl.get_screen_height() as f32;
            
            let mut coord_before_zoom = Vector2::new(
                (mouse_pos.x / rl.get_screen_width() as f32 - 0.5) * aspect_ratio,
                -(mouse_pos.y / rl.get_screen_height() as f32 - 0.5)
            );
            coord_before_zoom *= 2.5 / zoom;
            coord_before_zoom += offset;

            let zoom_factor = 1.1;
            if wheel_move > 0.0 { zoom *= zoom_factor; } else { zoom /= zoom_factor; }
            
            let mut coord_after_zoom = Vector2::new(
                (mouse_pos.x / rl.get_screen_width() as f32 - 0.5) * aspect_ratio,
                -(mouse_pos.y / rl.get_screen_height() as f32 - 0.5)
            );
            coord_after_zoom *= 2.5 / zoom;
            coord_after_zoom += offset;
            
            offset += coord_before_zoom - coord_after_zoom;
        }

        let key_zoom_speed = 1.02;
        if rl.is_key_down(KeyboardKey::KEY_E) { zoom *= key_zoom_speed; }
        if rl.is_key_down(KeyboardKey::KEY_Q) { zoom /= key_zoom_speed; }

        let dc = rl.get_frame_time() * increment_speed * 0.0005;
        c.x += dc;
        c.y += dc;

        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        shader.set_shader_value(c_loc, c);
        shader.set_shader_value(zoom_loc, zoom);
        shader.set_shader_value(offset_loc, offset);
        shader.set_shader_value(screen_size_loc, screen_size);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d_tex = d.begin_texture_mode(&thread, &mut target);
            d_tex.draw_rectangle(0, 0, d_tex.get_screen_width(), d_tex.get_screen_height(), Color::BLACK);
        }

        {
            let mut d_shader = d.begin_shader_mode(&mut shader);
            d_shader.draw_texture(target.texture(), 0, 0, Color::WHITE);
        }

        if show_controls {
            d.draw_text("Drag with Left Mouse to Pan", 10, 15, 10, Color::RAYWHITE);
            d.draw_text("Use Mouse Wheel or Q/E to Zoom", 10, 30, 10, Color::RAYWHITE);
            d.draw_text("Press F1 to toggle controls", 10, 45, 10, Color::RAYWHITE);
            d.draw_text("Press [1-6] to change fractal", 10, 60, 10, Color::RAYWHITE);
            d.draw_text("Press ARROWS to change animation speed", 10, 75, 10, Color::RAYWHITE);
            d.draw_text("Press SPACE to stop animation", 10, 90, 10, Color::RAYWHITE);
            d.draw_text("Press R to reset view", 10, 105, 10, Color::RAYWHITE);
        }
        let elapsed = start.elapsed().as_secs_f32();
        d.draw_text(&format!("Frame Time: {:.3} ms | Dimensions: {}x{}", elapsed * 1000.0, d.get_screen_width(), d.get_screen_height()), 10, d.get_screen_height() - 20, 10, Color::RAYWHITE);
    }
}
