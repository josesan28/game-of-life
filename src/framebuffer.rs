use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width as i32,
            self.height as i32,
            self.background_color,
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn get_color(&self, x: u32, y: u32) -> Option<Color> {
        if x < self.width && y < self.height {
            Some(self.color_buffer.get_color(x as i32, y as i32))
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&mut self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let win_w = window.get_screen_width() as f32;
            let win_h = window.get_screen_height() as f32;

            let tex_w = texture.width as f32;
            let tex_h = texture.height as f32;

            let scale = (win_w / tex_w).min(win_h / tex_h);
            let offset_x = (win_w - tex_w * scale) / 2.0;
            let offset_y = (win_h - tex_h * scale) / 2.0;

            let mut render = window.begin_drawing(raylib_thread);

            render.draw_texture_ex(
                &texture,
                Vector2::new(offset_x, offset_y),
                0.0,
                scale,
                Color::WHITE,
            );
        }
    }
}