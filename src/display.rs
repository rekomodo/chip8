use minifb::{Key, Window, WindowOptions};

const DISPLAY_SCALE: usize = 10;

pub struct Display {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        let mut window = Window::new(
            "CHIP8 @rekomodo",
            width * DISPLAY_SCALE,
            height * DISPLAY_SCALE,
            WindowOptions::default(),
        )
        .unwrap();

        window.set_target_fps(60);

        Display {
            window,
            buffer: vec![0u32; width * height],
            width: width,
            height: height,
        }
    }

    pub fn update_display(&mut self) {
        let mut buffer_lines = vec![];
        for b in self.buffer.iter() {
            buffer_lines.extend((0..DISPLAY_SCALE).map(|_| if *b > 0 { 0xFFFFFFFF } else { 0u32 }))
        }

        let mut buffer = vec![];
        for i in 0..self.height {       
            for _ in 0..DISPLAY_SCALE {
                let line_width = self.width * DISPLAY_SCALE;
                buffer.extend_from_slice(&buffer_lines[i * line_width..(i + 1) * line_width]);
            }
        }

        self.window
            .update_with_buffer(
                &buffer,
                self.width * DISPLAY_SCALE,
                self.height * DISPLAY_SCALE,
            )
            .unwrap();
    }

    pub fn xor_pixel(&mut self, row: usize, col: usize, val: u32) {
        assert!((0..self.width).contains(&col));
        assert!((0..self.height).contains(&row));

        let idx = row * self.width + col;
        self.buffer[idx] ^= val;
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0u32);
    }
}
