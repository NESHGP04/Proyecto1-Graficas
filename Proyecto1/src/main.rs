use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const MAP_WIDTH: usize = 12;
const MAP_HEIGHT: usize = 12;
const FOV: f64 = std::f64::consts::PI / 3.0;
const MOVE_SPEED: f64 = 3.0;
const ROT_SPEED: f64 = 2.0;

// 0 = vacío, 1-4 = paredes, 5 = álbum
static mut MAP: [[i32; MAP_WIDTH]; MAP_HEIGHT] = [
    [1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,1,0,0,0,0,0,5,1],
    [1,0,1,0,1,0,1,1,1,0,1,1],
    [1,0,1,0,0,0,0,0,1,0,0,1],
    [1,0,1,1,1,1,1,0,1,1,0,1],
    [1,0,0,0,0,0,1,0,0,1,0,1],
    [1,1,1,1,1,0,1,1,0,1,0,1],
    [1,0,0,0,1,0,0,0,0,1,0,1],
    [1,0,1,0,1,1,1,1,0,1,0,1],
    [1,0,1,0,0,0,0,1,0,0,0,1],
    [1,0,0,0,1,0,0,0,0,1,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1],
];

fn wall_color(id: i32) -> Color {
    match id {
        1 => Color::RGB(255, 0, 0),
        2 => Color::RGB(0, 255, 0),
        3 => Color::RGB(0, 0, 255),
        4 => Color::RGB(255, 255, 0),
        _ => Color::RGB(200, 200, 200),
    }
}

