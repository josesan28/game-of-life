mod framebuffer;
mod line;
mod game_of_life;
mod patterns;

use raylib::prelude::*;
use raylib::consts::TraceLogLevel;
use std::time::Duration;
use std::thread;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use patterns::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const GRID_WIDTH: u32 = 100;
const GRID_HEIGHT: u32 = 100;

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        Color::new(0, 0, 0, 255), // fondo negro
    );

    let mut game = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT);

    // Cargar patrones
    add_glider(&mut game, 10, 10);
    add_blinker(&mut game, 30, 30);
    add_toad(&mut game, 50, 20);
    // Puedes añadir más

    let frame_delay = Duration::from_millis(100); // sin mut porque no cambia

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_S) {
            framebuffer.render_to_file("screenshot.png");
            println!("Screenshot guardado como screenshot.png");
        }

        game.next_generation();

        framebuffer.clear();
        game.render_to(&mut framebuffer, Color::WHITE, Color::BLACK);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(frame_delay);
    }
}