// maze.rs
use std::fs::File;
use std::io::{BufRead, BufReader};

pub const MAP_WIDTH: usize = 16;
pub const MAP_HEIGHT: usize = 16;

pub static mut MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [[0; MAP_WIDTH]; MAP_HEIGHT];

pub fn load_maze_from_file(filename: &str) -> Result<(), String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    
    unsafe {
        MAP = [[0; MAP_WIDTH]; MAP_HEIGHT];
        
        for (y, line) in reader.lines().enumerate() {
            if y >= MAP_HEIGHT {
                break;
            }
            
            let line = line.map_err(|e| e.to_string())?;
            for (x, ch) in line.chars().enumerate() {
                if x >= MAP_WIDTH {
                    break;
                }
                
                MAP[y][x] = match ch {
                    '0' | ' ' => 0,  // espacio vacío
                    '5' => 5,  // meta/salida (ANTES de '1'..='9')
                    '1'..='9' => ch.to_digit(10).unwrap() as u8,  // paredes con textura
                    _ => 0,
                };
            }
        }
    }
    
    Ok(())
}