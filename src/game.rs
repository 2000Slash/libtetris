use crate::board::Board;



pub struct Game {
    board: Board,
    score: i32,
}

impl Game {
    pub fn create() -> Game {
        Game {
            board: Board::default(),
            score: 0,
        }
    }
}