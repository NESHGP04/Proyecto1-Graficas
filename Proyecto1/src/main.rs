mod maze;
mod player;
mod caster;
mod framebuffer;
mod sprite;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
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

    // Sprite Renderer
    let mut sprite_renderer = SpriteRenderer::new();
    let album_texture = texture_creator.load_texture("../assets/sprites/album.png")?;
    sprite_renderer.add_texture(album_texture);
    let hs_texture = texture_creator.load_texture("../assets/sprites/hs.png")?;
    sprite_renderer.add_texture(hs_texture);

    // Pantallas
    let inicio_image = texture_creator.load_texture("../assets/pages/inicio.png")?;
    let instrucciones_image = texture_creator.load_texture("../assets/pages/instrucciones.png")?;
    let victoria_image = texture_creator.load_texture("../assets/pages/victoria.png")?;
    
    // Pantallas de nivel
    let level_images = vec![
        None, // nivel 1 no tiene imagen porque usamos inicio/instrucciones
        Some(texture_creator.load_texture("../assets/pages/level2.png")?),
        Some(texture_creator.load_texture("../assets/pages/level3.png")?),
    ];

    // Niveles
    let levels = vec![
        "../maze/maze1.txt",
        "../maze/maze2.txt",
        "../maze/maze3.txt"
    ];

    // Fuente
    let font = ttf_context.load_font("/System/Library/Fonts/Supplemental/Arial.ttf", 24)?;

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

    const FOV: f64 = std::f64::consts::PI / 3.0;

    let mut event_pump = sdl_context.event_pump()?;

    // -----> Bucle de niveles <-----
    for (i, level_path) in levels.iter().enumerate() {
        println!("Cargando nivel {}", i + 1);

        // cargar maze
        let _maze = load_maze_from_file(level_path)?;

        // estado jugador nuevo en cada nivel
        let mut player = Player::new();

        // sprites aleatorios en el mapa
        loop {
            let x = rand::random::<f64>() * (maze::MAP_WIDTH as f64);
            let y = rand::random::<f64>() * (maze::MAP_HEIGHT as f64);
            if is_empty_cell(x, y) {
                sprite_renderer.add_sprite(Sprite { x, y, texture_index: 1 });
                break;
            }
        }

        let mut last_time = Instant::now();
        let mut victoria = false;
        let mut inicio = i == 0; // solo en el primer nivel
        let mut instrucciones = false;

        // ------ Pantalla de "Nivel X" ------
        if let Some(level_img) = &level_images[i] {
            'level_screen: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } 
                        | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(()),
                        Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                            break 'level_screen;
                        }
                        _ => {}
                    }
                }

            canvas.copy(level_img, None, Some(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)))?;
            canvas.present();
            std::thread::sleep(Duration::from_millis(16));
            }
        }

        // ------ Bucle principal del nivel ------
        'level_loop: loop {
            let now = Instant::now();
            let delta_time = now.duration_since(last_time).as_secs_f64();
            last_time = now;

            let plane_x = -player.dir_angle.sin() * (FOV / 2.0).tan();
            let plane_y = player.dir_angle.cos() * (FOV / 2.0).tan();

            // Eventos
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } 
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(()),
                    Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                        if inicio {
                            inicio = false;
                            instrucciones = true;
                        } else if instrucciones {
                            instrucciones = false; // iniciar juego
                        } else if victoria {
                            break 'level_loop; // pasar al siguiente nivel
                        }
                    },
                    _ => {}
                }
            }

            // Pantalla inicio
            if inicio {
                canvas.copy(&inicio_image, None, Some(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)))?;
                canvas.present();
                std::thread::sleep(Duration::from_millis(16));
                continue;
            }

            // Pantalla instrucciones
            if instrucciones {
                canvas.copy(&instrucciones_image, None, Some(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)))?;
                canvas.present();
                std::thread::sleep(Duration::from_millis(16));
                continue;
            }

            // Movimiento jugador
            let keys: Vec<Keycode> = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();
            player.update_position(&keys, delta_time);

            let mouse_state = event_pump.relative_mouse_state();
            player.rotate(mouse_state.x());

            // Comprobar meta (casilla 5 = salida)
            let mut picked_album = false;
            unsafe {
                if maze::MAP[player.y as usize][player.x as usize] == 5 && !picked_album {
                    // reproducir sonido
                    sdl2::mixer::Channel::all().play(&pickup_sound, 0)?;

                    // agregar sprite de album
                    sprite_renderer.add_sprite(Sprite {
                        x: player.x as f64 + 0.5,
                        y: player.y as f64 + 0.5,
                        texture_index: 0,
                    });

                    picked_album = true;
                    victoria = true;
                }
            }

            if victoria {
                // Mostrar victoria solo si es el último nivel
                if i == levels.len() - 1 {
                    canvas.copy(&victoria_image, None, Some(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)))?;
                    canvas.present();
                    std::thread::sleep(Duration::from_secs(2));
                } else {
                    // Sonido de victoria para niveles intermedios sin mostrar la pantalla
                    sdl2::mixer::Channel::all().play(&pickup_sound, 0)?;
                }
                break 'level_loop;
            }


            // Render juego
            draw_background(&mut canvas, SCREEN_WIDTH, SCREEN_HEIGHT)?;
            render_scene(&mut canvas, &player, &wall_textures, SCREEN_WIDTH, SCREEN_HEIGHT)?;
            sprite_renderer.draw_sprites(
                &mut canvas,
                player.x, player.y,
                player.dir_angle, plane_x, plane_y,
                SCREEN_WIDTH, SCREEN_HEIGHT,
            )?;
            draw_minimap(&mut canvas, &player, SCREEN_WIDTH)?;
            let fps = (1.0 / delta_time) as i32;
            draw_fps(&mut canvas, &font, &texture_creator, fps)?;
            canvas.present();
            std::thread::sleep(Duration::from_millis(65));
        }
    }

    println!("¡Has completado todos los niveles!");
    Ok(())
}
