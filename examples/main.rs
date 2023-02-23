use std::thread;

use tetris::{game::{Game}, Randomizer};
use rand::RngCore;


fn draw_cells(cells: Vec<i32>) {
    let mut i = 0;
    for cell in cells {
        if i % 10 == 0 {
            println!();
        }
        print!("{}", cell);
        i += 1;
    }
    println!();
}

struct Rand;

impl Randomizer for Rand {
    fn next(&mut self) -> usize {
        rand::thread_rng().next_u32() as usize
    }
}

fn main() {
    let mut game = Game::new();
    while !game.lost {
        game.tick();
        thread::sleep(std::time::Duration::from_millis(10));
        draw_cells(game.draw());
    }
}