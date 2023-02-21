pub mod tetromino;
pub mod board;
pub mod shape;

use crate::board::Board;



pub struct Game {
    board: Board,
    pub score: i32,
    pub lost: bool,
}


impl Game {
    pub fn create() -> Game {
        Game {
            board: Board::default(),
            score: 0,
            lost: false
        }
    }

    pub fn restart(&mut self) {
        self.score = 0;
        self.lost = false;
        self.board.reset();
    }

    pub fn tick(&mut self) {
        self.board.tick();
        if self.board.lost {
            self.lost = true;
        }
    }
}