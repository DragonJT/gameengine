#[derive(Debug)]
pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub channels: i32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn new(width: i32, height: i32, channels: i32) -> Self {
        let size = (width * height * channels) as usize;

        Texture {
            width,
            height,
            channels,
            data: vec![0; size],
        }
    }

    fn index(&self, x: i32, y: i32) -> usize {
        ((y * self.width + x) * self.channels) as usize
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: &[u8]) {
        let r2 = radius * radius;
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= r2 {
                    let x = cx + dx;
                    let y = cy + dy;
                    self.set_pixel(x, y, color);
                }
            }
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: &[u8]) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let idx = self.index(x, y);

        for c in 0..self.channels as usize {
            self.data[idx + c] = color[c];
        }
    }
}
