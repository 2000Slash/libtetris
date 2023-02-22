use crate::{Randomizer, board::Board};


struct StaticRand;

impl Randomizer for StaticRand {
    fn next(&mut self) -> usize {
        1
    }
}
pub struct Game {
    board: Board,
    pub score: i32,
    pub lost: bool
}


impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(Box::new(StaticRand), 10, 20),
            score: 0,
            lost: false
        }
    }

    pub fn draw(&self) -> Vec<i32> {
        self.board.draw()
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