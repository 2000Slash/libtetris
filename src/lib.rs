pub mod tetromino;
pub mod board;
pub mod shape;

use std::{panic};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::board::Board;

pub trait Randomizer {
    fn next(&mut self) -> usize;
}

#[wasm_bindgen(module = "/src/random.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn get_random() -> i32;
}

//Simple "randomizer" that uses the JS Math.random() function
struct StaticRand;

impl Randomizer for StaticRand {
    fn next(&mut self) -> usize {
        get_random() as usize
    }
}

#[wasm_bindgen]
pub struct Game {
    board: Board,
    pub score: i32,
    pub lost: bool
}

#[wasm_bindgen]
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