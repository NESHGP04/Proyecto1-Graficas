// caster.rs
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;
use crate::maze;
use crate::player::Player;

pub fn render_scene(
    canvas: &mut WindowCanvas,
    player: &Player,
    wall_textures: &[Texture],
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let num_rays = screen_width;
    let fov = std::f64::consts::PI / 3.0;
    
    for i in 0..num_rays {
        let camera_x = 2.0 * i as f64 / num_rays as f64 - 1.0;
        let ray_angle = player.dir_angle + camera_x * (fov / 2.0);
        
        let (distance, wall_type, hit_x) = cast_ray(player, ray_angle);
        
        if distance > 0.0 && distance < f64::INFINITY {
            // Corregir distancia por fish-eye
            let corrected_distance = distance * (ray_angle - player.dir_angle).cos();
            
            let wall_height = (screen_height as f64 / corrected_distance) as i32;
            let draw_start = ((screen_height as i32 - wall_height) / 2).max(0);
            let draw_end = ((screen_height as i32 + wall_height) / 2).min(screen_height as i32);
            
            if wall_type > 0 && wall_type <= wall_textures.len() as u8 {
                let texture = &wall_textures[(wall_type - 1) as usize];
                let tex_query = texture.query();
                let tex_x = (hit_x * tex_query.width as f64) as i32 % tex_query.width as i32;
                
                let src_rect = Rect::new(tex_x, 0, 1, tex_query.height);
                let dst_rect = Rect::new(
                    i as i32,
                    draw_start,
                    1,
                    (draw_end - draw_start) as u32
                );
                
                canvas.copy(texture, src_rect, dst_rect)?;
            }
        }
    }
    
    Ok(())
}

fn cast_ray(player: &Player, angle: f64) -> (f64, u8, f64) {
    let ray_dir_x = angle.cos();
    let ray_dir_y = angle.sin();
    
    let mut map_x = player.x as i32;
    let mut map_y = player.y as i32;
    
    let delta_dist_x = if ray_dir_x.abs() < 1e-10 { 
        f64::INFINITY 
    } else { 
        (1.0 / ray_dir_x).abs() 
    };
    
    let delta_dist_y = if ray_dir_y.abs() < 1e-10 { 
        f64::INFINITY 
    } else { 
        (1.0 / ray_dir_y).abs() 
    };
    
    let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
        (-1, (player.x - map_x as f64) * delta_dist_x)
    } else {
        (1, (map_x as f64 + 1.0 - player.x) * delta_dist_x)
    };
    
    let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
        (-1, (player.y - map_y as f64) * delta_dist_y)
    } else {
        (1, (map_y as f64 + 1.0 - player.y) * delta_dist_y)
    };
    
    let mut hit = false;
    let mut side = 0;
    let mut wall_type = 0u8;
    let max_iterations = 100; // Prevenir loops infinitos
    let mut iterations = 0;
    
    while !hit && iterations < max_iterations {
        iterations += 1;
        
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }
        
        // Verificar límites del mapa
        if map_x < 0 || map_x >= maze::MAP_WIDTH as i32 || 
           map_y < 0 || map_y >= maze::MAP_HEIGHT as i32 {
            return (f64::INFINITY, 0, 0.0);
        }
        
        unsafe {
            wall_type = maze::MAP[map_y as usize][map_x as usize];
            // Una pared es cualquier valor diferente de 0 y diferente de 5 (meta)
            if wall_type != 0 && wall_type != 5 {
                hit = true;
            }
        }
    }
    
    if !hit {
        return (f64::INFINITY, 0, 0.0);
    }
    
    let distance = if side == 0 {
        (map_x as f64 - player.x + (1.0 - step_x as f64) / 2.0) / ray_dir_x
    } else {
        (map_y as f64 - player.y + (1.0 - step_y as f64) / 2.0) / ray_dir_y
    };
    
    let wall_x = if side == 0 {
        player.y + distance * ray_dir_y
    } else {
        player.x + distance * ray_dir_x
    };
    
    let hit_x = wall_x - wall_x.floor();
    
    (distance.abs(), wall_type, hit_x)
}