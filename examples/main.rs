use std::thread;

use libtetris::Game;


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


fn main() {
    let mut game = Game::create();
    while !game.lost {
        game.tick();
        thread::sleep(std::time::Duration::from_millis(50));
        draw_cells(game.draw());
    }
}