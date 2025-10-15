use raylib::prelude::*;

const INITIAL_SCREEN_WIDTH: i32 = 1280;
const INITIAL_SCREEN_HEIGHT: i32 = 720;

const WALL_X_POS_FACTOR_SCREEN: f32 = 0.35;
const WALL_THICKNESS: f32 = 8.0;

fn main() {
    let (mut rl, mut thread) = raylib::init()
        .size(INITIAL_SCREEN_WIDTH, INITIAL_SCREEN_HEIGHT)
        .title("raylib [shaders] example - double slit setup")
        .resizable()
        .build();

    let shader_path = "shaders/double_slit.fs";
    let mut shader = rl.load_shader(&mut thread, None, Some(shader_path));

    let screen_size_loc = shader.get_shader_location("screenSize");
    let time_loc = shader.get_shader_location("time");
    let wavelength_loc = shader.get_shader_location("wavelength");
    let slit_width_loc = shader.get_shader_location("slitWidth");
    let slit_distance_loc = shader.get_shader_location("slitDistance");
    let wall_x_loc = shader.get_shader_location("wallXPos");
    let wall_thickness_loc = shader.get_shader_location("wallThickness");
    shader.set_shader_value(wall_thickness_loc, WALL_THICKNESS);

    let mut time = 0.0;
    let mut animation_speed = 1.0;
    let mut wavelength: f32 = 0.05;
    let mut slit_width: f32 = 20.0;
    let mut slit_distance: f32 = 80.0;
    let mut wall_x_pos: f32 = (WALL_X_POS_FACTOR_SCREEN - 0.5) * (INITIAL_SCREEN_WIDTH as f32)
        / (INITIAL_SCREEN_HEIGHT as f32);

    let mut target = rl
        .load_render_texture(
            &mut thread,
            INITIAL_SCREEN_WIDTH as u32,
            INITIAL_SCREEN_HEIGHT as u32,
        )
        .unwrap();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            let new_target = rl
                .load_render_texture(
                    &mut thread,
                    rl.get_screen_width() as u32,
                    rl.get_screen_height() as u32,
                )
                .unwrap();
            target = new_target;
            wall_x_pos = (WALL_X_POS_FACTOR_SCREEN - 0.5) * (rl.get_screen_width() as f32)
                / (rl.get_screen_height() as f32);
            println!(
                "Window resized: new dimensions = {} x {}",
                rl.get_screen_width(),
                rl.get_screen_height()
            );
        }

        let screen_w = rl.get_screen_width() as u32;
        let screen_h = rl.get_screen_height() as u32;
        let screen_size = Vector2::new(screen_w as f32, screen_h as f32);

        time += rl.get_frame_time() * animation_speed;

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            animation_speed = if animation_speed == 0.0 { 1.0 } else { 0.0 };
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            wavelength -= 0.001;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            wavelength += 0.001;
        }

        if rl.is_key_down(KeyboardKey::KEY_W) {
            slit_distance += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            slit_distance -= 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            slit_width += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            slit_width -= 1.0;
        }

        wavelength = wavelength.max(0.01);
        wavelength = wavelength.min(0.1);
        slit_width = slit_width.max(2.0);
        slit_distance = slit_distance.max(slit_width + 4.0);

        shader.set_shader_value(screen_size_loc, screen_size);
        shader.set_shader_value(time_loc, time);
        shader.set_shader_value(wavelength_loc, wavelength);
        shader.set_shader_value(slit_width_loc, slit_width);
        shader.set_shader_value(slit_distance_loc, slit_distance);
        shader.set_shader_value(wall_x_loc, wall_x_pos);

        let mut d = rl.begin_drawing(&mut thread);
        d.clear_background(Color::BLACK);
        {
            let mut texture = target.texture();
            let mut sh = d.begin_shader_mode(&mut shader);
            sh.draw_texture_rec(
                &mut texture,
                Rectangle::new(0.0, 0.0, screen_w as f32, -(screen_h as i32) as f32),
                Vector2::new(0.0, 0.0),
                Color::WHITE,
            );
        }

        let screen_w = d.get_screen_width() as f32;
        let screen_h = d.get_screen_height() as f32;
        let wall_x = screen_w * WALL_X_POS_FACTOR_SCREEN;
        let center_y = screen_h / 2.0;

        let slit_half_dist = slit_distance / 2.0;
        let slit_half_width = slit_width / 2.0;

        let top_slit_y = center_y - slit_half_dist;
        let bottom_slit_y = center_y + slit_half_dist;

        let rect1 = Rectangle::new(wall_x, 0.0, WALL_THICKNESS, top_slit_y - slit_half_width);
        let rect2 = Rectangle::new(
            wall_x,
            top_slit_y + slit_half_width,
            WALL_THICKNESS,
            slit_distance - slit_width,
        );
        let rect3_y = bottom_slit_y + slit_half_width;
        let rect3 = Rectangle::new(wall_x, rect3_y, WALL_THICKNESS, screen_h - rect3_y);

        d.draw_rectangle_rec(rect1, Color::RED);
        d.draw_rectangle_rec(rect2, Color::RED);
        d.draw_rectangle_rec(rect3, Color::RED);

        let text_bg = Rectangle::new(0.0, 0.0, 220.0, 100.0);
        d.draw_rectangle_rec(text_bg, Color::new(0, 0, 0, 200));
        let text_bg_right = Rectangle::new((d.get_screen_width() - 160) as f32, 0.0, 160.0, 55.0);
        d.draw_rectangle_rec(text_bg_right, Color::new(0, 0, 0, 200));

        d.draw_text("Double Slit Setup", 10, 10, 20, Color::RAYWHITE);
        d.draw_text("SPACE: Pause/Resume", 10, 40, 10, Color::RAYWHITE);
        d.draw_text("A/S: Change Wavelength", 10, 55, 10, Color::RAYWHITE);
        d.draw_text("Q/W: Change Slit Distance", 10, 70, 10, Color::RAYWHITE);
        d.draw_text("UP/DOWN: Change Slit Width", 10, 85, 10, Color::RAYWHITE);

        let ui_x = d.get_screen_width() - 150;
        d.draw_text(
            &format!("Wavelength: {:.3}px", wavelength),
            ui_x,
            10,
            10,
            Color::LIME,
        );
        d.draw_text(
            &format!("Slit Distance: {:.0}px", slit_distance),
            ui_x,
            25,
            10,
            Color::LIME,
        );
        d.draw_text(
            &format!("Slit Width: {:.0}px", slit_width),
            ui_x,
            40,
            10,
            Color::LIME,
        );
    }
}
