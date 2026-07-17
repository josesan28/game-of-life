use raylib::prelude::*;
use raylib::consts::TraceLogLevel;
use std::thread;
use std::time::Duration;

mod framebuffer;
use framebuffer::Framebuffer;

mod line;
use line::line;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Movimiento")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, Color::new(50, 50, 100, 255));
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let mut translate_x = 0.0;
    let mut translate_y = 0.0;
    let mut speed = 2.0;
    let mut direction_x = 1.0;
    let mut direction_y = 1.0;

    // Variable para screenshot
    let mut screenshot_counter = 1;

    // Variables para FPS
    //let mut frame_count = 0;
    //let mut fps_timer = std::time::Instant::now();

    while !window.window_should_close() {
        // Limpiar framebuffer
        framebuffer.clear();

        // Actualizar posición según dirección y velocidad
        translate_x += speed * direction_x;
        translate_y += speed * direction_y;

        // Rebotar en los bordes
        let margin = 50.0;
        let object_width = 300.0;
        let object_height = 300.0;
        let left = margin + translate_x;
        let right = margin + translate_x + object_width;
        let top = margin + translate_y;
        let bottom = margin + translate_y + object_height;

        if right >= window_width as f32 || left <= 0.0 {
            direction_x *= -1.0;
        }
        if bottom >= window_height as f32 || top <= 0.0 {
            direction_y *= -1.0;
        }

        render(&mut framebuffer, translate_x, translate_y);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        if window.is_key_pressed(KeyboardKey::KEY_S) {
            let filename = format!("screenshot_{:03}.png", screenshot_counter);
            framebuffer.render_to_file(&filename);
            println!("Screenshot guardado: {}", filename);
            screenshot_counter += 1;
        }

        // Control de FPS
        thread::sleep(Duration::from_millis(16));
    }
}

fn render(
    framebuffer: &mut Framebuffer,
    translate_x: f32,
    translate_y: f32,
) {
    framebuffer.set_current_color(Color::GREEN);
    line(
        framebuffer,
        Vector2::new(50.0 + translate_x, 50.0 + translate_y),
        Vector2::new(350.0 + translate_x, 350.0 + translate_y),
    );
    framebuffer.set_current_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(350.0 + translate_x, 50.0 + translate_y),
        Vector2::new(50.0 + translate_x, 350.0 + translate_y),
    );
}