use crate::{shape::Shape, Randomizer};



#[derive(Debug)]
pub struct Tetromino {
    shape: Shape,
    pub pos_x: i32,
    pub pos_y: i32,
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

    pub fn get_color(&self) -> i32 {
        match self.shape {
            Shape::I => 1,
            Shape::J => 2,
            Shape::L => 3,
            Shape::O => 4,
            Shape::S => 5,
            Shape::T => 6,
            Shape::Z => 7,
        }
    }

    pub fn create(random: &mut Box<dyn Randomizer>) -> Tetromino {
        Tetromino {
            shape: (random.next() % 7).into(),
            pos_x: 4,
            pos_y: 0,
            rotation: 0
        }
    }
}