pub mod tetromino;
pub mod board;
pub mod shape;
use crate::board::Board;



pub struct Game {
    board: Board,
    pub score: i32,
}

impl Game {
    pub fn create() -> Game {
        Game {
            board: Board::default(),
            score: 0,
        }
    }

    pub fn tick(&mut self) {
        self.board.tick();
    }
}