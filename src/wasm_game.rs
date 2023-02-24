use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Randomizer, board::Board};

#[wasm_bindgen(module = "/src/random.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn get_random() -> i32;
}

struct StaticRand;

impl Randomizer for StaticRand {
    fn next(&mut self) -> usize {
        get_random() as usize
    }
}

#[wasm_bindgen]
pub struct WasmGame {
    board: Board,
    pub score: i32,
    pub lost: bool
}

#[wasm_bindgen]
impl WasmGame {
    pub fn new() -> WasmGame {
        WasmGame {
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

    pub fn hard_drop(&mut self) {
        self.board.hard_drop();
    }

    pub fn store(&mut self) {
        self.board.store();
    }

    pub fn left(&mut self) {
        self.board.left();
    }

    pub fn right(&mut self) {
        self.board.right();
    }

    pub fn down(&mut self) {
        self.board.down();
    }

    pub fn rotate_right(&mut self) {
        self.board.rotate_right();
    }

    pub fn rotate_left(&mut self) {
        self.board.rotate_left();
    }
}