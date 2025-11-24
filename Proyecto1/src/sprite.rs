// sprite.rs
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;
use crate::maze;

pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub texture_index: usize,
}

pub struct SpriteRenderer<'a> {
    textures: Vec<Texture<'a>>,
    sprites: Vec<Sprite>,
}

impl<'a> SpriteRenderer<'a> {
    pub fn new() -> Self {
        SpriteRenderer {
            textures: Vec::new(),
            sprites: Vec::new(),
        }
    }
    
    pub fn add_texture(&mut self, texture: Texture<'a>) {
        self.textures.push(texture);
    }
    
    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }
    
    pub fn draw_sprites(
        &self,
        canvas: &mut WindowCanvas,
        player_x: f64,
        player_y: f64,
        player_angle: f64,
        plane_x: f64,
        plane_y: f64,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), String> {
        let mut sprite_distances: Vec<(usize, f64)> = self.sprites
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let dx = s.x - player_x;
                let dy = s.y - player_y;
                let distance = dx * dx + dy * dy;
                (i, distance)
            })
            .collect();
        
        // Ordenar sprites por distancia (más lejanos primero)
        sprite_distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        let dir_x = player_angle.cos();
        let dir_y = player_angle.sin();
        
        for (idx, _) in sprite_distances {
            let sprite = &self.sprites[idx];
            
            let sprite_x = sprite.x - player_x;
            let sprite_y = sprite.y - player_y;
            
            let inv_det = 1.0 / (plane_x * dir_y - dir_x * plane_y);
            
            let transform_x = inv_det * (dir_y * sprite_x - dir_x * sprite_y);
            let transform_y = inv_det * (-plane_y * sprite_x + plane_x * sprite_y);
            
            if transform_y <= 0.0 {
                continue;
            }
            
            let sprite_screen_x = ((screen_width as f64 / 2.0) * (1.0 + transform_x / transform_y)) as i32;
            
            let sprite_height = ((screen_height as f64 / transform_y).abs()) as i32;
            let sprite_width = sprite_height;
            
            let draw_start_y = (-sprite_height / 2 + screen_height as i32 / 2).max(0);
            let draw_end_y = (sprite_height / 2 + screen_height as i32 / 2).min(screen_height as i32);
            
            let draw_start_x = (-sprite_width / 2 + sprite_screen_x).max(0);
            let draw_end_x = (sprite_width / 2 + sprite_screen_x).min(screen_width as i32);
            
            if sprite.texture_index < self.textures.len() {
                let texture = &self.textures[sprite.texture_index];
                
                let dst_rect = Rect::new(
                    draw_start_x,
                    draw_start_y,
                    (draw_end_x - draw_start_x) as u32,
                    (draw_end_y - draw_start_y) as u32,
                );
                
                canvas.copy(texture, None, Some(dst_rect))?;
            }
        }
        
        Ok(())
    }
}

pub fn is_empty_cell(x: f64, y: f64) -> bool {
    let map_x = x as usize;
    let map_y = y as usize;
    
    if map_x >= maze::MAP_WIDTH || map_y >= maze::MAP_HEIGHT {
        return false;
    }
    
    unsafe {
        maze::MAP[map_y][map_x] == 0
    }
}