mod framebuffer;
mod game_of_life;
mod line;
mod patterns;

use raylib::consts::TraceLogLevel;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use patterns::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const GRID_WIDTH: u32 = 100;
const GRID_HEIGHT: u32 = 100;
const DEAD_COLOR: Color = Color::new(7, 18, 38, 255);
const ALIVE_COLOR: Color = Color::new(89, 255, 214, 255);

fn seed_patterns(framebuffer: &mut Framebuffer) {
    add_pulsar(framebuffer, 6, 2, ALIVE_COLOR);
    add_pulsar(framebuffer, 81, 2, ALIVE_COLOR);
    add_pulsar(framebuffer, 6, 85, ALIVE_COLOR);
    add_pulsar(framebuffer, 81, 85, ALIVE_COLOR);

    add_gosper_glider_gun(framebuffer, 2, 17, ALIVE_COLOR);
    add_reverse_gosper_glider_gun(framebuffer, 62, 74, ALIVE_COLOR);

    add_lightweight_spaceship_right(framebuffer, 26, 34, ALIVE_COLOR);
    add_lightweight_spaceship(framebuffer, 69, 34, ALIVE_COLOR);
    add_lightweight_spaceship_right(framebuffer, 26, 63, ALIVE_COLOR);
    add_lightweight_spaceship(framebuffer, 69, 63, ALIVE_COLOR);

    add_glider(framebuffer, 39, 39, ALIVE_COLOR);
    add_glider_rotated(framebuffer, 58, 39, ALIVE_COLOR, 1);
    add_glider_rotated(framebuffer, 58, 58, ALIVE_COLOR, 2);
    add_glider_rotated(framebuffer, 39, 58, ALIVE_COLOR, 3);
    add_r_pentomino(framebuffer, 49, 49, ALIVE_COLOR);

    add_beacon(framebuffer, 21, 48, ALIVE_COLOR);
    add_beacon(framebuffer, 75, 48, ALIVE_COLOR);
    add_acorn(framebuffer, 27, 71, ALIVE_COLOR);
    add_acorn(framebuffer, 66, 26, ALIVE_COLOR);
    add_blinker(framebuffer, 46, 27, ALIVE_COLOR);
    add_toad(framebuffer, 47, 70, ALIVE_COLOR);
}

fn main() {
    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT, DEAD_COLOR);

    let mut game = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT);

    seed_patterns(&mut framebuffer);

    let mut frame_delay = Duration::from_millis(200);
    let mut paused = false;
    let mut generation = 0;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
            println!("{}", if paused { "Pausado" } else { "Reanudado" });
        }
        if window.is_key_pressed(KeyboardKey::KEY_EQUAL)
            || window.is_key_pressed(KeyboardKey::KEY_KP_ADD)
        {
            let ms = frame_delay.as_millis() as u64;
            if ms > 16 {
                frame_delay = Duration::from_millis(ms - 16);
                println!("Velocidad: {} fps", 1000 / frame_delay.as_millis());
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_MINUS)
            || window.is_key_pressed(KeyboardKey::KEY_KP_SUBTRACT)
        {
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
            seed_patterns(&mut framebuffer);
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