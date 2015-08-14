use cli::cli_constants::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub struct Viewport {
    width: usize,
    height: usize,
}

impl Viewport {
    pub fn new() -> Viewport { Viewport {width: 0, height: 0} }

    pub fn width(&self) -> usize { self.width }

    pub fn height(&self) -> usize { self.height }
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

    pub fn x(&self, xx: usize) -> ViewportBuilder {
        ViewportBuilder { width: xx, height: self.height }
    }

    pub fn y(&self, yy: usize) -> ViewportBuilder {
        ViewportBuilder { width: self.width, height: yy }
    }
}
