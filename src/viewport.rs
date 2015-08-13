use cli::cli_constants::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub struct Viewport {
    width: usize,
    height: usize,
}

pub struct ViewportBuilder {
    width: usize,
    height: usize,
}

impl ViewportBuilder {
    pub fn new() -> ViewportBuilder {
        ViewportBuilder { width: SCREEN_WIDTH, height: SCREEN_HEIGHT }
    }

    pub fn finalize(&self) -> Viewport {
        Viewport { width: self.width, height: self.height }
    }
}
