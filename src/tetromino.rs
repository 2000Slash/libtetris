use crate::shape::Shape;



pub struct Tetromino {
    shape: Shape,
    pos_x: i32,
    pos_y: i32,
    rotation: i32,
}

impl Tetromino {
    pub fn get_collision(&self) -> Vec<Vec<i32>> {
        self.shape.get_collision(self.rotation)
    }

    pub fn move_left(&mut self) {
        self.pos_x -= 1;
    }

    pub fn move_right(&mut self) {
        self.pos_x += 1;
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn move_down(&mut self) {
        self.pos_y += 1;   
    }
}