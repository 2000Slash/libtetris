mod tetromino;
mod board;
mod shape;
#[cfg(feature="wasm")]
pub mod wasm_game;
pub mod game;

pub trait Randomizer {
    fn next(&mut self) -> usize;
}