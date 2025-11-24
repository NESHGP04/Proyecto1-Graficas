// framebuffer.rs
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::ttf::Font;
use crate::player::Player;
use crate::maze;

pub fn draw_background(canvas: &mut WindowCanvas, width: u32, height: u32) -> Result<(), String> {
    // Cielo (mitad superior)
    canvas.set_draw_color(Color::RGB(135, 206, 235)); // Azul cielo
    canvas.fill_rect(Rect::new(0, 0, width, height / 2))?;
    
    // Piso (mitad inferior)
    canvas.set_draw_color(Color::RGB(101, 67, 33)); // Marrón
    canvas.fill_rect(Rect::new(0, (height / 2) as i32, width, height / 2))?;
    
    Ok(())
}

pub fn draw_minimap(canvas: &mut WindowCanvas, player: &Player, screen_width: u32) -> Result<(), String> {
    let minimap_size = 200; // Más grande para ver mejor
    let cell_size = minimap_size / maze::MAP_WIDTH as i32;
    let minimap_x = (screen_width as i32 - minimap_size - 10) as i32;
    let minimap_y = 10;
    
    // Fondo del minimapa
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 200));
    canvas.fill_rect(Rect::new(minimap_x, minimap_y, minimap_size as u32, minimap_size as u32))?;
    
    // Dibujar el mapa
    unsafe {
        for y in 0..maze::MAP_HEIGHT {
            for x in 0..maze::MAP_WIDTH {
                let cell_x = minimap_x + (x as i32 * cell_size);
                let cell_y = minimap_y + (y as i32 * cell_size);
                
                let color = match maze::MAP[y][x] {
                    0 => Color::RGB(40, 40, 40),     // Vacío (oscuro)
                    5 => Color::RGB(255, 215, 0),    // Meta (dorado brillante)
                    _ => Color::RGB(180, 180, 180),  // Pared (gris claro)
                };
                
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(cell_x, cell_y, cell_size as u32, cell_size as u32))?;
                
                // Borde para la meta para que resalte más
                if maze::MAP[y][x] == 5 {
                    canvas.set_draw_color(Color::RGB(255, 100, 0)); // Naranja
                    canvas.draw_rect(Rect::new(cell_x, cell_y, cell_size as u32, cell_size as u32))?;
                }
            }
        }
    }
    
    // Dibujar jugador (más grande y visible)
    let player_x = minimap_x + (player.x * cell_size as f64) as i32;
    let player_y = minimap_y + (player.y * cell_size as f64) as i32;
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(Rect::new(player_x - 3, player_y - 3, 6, 6))?;
    
    // Dirección del jugador
    let dir_length = 12;
    let end_x = player_x + (player.dir_angle.cos() * dir_length as f64) as i32;
    let end_y = player_y + (player.dir_angle.sin() * dir_length as f64) as i32;
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.draw_line((player_x, player_y), (end_x, end_y))?;
    
    Ok(())
}

pub fn draw_fps(
    canvas: &mut WindowCanvas,
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
    fps: i32,
) -> Result<(), String> {
    let fps_text = format!("FPS: {}", fps);
    let surface = font
        .render(&fps_text)
        .blended(Color::RGB(255, 255, 0))
        .map_err(|e| e.to_string())?;
    
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    
    let target = Rect::new(10, 10, surface.width(), surface.height());
    canvas.copy(&texture, None, Some(target))?;
    
    Ok(())
}