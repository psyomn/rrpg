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
        ViewportBuilder { width: 0, height: 0 }
    }

    pub fn default_x(&self) -> ViewportBuilder {
        ViewportBuilder { width: SCREEN_WIDTH, height: self.height }
    }

    pub fn default_y(&self) -> ViewportBuilder {
        ViewportBuilder { width: self.width, height: SCREEN_HEIGHT }
    }

    pub fn finalize(&self) -> Viewport {
        Viewport { width: self.width, height: self.height }
    }
}
