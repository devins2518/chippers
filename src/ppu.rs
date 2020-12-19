pub struct PPU {
    context: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl PPU {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();

        let window = video
            .window("chippers", 64, 32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        PPU {
            context,
            video,
            canvas,
        }
    }

    // Clear screen
    pub fn CLS(&mut self) {
        self.canvas.clear();
    }
}
