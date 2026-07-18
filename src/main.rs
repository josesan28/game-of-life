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
const DEAD_COLOR: Color = Color::BLACK;
const ALIVE_COLOR: Color = Color::WHITE;

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        DEAD_COLOR,
    );

    let mut game = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT);

    add_glider(&mut framebuffer, 10, 10, ALIVE_COLOR);
    add_blinker(&mut framebuffer, 30, 30, ALIVE_COLOR);
    add_toad(&mut framebuffer, 50, 20, ALIVE_COLOR);
    add_beacon(&mut framebuffer, 70, 40, ALIVE_COLOR);

    let mut frame_delay = Duration::from_millis(200);
    let mut paused = false;
    let mut generation = 0;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
            println!("{}", if paused { "Pausado" } else { "Reanudado" });
        }
        if window.is_key_pressed(KeyboardKey::KEY_EQUAL) || window.is_key_pressed(KeyboardKey::KEY_KP_ADD) {
            let ms = frame_delay.as_millis() as u64;
            if ms > 16 {
                frame_delay = Duration::from_millis(ms - 16);
                println!("Velocidad: {} fps", 1000 / frame_delay.as_millis());
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_MINUS) || window.is_key_pressed(KeyboardKey::KEY_KP_SUBTRACT) {
            let ms = frame_delay.as_millis() as u64;
            if ms < 1000 {
                frame_delay = Duration::from_millis(ms + 16);
                println!("Velocidad: {} fps", 1000 / frame_delay.as_millis());
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_S) {
            framebuffer.render_to_file(format!("screenshot_gen_{}.png", generation).as_str());
            println!("Screenshot guardado como screenshot_gen_{}.png", generation);
        }
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            println!("Reiniciando...");
            framebuffer.clear();
            add_glider(&mut framebuffer, 10, 10, ALIVE_COLOR);
            add_blinker(&mut framebuffer, 30, 30, ALIVE_COLOR);
            add_toad(&mut framebuffer, 50, 20, ALIVE_COLOR);
            add_beacon(&mut framebuffer, 70, 40, ALIVE_COLOR);
            generation = 0;
        }

        if !paused {
            game.next_generation(&mut framebuffer, ALIVE_COLOR, DEAD_COLOR);
            generation += 1;
            if generation % 10 == 0 {
                println!("Generación {}", generation);
            }
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(frame_delay);
    }
}