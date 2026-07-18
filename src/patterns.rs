use crate::framebuffer::Framebuffer;
use raylib::prelude::Color;

fn draw_live_cell(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    framebuffer.set_current_color(color_alive);
    framebuffer.set_pixel(x, y);
}

pub fn add_glider(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    let coords = [(1,0), (2,1), (0,2), (1,2), (2,2)];
    for (dx, dy) in coords.iter() {
        draw_live_cell(framebuffer, x + dx, y + dy, color_alive);
    }
}

pub fn add_blinker(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    for i in 0..3 {
        draw_live_cell(framebuffer, x + i, y, color_alive);
    }
}

pub fn add_toad(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    let coords = [(1,0), (2,0), (3,0), (0,1), (1,1), (2,1)];
    for (dx, dy) in coords.iter() {
        draw_live_cell(framebuffer, x + dx, y + dy, color_alive);
    }
}

pub fn add_beacon(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    let coords = [
        (0,0), (1,0), (0,1), (1,1),
        (2,2), (3,2), (2,3), (3,3)
    ];
    for (dx, dy) in coords.iter() {
        draw_live_cell(framebuffer, x + dx, y + dy, color_alive);
    }
}