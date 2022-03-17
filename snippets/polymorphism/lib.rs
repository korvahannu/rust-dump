
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Components hold a vector of any time that implements the trait Draw
    // Box is used because we do not know the specific size of the object inside vector
    // dyn for dynamic
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button");
    }
}