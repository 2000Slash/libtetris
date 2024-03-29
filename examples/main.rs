use std::thread;

use tetris::{game::{Game}, Randomizer};
use rand::RngCore;


fn draw_cells(cells: Vec<i32>) {
    for (i, cell) in cells.into_iter().enumerate() {
        if i % 10 == 0 {
            println!();
        }
        if cell > 0 {
            print!("█");
        } else {
            print!(" ");
        }
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
    let mut game = Game::new(Box::new(Rand));
    while !game.lost {
        game.tick();
        thread::sleep(std::time::Duration::from_millis(1));
        draw_cells(game.draw());
    }
}