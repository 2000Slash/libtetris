use std::fmt::{Debug, Display};

use rand::rngs::ThreadRng;

use crate::tetromino::Tetromino;

pub struct Board {
    width: i32,
    height: i32,
    cells: Vec<Vec<i32>>,
    current_tetromino: Option<Tetromino>,
    placement_timer: i32,
    random: ThreadRng,
    pub paused: bool,
    pub lost: bool
}

impl Default for Board {
    fn default() -> Board {
        Board {
            width: 10,
            height: 20,
            cells: vec![vec![0; 10]; 20],
            current_tetromino: None,
            placement_timer: 0,
            random: rand::thread_rng(),
            paused: false,
            lost: false
        }
    }
}

impl Board {
    pub fn draw(&self) -> Vec<i32> {
        let mut result = Vec::new();
        for row in &self.cells {
            for cell in row {
                result.push(*cell);
            }
        }
        if let Some(tetromino) = &self.current_tetromino {
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        result[(tetromino.pos_y as usize + y) * self.width as usize + tetromino.pos_x as usize + x] = tetromino.get_color();
                    }
                }
            }
        }
        result
    }

    pub fn reset(&mut self) {
        self.cells = vec![vec![0; self.width as usize]; self.height as usize];
        self.current_tetromino = None;
        self.placement_timer = 0;
        self.paused = false;
        self.lost = false;
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }
        if self.current_tetromino.is_some() {
            if self.check_collision() {
                self.placement_timer += 1;
                if self.placement_timer >= 1 {
                    self.placement_timer = 0;
                    self.place();
                }
            } else {
                self.placement_timer = 0;
                self.current_tetromino.as_mut().unwrap().move_down();
            }
        } else {
            self.current_tetromino = Some(Tetromino::create(&mut self.random));
        }
    }

    fn place(&mut self) {
        if let Some(tetromino) = &self.current_tetromino {
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        if tetromino.pos_y as usize + y == 0 {
                            self.lost = true;
                            return;
                        }
                        self.cells[tetromino.pos_y as usize + y][tetromino.pos_x as usize + x] = tetromino.get_color();
                    }
                }
            }
        }
        self.current_tetromino = None;
    }

    fn check_collision(&self) -> bool {
        if let Some(tetromino) = &self.current_tetromino {
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        if tetromino.pos_y as usize + y + 1 >= self.height as usize {
                            return true;
                        }
                        if self.cells[tetromino.pos_y as usize + y + 1][tetromino.pos_x as usize + x] > 0 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}