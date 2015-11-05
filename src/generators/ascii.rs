use ::sym::Encode;

pub struct ASCII {
    pub height: u32,
    pub xdim: u32,
    pub ydim: u32,
}

impl ASCII {
    pub fn new() -> ASCII {
        ASCII{height: 10, xdim: 1, ydim: 1}
    }

    pub fn height(mut self, h: u32) -> ASCII {
        self.height = h;
        self
    }

    pub fn xdim(mut self, x: u32) -> ASCII {
        self.xdim = x;
        self
    }

    pub fn ydim(mut self, y: u32) -> ASCII {
        self.ydim = y;
        self
    }

    pub fn generate<T: Encode>(&self, barcode: &T) -> String {
        "SWAG".to_string()
    }
}