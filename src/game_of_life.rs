use crate::framebuffer::Framebuffer;
use raylib::prelude::Color;

pub struct GameOfLife {
    width: u32,
    height: u32,
    next: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> Self {
        let next = vec![vec![false; width as usize]; height as usize];
        GameOfLife { width, height, next }
    }

    pub fn next_generation(&mut self, framebuffer: &mut Framebuffer, color_alive: Color, color_dead: Color) {
        for row in self.next.iter_mut() {
            for cell in row.iter_mut() {
                *cell = false;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let live_neighbors = self.count_live_neighbors(framebuffer, x, y, color_alive);
                let is_alive = self.is_alive(framebuffer, x, y, color_alive);

                let new_state = match (is_alive, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
                self.next[y as usize][x as usize] = new_state;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let color = if self.next[y as usize][x as usize] {
                    color_alive
                } else {
                    color_dead
                };
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x, y);
            }
        }
    }

    fn count_live_neighbors(&self, framebuffer: &Framebuffer, x: u32, y: u32, color_alive: Color) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0
                    && nx < self.width as i32
                    && ny >= 0
                    && ny < self.height as i32
                    && self.is_alive(framebuffer, nx as u32, ny as u32, color_alive)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_alive(&self, framebuffer: &Framebuffer, x: u32, y: u32, color_alive: Color) -> bool {
        framebuffer.get_color(x, y) == Some(color_alive)
    }
}
