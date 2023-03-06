
use std::collections::HashMap;

use rand::{thread_rng, Rng};

use crate::{game::Game, Randomizer};

#[derive(Default)]
struct State {
    games: HashMap<i32, Game>,
    cells: HashMap<i32, Vec<i32>>
}

static mut STATE: Option<State> = None;

struct Rand;

impl Randomizer for Rand {
    fn next(&mut self) -> usize {
        thread_rng().gen()
    }
}

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        STATE = Some(State::default());
        println!("Initialized");
    }
}

#[no_mangle]
pub extern "C" fn create_game() -> i32 {
    unsafe {
        if let Some(state) = &mut STATE {
            let id = thread_rng().gen();
            state.games.insert(id, Game::new(Box::new(Rand)));
            state.cells.insert(id, Vec::new());
            return id;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn destroy_game(id: i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            state.games.remove(&id);
            state.cells.remove(&id);
        }
    }
}

#[no_mangle]
pub extern "C" fn tick(id: i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                game.tick();
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn draw(id: i32) -> *const i32 {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                state.cells.insert(id, game.draw());
                return state.cells.get(&id).unwrap().as_ptr();
            }
        }
    }
    std::ptr::null()
}

#[no_mangle]
pub extern "C" fn hard_drop(id: i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                game.hard_drop();
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn store(id :i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                game.store();
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn left(id: i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                game.left();
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn right(id: i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                game.right();
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn down(id: i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                game.down();
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn rotate_right(id: i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                game.rotate_right();
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn rotate_left(id: i32) {
    unsafe {
        if let Some(state) = &mut STATE {
            if let Some(game) = state.games.get_mut(&id) {
                game.rotate_left();
            }
        }
    }
}