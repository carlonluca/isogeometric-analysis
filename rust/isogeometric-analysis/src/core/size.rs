pub struct Size {
    pub width: u32,
    pub height: u32
}

impl Size {
    pub fn is_empty(&self) -> bool {
        if self.width <= 0 || self.height <= 0 { true } else { false }
    }
}
