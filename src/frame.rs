pub struct Frame {
    pub pixels: [[(u8, u8, u8); Frame::HEIGHT]; Frame::WIDTH],
    //pub frame_finished: bool, // Poor mans callback
    //pub on_pixel_set: Option<Box<dyn FnMut(usize, usize, (u8, u8, u8))>>,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            pixels: [[(0, 0, 0); Frame::HEIGHT]; Frame::WIDTH],
            //frame_finished: false,
            //on_pixel_set: None,
        }
    }
}

impl Frame {
    pub const WIDTH: usize = 256;
    pub const HEIGHT: usize = 240;

    //Method to register the callback
    // pub fn set_on_pixel_set<F>(&mut self, callback: F)
    // where
    //     F: FnMut(usize, usize, (u8, u8, u8)) + 'static,
    // {
    //     self.on_pixel_set = Some(Box::new(callback));
    // }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        self.pixels[x - 1][y] = color;
        //if x == 256 && y == 240 {
        //    self.frame_finished = true;
        //}
        //if let Some(ref mut callback) = self.on_pixel_set {
        //    callback(x, y, color);
        //}
    }
}
