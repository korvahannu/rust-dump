

pub struct Square {
    pub width : usize,
    pub height: usize
}

impl Square {
    pub fn area(&self) -> usize {
        self.width * self.height
    }
}