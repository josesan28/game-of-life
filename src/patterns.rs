use crate::framebuffer::Framebuffer;
use raylib::prelude::Color;

const GOSPER_GLIDER_GUN: [(u32, u32); 36] = [
    (24, 0),
    (22, 1),
    (24, 1),
    (12, 2),
    (13, 2),
    (20, 2),
    (21, 2),
    (34, 2),
    (35, 2),
    (11, 3),
    (15, 3),
    (20, 3),
    (21, 3),
    (34, 3),
    (35, 3),
    (0, 4),
    (1, 4),
    (10, 4),
    (16, 4),
    (20, 4),
    (21, 4),
    (0, 5),
    (1, 5),
    (10, 5),
    (14, 5),
    (16, 5),
    (17, 5),
    (22, 5),
    (24, 5),
    (10, 6),
    (16, 6),
    (24, 6),
    (11, 7),
    (15, 7),
    (12, 8),
    (13, 8),
];

fn draw_live_cell(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    framebuffer.set_current_color(color_alive);
    framebuffer.set_pixel(x, y);
}

fn draw_pattern(
    framebuffer: &mut Framebuffer,
    x: u32,
    y: u32,
    color_alive: Color,
    cells: &[(u32, u32)],
) {
    for &(dx, dy) in cells {
        draw_live_cell(framebuffer, x + dx, y + dy, color_alive);
    }
}

pub fn add_glider(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    add_glider_rotated(framebuffer, x, y, color_alive, 0);
}

pub fn add_glider_rotated(
    framebuffer: &mut Framebuffer,
    x: u32,
    y: u32,
    color_alive: Color,
    rotation: u8,
) {
    let cells = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    for &(dx, dy) in &cells {
        let (rx, ry) = match rotation % 4 {
            1 => (2 - dy, dx),
            2 => (2 - dx, 2 - dy),
            3 => (dy, 2 - dx),
            _ => (dx, dy),
        };
        draw_live_cell(framebuffer, x + rx, y + ry, color_alive);
    }
}

pub fn add_blinker(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    for i in 0..3 {
        draw_live_cell(framebuffer, x + i, y, color_alive);
    }
}

pub fn add_toad(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    let cells = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
    draw_pattern(framebuffer, x, y, color_alive, &cells);
}

pub fn add_beacon(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    let cells = [
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 2),
        (2, 3),
        (3, 3),
    ];
    draw_pattern(framebuffer, x, y, color_alive, &cells);
}

pub fn add_pulsar(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    let edges = [0, 5, 7, 12];
    let bars = [2, 3, 4, 8, 9, 10];

    for &edge in &edges {
        for &bar in &bars {
            draw_live_cell(framebuffer, x + edge, y + bar, color_alive);
            draw_live_cell(framebuffer, x + bar, y + edge, color_alive);
        }
    }
}

pub fn add_lightweight_spaceship(
    framebuffer: &mut Framebuffer,
    x: u32,
    y: u32,
    color_alive: Color,
) {
    let cells = [
        (1, 0),
        (4, 0),
        (0, 1),
        (0, 2),
        (4, 2),
        (0, 3),
        (1, 3),
        (2, 3),
        (3, 3),
    ];
    draw_pattern(framebuffer, x, y, color_alive, &cells);
}

pub fn add_lightweight_spaceship_right(
    framebuffer: &mut Framebuffer,
    x: u32,
    y: u32,
    color_alive: Color,
) {
    let cells = [
        (0, 0),
        (3, 0),
        (4, 1),
        (0, 2),
        (4, 2),
        (1, 3),
        (2, 3),
        (3, 3),
        (4, 3),
    ];
    draw_pattern(framebuffer, x, y, color_alive, &cells);
}

pub fn add_r_pentomino(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    let cells = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];
    draw_pattern(framebuffer, x, y, color_alive, &cells);
}

pub fn add_acorn(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    let cells = [(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)];
    draw_pattern(framebuffer, x, y, color_alive, &cells);
}

pub fn add_gosper_glider_gun(framebuffer: &mut Framebuffer, x: u32, y: u32, color_alive: Color) {
    draw_pattern(framebuffer, x, y, color_alive, &GOSPER_GLIDER_GUN);
}

pub fn add_reverse_gosper_glider_gun(
    framebuffer: &mut Framebuffer,
    x: u32,
    y: u32,
    color_alive: Color,
) {
    for &(dx, dy) in &GOSPER_GLIDER_GUN {
        draw_live_cell(framebuffer, x + 35 - dx, y + 8 - dy, color_alive);
    }
}