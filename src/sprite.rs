use image::GenericImageView;

#[derive(Clone)]
pub struct Sprite {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixels: Vec<u32>,
}

impl Sprite {
    pub fn load(path: &str) -> Self {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(_) => image::open("assets/sprites/missing_texture.png").unwrap_or_else(|_| {
                image::DynamicImage::new_rgba8(16, 16)
            }),
        };
        
        let (width, height) = img.dimensions();
        let mut pixels = Vec::with_capacity((width * height) as usize);
        
        for pixel in img.pixels() {
            let (_, _, p) = pixel;
            let r = p[0];
            let g = p[1];
            let b = p[2];
            let a = p[3];
            pixels.push(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | b as u32);
        }
        
        Sprite { width, height, pixels }
    }
    
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    
    pub fn scale(&self, scale: u32) -> Self {
        if scale == 1 { return self.clone(); }
        
        let new_width = self.width * scale;
        let new_height = self.height * scale;
        let mut pixels = vec![0; (new_width * new_height) as usize];
        let old_pixels = &self.pixels;
        let old_width = self.width;
        
        for (idx, &color) in old_pixels.iter().enumerate() {
            if (color >> 24) == 0 { continue; }
            let x = (idx as u32) % old_width;
            let y = (idx as u32) / old_width;
            let base_y = y * scale * new_width;
            
            for dy in 0..scale {
                let row_start = (base_y + dy * new_width + x * scale) as usize;
                for dx in 0..scale {
                    pixels[row_start + dx as usize] = color;
                }
            }
        }
        
        Sprite { width: new_width, height: new_height, pixels }
    }
    
    pub fn draw(&self, buffer: &mut [u32], buf_w: usize, buf_h: usize, cx: i32, cy: i32) {
        let hw = (self.width / 2) as i32;
        let hh = (self.height / 2) as i32;
        let start_y = (cy - hh).max(0) as usize;
        let end_y = ((cy + hh).min(buf_h as i32 - 1) as usize).min(buf_h);
        let start_x = (cx - hw).max(0) as i32;
        
        for y in start_y..end_y {
            let src_y = (y as i32 - (cy - hh)) as u32;
            let buf_idx = y * buf_w;
            let src_row = (src_y * self.width) as usize;
            
            for x in start_x..(cx + hw) as i32 {
                if x >= buf_w as i32 { break; }
                let src_x = (x - (cx - hw)) as u32;
                let color = self.pixels[src_row + src_x as usize];
                if color >> 24 != 0 {
                    buffer[buf_idx + x as usize] = color;
                }
            }
        }
    }
    
    pub fn draw_flipped(&self, buffer: &mut [u32], buf_w: usize, buf_h: usize, cx: i32, cy: i32) {
        let hw = (self.width / 2) as i32;
        let hh = (self.height / 2) as i32;
        let start_y = (cy - hh).max(0) as usize;
        let end_y = ((cy + hh).min(buf_h as i32 - 1) as usize).min(buf_h);
        let start_x = (cx - hw).max(0) as i32;
        let flip_offset = self.width as i32 - 1;
        
        for y in start_y..end_y {
            let src_y = (y as i32 - (cy - hh)) as u32;
            let buf_idx = y * buf_w;
            let src_row = (src_y * self.width) as usize;
            
            for x in start_x..(cx + hw) as i32 {
                if x >= buf_w as i32 { break; }
                let src_x = flip_offset - (x - (cx - hw));
                let color = self.pixels[src_row + src_x as usize];
                if color >> 24 != 0 {
                    buffer[buf_idx + x as usize] = color;
                }
            }
        }
    }
}