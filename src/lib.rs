pub mod tetromino;
pub mod board;
pub mod shape;

use crate::board::Board;

pub trait Randomizer {
    fn next(&mut self) -> usize;
}

//Simple "randomizer" that always returns 0
struct StaticRand;

impl Randomizer for StaticRand {
    fn next(&mut self) -> usize {
        0
    }
}

pub struct GameBuilder {
    width: i32,
    height: i32,
    random: Box<dyn Randomizer>
}


impl GameBuilder {
    pub fn new() -> GameBuilder {
        GameBuilder {
            width: 10,
            height: 20,
            random: Box::from(StaticRand)
        }
    }

    pub fn width(mut self, width: i32) -> GameBuilder {
        self.width = width;
        self
    }

    pub fn height(mut self, height: i32) -> GameBuilder {
        self.height = height;
        self
    }

    pub fn random(mut self, random: Box<dyn Randomizer>) -> GameBuilder {
        self.random = random;
        self
    }

    pub fn build(self) -> Game {
        let board = Board::new(self.random, self.width, self.height);
        Game {
            board,
            score: 0,
            lost: false
        }
    }
}

pub struct Game {
    board: Board,
    pub score: i32,
    pub lost: bool
}

impl Game {
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