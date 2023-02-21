use std::fmt::Debug;

use crate::tetromino::Tetromino;

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<i32>>,
    pub current_tetromino: Option<Tetromino>,
    pub placement_timer: i32
}

impl Default for Board {
    fn default() -> Board {
        Board {
            width: 10,
            height: 20,
            cells: vec![vec![0; 10]; 20],
            current_tetromino: None,
            placement_timer: 0
        }
    }
}

impl Debug for Board {
    //print the full board with 0 and 1
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in &self.cells {
            for cell in row {
                s.push_str(&format!("{}", cell));
            }
            s.push('\n');
        }
        if let Some(tetromino) = &self.current_tetromino {
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        s.replace_range((tetromino.pos_y as usize + y) * (self.width as usize + 1) + tetromino.pos_x as usize + x..(tetromino.pos_y as usize + y) * (self.width as usize + 1) + tetromino.pos_x as usize + x + 1, "X");
                    }
                }
            }
        }
        write!(f, "{}", s)?;
        Ok(())
    }
}

impl Board {
    pub fn tick(&mut self) {
        println!("{:?}", self);
        if self.current_tetromino.is_some() {
            if self.check_collision() {
                self.placement_timer += 1;
                if self.placement_timer >= 10 {
                    self.placement_timer = 0;
                    self.place();
                }
            } else {
                self.placement_timer = 0;
                self.current_tetromino.as_mut().unwrap().move_down();
            }
        } else {
            self.current_tetromino = Some(Tetromino::create());
        }
    }

    fn place(&mut self) {
        if let Some(tetromino) = &self.current_tetromino {
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        self.cells[tetromino.pos_y as usize + y][tetromino.pos_x as usize + x] = 1;
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
                        if self.cells[tetromino.pos_y as usize + y + 1][tetromino.pos_x as usize + x] == 1 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}