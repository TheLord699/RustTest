use image::GenericImageView;

#[derive(Clone)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

impl Sprite {
    pub fn load(path: &str) -> Self {
        let img = image::open(path).expect("Failed to open image");
        let (width, height) = img.dimensions();
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                let p = img.get_pixel(x, y);
                let r = p[0];
                let g = p[1];
                let b = p[2];
                let a = p[3];
                let color = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
                pixels.push(color);
            }
        }

        Sprite { width, height, pixels }
    }

    pub fn scale(&self, scale: u32) -> Self {
        let new_width = self.width * scale;
        let new_height = self.height * scale;
        let mut pixels = vec![0; (new_width * new_height) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixels[(y * self.width + x) as usize];
                if (color >> 24) == 0 { continue; }

                for dy in 0..scale {
                    for dx in 0..scale {
                        let sx = x * scale + dx;
                        let sy = y * scale + dy;
                        pixels[(sy * new_width + sx) as usize] = color;
                    }
                }
            }
        }

        Sprite { width: new_width, height: new_height, pixels }
    }

    pub fn draw(
        &self,
        buffer: &mut [u32],
        buffer_width: usize,
        buffer_height: usize,
        center_x: i32,
        center_y: i32,
    ) {
        let half_width = (self.width / 2) as i32;
        let half_height = (self.height / 2) as i32;

        for y in 0..self.height as i32 {
            let buffer_row = center_y - half_height + y;
            if buffer_row < 0 || buffer_row >= buffer_height as i32 { continue; }

            let base_index = buffer_row as usize * buffer_width;
            for x in 0..self.width as i32 {
                let buffer_col = center_x - half_width + x;
                if buffer_col < 0 || buffer_col >= buffer_width as i32 { continue; }

                let color = self.pixels[(y as u32 * self.width + x as u32) as usize];
                if (color >> 24) == 0 { continue; }

                buffer[base_index + buffer_col as usize] = color;
            }
        }
    }
}
