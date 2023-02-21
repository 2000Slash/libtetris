use std::thread;

use libtetris::Game;



fn main() {
    let mut game = Game::create();
    while !game.lost {
        game.tick();
        thread::sleep(std::time::Duration::from_millis(10));
    }
}