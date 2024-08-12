use minifb::{Key, Window, WindowOptions};

const DISPLAY_SCALE: usize = 10;
const COLOR_ON: u32 = 0x00FFFFFF;
const COLOR_OFF: u32 = 0x00000000;

pub struct Display {
    window: Window,
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
            width: width,
            height: height,
        }
    }

    pub fn update_display(&mut self, buffer : &[u32]) {
        let mut buffer_lines = vec![];
        for b in buffer.iter() {
            buffer_lines.extend((0..DISPLAY_SCALE).map(|_| if *b > 0 { COLOR_ON } else { COLOR_OFF }))
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
}
