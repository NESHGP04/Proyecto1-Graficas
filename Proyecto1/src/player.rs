// player.rs
use sdl2::keyboard::Keycode;
use crate::maze;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub dir_angle: f64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 1.5,
            y: 1.5,
            dir_angle: 0.0,
        }
    }
    
    pub fn update_position(&mut self, keys: &[Keycode], delta_time: f64) {
        let move_speed = 3.0 * delta_time;
        let mut new_x = self.x;
        let mut new_y = self.y;
        
        for key in keys {
            match key {
                Keycode::W | Keycode::Up => {
                    new_x += self.dir_angle.cos() * move_speed;
                    new_y += self.dir_angle.sin() * move_speed;
                }
                Keycode::S | Keycode::Down => {
                    new_x -= self.dir_angle.cos() * move_speed;
                    new_y -= self.dir_angle.sin() * move_speed;
                }
                Keycode::A => {
                    new_x += self.dir_angle.sin() * move_speed;
                    new_y -= self.dir_angle.cos() * move_speed;
                }
                Keycode::D => {
                    new_x -= self.dir_angle.sin() * move_speed;
                    new_y += self.dir_angle.cos() * move_speed;
                }
                _ => {}
            }
        }
        
        // Collision detection
        if !self.is_wall(new_x, self.y) {
            self.x = new_x;
        }
        if !self.is_wall(self.x, new_y) {
            self.y = new_y;
        }
    }
    
    pub fn rotate(&mut self, mouse_delta: i32) {
        let rotation_speed = 0.002;
        self.dir_angle += mouse_delta as f64 * rotation_speed;
    }
    
    fn is_wall(&self, x: f64, y: f64) -> bool {
        let map_x = x as usize;
        let map_y = y as usize;
        
        if map_x >= maze::MAP_WIDTH || map_y >= maze::MAP_HEIGHT {
            return true;
        }
        
        unsafe {
            maze::MAP[map_y][map_x] != 0 && maze::MAP[map_y][map_x] != 5
        }
    }
}