use libtetris::Game;



fn main() {
    let mut game = Game::create();
    for _ in 0..100 {
        game.tick();
    }
}
