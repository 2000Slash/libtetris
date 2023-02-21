#[derive(Debug, Clone, Copy)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

impl Shape {
    pub fn get_collision(&self, rotation: i32) -> Vec<Vec<i32>> {
        match rotation%4 {
            0 => self.get_collision_0(),
            1 => self.get_collision_1(),
            2 => self.get_collision_2(),
            3 => self.get_collision_3(),
            _ => self.get_collision_0(),
        }
    }

    fn get_collision_0(&self) -> Vec<Vec<i32>> {
        match self {
            Shape::I => vec![vec![0, 0, 0, 0],
                             vec![1, 1, 1, 1],
                             vec![0, 0, 0, 0],
                             vec![0, 0, 0, 0]],
            Shape::O => vec![vec![1, 1],
                             vec![1, 1]],
            Shape::T => vec![vec![0, 1, 0],
                             vec![1, 1, 1],
                             vec![0, 0, 0]],
            Shape::J => vec![vec![1, 0, 0],
                             vec![1, 1, 1],
                             vec![0, 0, 0]],
            Shape::L => vec![vec![0, 0, 1],
                             vec![1, 1, 1],
                             vec![0, 0, 0]],
            Shape::S => vec![vec![0, 1, 1],
                             vec![1, 1, 0],
                             vec![0, 0, 0]],
            Shape::Z => vec![vec![1, 1, 0],
                             vec![0, 1, 1],
                             vec![0, 0, 0]],
        }
    }

    fn get_collision_1(&self) -> Vec<Vec<i32>> {
        match self {
            Shape::I => vec![vec![0, 0, 1, 0],
                             vec![0, 0, 1, 0],
                             vec![0, 0, 1, 0],
                             vec![0, 0, 1, 0]],
            Shape::O => self.get_collision_0(),
            Shape::T => vec![vec![0, 1, 0],
                             vec![0, 1, 1],
                             vec![0, 1, 0]],
            Shape::J => vec![vec![0, 1, 1],
                             vec![0, 1, 0],
                             vec![0, 1, 0]],
            Shape::L => vec![vec![0, 1, 0],
                             vec![0, 1, 0],
                             vec![0, 1, 1]],
            Shape::S => vec![vec![0, 1, 0],
                             vec![0, 1, 1],
                             vec![0, 0, 1]],
            Shape::Z => vec![vec![0, 0, 1],
                             vec![0, 1, 1],
                             vec![0, 1, 0]],
        }
    }

    fn get_collision_2(&self) -> Vec<Vec<i32>> {
        match self {
            Shape::I => vec![vec![0, 0, 0, 0],
                             vec![0, 0, 0, 0],
                             vec![1, 1, 1, 1],
                             vec![0, 0, 0, 0]],
            Shape::O => self.get_collision_0(),
            Shape::T => vec![vec![0, 0, 0],
                             vec![1, 1, 1],
                             vec![0, 1, 0]],
            Shape::J => vec![vec![0, 0, 0],
                             vec![1, 1, 1],
                             vec![0, 0, 1]],
            Shape::L => vec![vec![0, 0, 0],
                             vec![1, 1, 1],
                             vec![1, 0, 0]],
            Shape::S => vec![vec![0, 0, 0],
                             vec![0, 1, 1],
                             vec![1, 1, 0]],
            Shape::Z => vec![vec![0, 0, 0],
                             vec![1, 1, 0],
                             vec![0, 1, 1]],
        }
    }

    fn get_collision_3(&self) -> Vec<Vec<i32>> {
        match self {
            Shape::I => vec![vec![0, 1, 0, 0],
                             vec![0, 1, 0, 0],
                             vec![0, 1, 0, 0],
                             vec![0, 1, 0, 0]],
            Shape::O => self.get_collision_0(),
            Shape::T => vec![vec![0, 1, 0],
                             vec![1, 1, 0],
                             vec![0, 1, 0]],
            Shape::J => vec![vec![0, 1, 0],
                             vec![0, 1, 0],
                             vec![1, 1, 0]],
            Shape::L => vec![vec![1, 1, 0],
                             vec![0, 1, 0],
                             vec![0, 1, 0]],
            Shape::S => vec![vec![1, 0, 0],
                             vec![1, 1, 0],
                             vec![0, 1, 0]],
            Shape::Z => vec![vec![0, 1, 0],
                             vec![1, 1, 0],
                             vec![1, 0, 0]],
        }
    }
}