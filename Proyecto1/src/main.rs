mod maze;
mod player;
mod caster;
mod framebuffer;
mod sprite;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use sprite::{Sprite, SpriteRenderer, is_empty_cell};

use crate::maze::load_maze_from_file;
use crate::player::Player;
use crate::caster::render_scene;
use crate::framebuffer::{draw_background, draw_minimap, draw_fps};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

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

    //Sprite
    let mut sprite_renderer = SpriteRenderer::new();

    // Carga texturas
    let album_texture = texture_creator.load_texture("../assets/sprites/album.png")?;
    sprite_renderer.add_texture(album_texture);

    let hs_texture = texture_creator.load_texture("../assets/sprites/hs.png")?;
    sprite_renderer.add_texture(hs_texture);

    // Ahora puedes agregar sprites con texture_index 0 y 1 respectivamente:
    sprite_renderer.add_sprite(Sprite { x: 10.5, y: 1.5, texture_index: 0 }); // álbum
    // hs.png (posición aleatoria en celda vacía)
    loop {
        let x = rand::random::<f64>() * (maze::MAP_WIDTH as f64);
        let y = rand::random::<f64>() * (maze::MAP_HEIGHT as f64);

        if is_empty_cell(x, y) {
            sprite_renderer.add_sprite(Sprite { x, y, texture_index: 1 });
            break;
        }
    }

    // Fuente
    let font = ttf_context.load_font("/System/Library/Fonts/Supplemental/Arial.ttf", 24)?;

    // Cargar laberinto desde archivo
    load_maze_from_file("../maze.txt")?;

    // Álbum sprite
    let album_texture = texture_creator.load_texture("../assets/sprites/album.png")?;
    let frame_width = 32;
    let frame_height = 32;
    let mut frame = 0;
    let mut frame_timer = 0.0;

    // Texturas paredes
    let mut wall_textures = Vec::new();
    for i in 1..=10 {
        let path = format!("../assets/tx/{}.png", i);
        let texture = texture_creator.load_texture(path)?;
        wall_textures.push(texture);
    }

    // Mouse
    let mouse_util = sdl_context.mouse();
    mouse_util.set_relative_mouse_mode(true);
    mouse_util.show_cursor(false);

    // Estado jugador
    let mut player = Player::new();

    let mut event_pump = sdl_context.event_pump()?;
    let mut last_time = Instant::now();
    let mut victoria = false;

    //FOV
    const FOV: f64 = std::f64::consts::PI / 3.0;

    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f64();
        last_time = now;

        let plane_x = -player.dir_angle.sin() * (FOV / 2.0).tan();
        let plane_y = player.dir_angle.cos() * (FOV / 2.0).tan();


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
        player.update_position(&keys, delta_time);

        // Rotación con el mouse
        let mouse_state = event_pump.relative_mouse_state();
        player.rotate(mouse_state.x());

        // Comprobar victoria
        unsafe {
            if maze::MAP[player.y as usize][player.x as usize] == 5 {
                maze::MAP[player.y as usize][player.x as usize] = 0;
                sdl2::mixer::Channel::all().play(&pickup_sound, 0)?;
                victoria = true;
            }
        }

        // Pantalla victoria
        if victoria {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            let surface = font.render("¡Has recuperado el álbum!")
                .blended(Color::RGB(255, 255, 255)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            canvas.copy(&texture, None, Rect::new(200, 250, 400, 50)).unwrap();
            
            canvas.present();
            std::thread::sleep(Duration::from_secs(3));
            break 'running;
        }

        // Fondo
        draw_background(&mut canvas, SCREEN_WIDTH, SCREEN_HEIGHT)?;

        // Raycasting
        render_scene(&mut canvas, &player, &wall_textures, SCREEN_WIDTH, SCREEN_HEIGHT)?;

        // Dibujar sprites
        sprite_renderer.draw_sprites(
            &mut canvas,
            player.x,
            player.y,
            player.dir_angle,
            plane_x,
            plane_y,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )?;

        // Minimapa
        draw_minimap(&mut canvas, &player, SCREEN_WIDTH)?;

        // FPS
        let fps = (1.0 / delta_time) as i32;
        draw_fps(&mut canvas, &font, &texture_creator, fps)?;

        // Presentar frame
        canvas.present();
        std::thread::sleep(Duration::from_millis(65));
    }

    Ok(())
}