fn main() -> Result<(), String> {
    // SDL Init
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;
    let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3)?;

    // Audio
    sdl2::mixer::open_audio(44100, sdl2::mixer::AUDIO_S16LSB, 2, 1024)?;
    sdl2::mixer::allocate_channels(4);

    let music = sdl2::mixer::Music::from_file("../assets/music/taylor.mp3")?;
    let pickup_sound = sdl2::mixer::Chunk::from_file("../assets/sfx/pickup.mp3")?;
    music.play(-1)?;

    // Ventana
    let window = video_subsystem
        .window("Taylor Swift Labyrinth", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Fuente
    let font = ttf_context.load_font("/System/Library/Fonts/Supplemental/Arial.ttf", 24)?;

    // Álbum sprite
    let album_texture = texture_creator.load_texture("../assets/sprites/album.webp")?;
    let frame_width = 32;
    let frame_height = 32;
    let mut frame = 0;
    let mut frame_timer = 0.0;

    // Mouse
    let mouse_util = sdl_context.mouse();
    mouse_util.set_relative_mouse_mode(true);
    mouse_util.show_cursor(false);

    // Estado jugador
    let mut pos_x: f64 = 1.5;
    let mut pos_y: f64 = 1.5;
    let mut dir_angle: f64 = 0.0;

    let mut event_pump = sdl_context.event_pump()?;
    let mut last_time = Instant::now();

    let mut victoria = false;

    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f64();
        last_time = now;

        // Eventos
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Movimiento
        let keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let move_step = MOVE_SPEED * delta_time;

        unsafe {
            if keys.contains(&Keycode::W) {
                let nx = pos_x + dir_angle.cos() * move_step;
                let ny = pos_y + dir_angle.sin() * move_step;
                if MAP[ny as usize][nx as usize] != 1 {
                    pos_x = nx;
                    pos_y = ny;
                }
            }
            if keys.contains(&Keycode::S) {
                let nx = pos_x - dir_angle.cos() * move_step;
                let ny = pos_y - dir_angle.sin() * move_step;
                if MAP[ny as usize][nx as usize] != 1 {
                    pos_x = nx;
                    pos_y = ny;
                }
            }
        }

        // Rotación mouse
        let mouse_state = event_pump.relative_mouse_state();
        dir_angle += (mouse_state.x() as f64) * 0.002;

        // Comprobar victoria
        unsafe {
            if MAP[pos_y as usize][pos_x as usize] == 5 {
                MAP[pos_y as usize][pos_x as usize] = 0;
                sdl2::mixer::Channel::all().play(&pickup_sound, 0)?;
                victoria = true;
            }
        }

        // Pantalla de victoria
        if victoria {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            let surface = font.render("¡Has recuperado el álbum!").blended(Color::RGB(255, 255, 255)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            canvas.copy(&texture, None, Rect::new(200, 250, 400, 50)).unwrap();
            canvas.present();
            std::thread::sleep(Duration::from_secs(3));
            break 'running;
        }

        // Fondo
        canvas.set_draw_color(Color::RGB(135, 206, 235)); // cielo
        canvas.fill_rect(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT / 2))?;
        canvas.set_draw_color(Color::RGB(80, 80, 80)); // suelo
        canvas.fill_rect(Rect::new(0, (SCREEN_HEIGHT / 2) as i32, SCREEN_WIDTH, SCREEN_HEIGHT / 2))?;

        // Raycasting
        unsafe {
            for x in 0..SCREEN_WIDTH {
                let camera_x = 2.0 * x as f64 / SCREEN_WIDTH as f64 - 1.0;
                let ray_angle = dir_angle + camera_x * FOV / 2.0;
                let ray_dir_x = ray_angle.cos();
                let ray_dir_y = ray_angle.sin();

                let mut distance = 0.0;
                let mut wall_id = 0;
                while distance < 20.0 {
                    distance += 0.05;
                    let tx = (pos_x + ray_dir_x * distance) as i32;
                    let ty = (pos_y + ray_dir_y * distance) as i32;

                    if tx < 0 || tx >= MAP_WIDTH as i32 || ty < 0 || ty >= MAP_HEIGHT as i32 {
                        wall_id = 1;
                        break;
                    } else if MAP[ty as usize][tx as usize] > 0 && MAP[ty as usize][tx as usize] != 5 {
                        wall_id = MAP[ty as usize][tx as usize];
                        break;
                    }
                }

                let wall_height = (SCREEN_HEIGHT as f64 / distance) as i32;
                let draw_start = -(wall_height / 2) + (SCREEN_HEIGHT as i32 / 2);
                let draw_end = (wall_height / 2) + (SCREEN_HEIGHT as i32 / 2);

                canvas.set_draw_color(wall_color(wall_id));
                canvas.fill_rect(Rect::new(x as i32, draw_start, 1, (draw_end - draw_start) as u32))?;
            }

            // Dibujar álbum (sprite)
            for my in 0..MAP_HEIGHT {
                for mx in 0..MAP_WIDTH {
                    if MAP[my][mx] == 5 {
                        let sprite_x = mx as f64 + 0.5 - pos_x;
                        let sprite_y = my as f64 + 0.5 - pos_y;
                        let inv_det = 1.0 / (dir_angle.cos() * 0.0 - 0.0 * dir_angle.sin());
                        let transform_x = inv_det * (dir_angle.sin() * sprite_x - dir_angle.cos() * sprite_y);
                        let transform_y = inv_det * (-0.0 * sprite_x + 0.0 * sprite_y);
                        if transform_y > 0.0 {
                            let sprite_screen_x = (SCREEN_WIDTH as f64 / 2.0) * (1.0 + transform_x / transform_y);
                            let sprite_size = (SCREEN_HEIGHT as f64 / transform_y) as u32;
                            let src = Rect::new(frame * frame_width, 0, frame_width as u32, frame_height as u32);
                            let dst = Rect::new(
                                (sprite_screen_x - sprite_size as f64 / 2.0) as i32,
                                (SCREEN_HEIGHT as i32 / 2) - (sprite_size as i32 / 2),
                                sprite_size,
                                sprite_size,
                            );
                            canvas.copy(&album_texture, src, dst)?;
                        }
                    }
                }
            }
        }

        // Minimapa
        let scale = 8;
        unsafe {
            for my in 0..MAP_HEIGHT {
                for mx in 0..MAP_WIDTH {
                    if MAP[my][mx] == 1 {
                        canvas.set_draw_color(Color::RGB(200, 200, 200));
                    } else if MAP[my][mx] == 5 {
                        canvas.set_draw_color(Color::RGB(0, 255, 255));
                    } else {
                        canvas.set_draw_color(Color::RGB(50, 50, 50));
                    }
                    canvas.fill_rect(Rect::new(
                        SCREEN_WIDTH as i32 - MAP_WIDTH as i32 * scale + mx as i32 * scale,
                        my as i32 * scale,
                        scale as u32,
                        scale as u32,
                    ))?;
                }
            }
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(
            SCREEN_WIDTH as i32 - MAP_WIDTH as i32 * scale + (pos_x as i32) * scale,
            (pos_y as i32) * scale,
            scale as u32,
            scale as u32,
        ))?;

        // Animación álbum
        frame_timer += delta_time;
        if frame_timer >= 0.15 {
            frame_timer = 0.0;
            frame = (frame + 1) % 4;
        }

        // FPS
        let fps = (1.0 / delta_time) as i32;
        let surface = font.render(&format!("FPS: {}", fps))
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        canvas.copy(&texture, None, Rect::new(10, 10, 100, 30))?;

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
