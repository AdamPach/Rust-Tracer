pub struct Width(pub f32);

pub struct Height(pub f32);

pub struct WindowSize{
    height: Height,
    width: Width,
}

impl WindowSize{
    pub fn new(width: Width, height: Height) -> Self{
        WindowSize{
            height, width
        }
    }
    
    pub fn get_size(&self) -> [f32;2]{
        [self.width.0, self.height.0]
    }
    
    pub fn get_width(&self) -> f32{
        self.width.0
    }
    
    pub fn get_height(&self) -> f32{
        self.height.0
    }
}