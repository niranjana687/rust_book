pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl<T> Screen<T>  
    where 
      T: Draw,
      {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: u32,
}

impl Draw for Button {
    fn draw(&self) {
        
    }
}