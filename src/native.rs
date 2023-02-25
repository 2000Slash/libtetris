#![allow(improper_ctypes_definitions)]
use crate::{board::Board, Randomizer};

extern "C" {
    pub fn get_random() -> i32;
}

struct ExternRand;

impl Randomizer for ExternRand {
    fn next(&mut self) -> usize {
        unsafe { get_random() as usize }
    }
}

#[repr(C)]
pub struct NativeGame {
    board: Board,
    pub score: i32,
    pub lost: bool
}

impl NativeGame {

    #[no_mangle]
    pub extern "C" fn new(width: i32, height: i32) -> NativeGame {
        NativeGame {
            board: Board::new(Box::new(ExternRand), width, height),
            score: 0,
            lost: false
        }
    }

    #[no_mangle]
    pub extern "C" fn draw(&self) -> *const i32 {
        self.board.draw().as_ptr()
    }

    #[no_mangle]
    pub extern "C" fn tick(&mut self) {
        self.board.tick();
        if self.board.lost {
            self.lost = true;
        }
    }

    #[no_mangle]
    pub extern "C" fn hard_drop(&mut self) {
        self.board.hard_drop();
    }

    #[no_mangle]
    pub extern "C" fn store(&mut self) {
        self.board.store();
    }

    #[no_mangle]
    pub extern "C" fn left(&mut self) {
        self.board.left();
    }

    #[no_mangle]
    pub extern "C" fn right(&mut self) {
        self.board.right();
    }

    #[no_mangle]
    pub extern "C" fn down(&mut self) {
        self.board.down();
    }

    #[no_mangle]
    pub extern "C" fn rotate_right(&mut self) {
        self.board.rotate_right();
    }

    #[no_mangle]
    pub extern "C" fn rotate_left(&mut self) {
        self.board.rotate_left();
    }
}