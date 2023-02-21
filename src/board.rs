use crate::tetromino::Tetromino;


pub struct Board {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<i32>>,
    pub current_tetromino: Option<Tetromino>,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            width: 10,
            height: 20,
            cells: vec![vec![0; 10]; 20],
            current_tetromino: None,
        }
    }
}

impl Board {
    
}