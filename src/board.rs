use crate::{tetromino::Tetromino, Randomizer};

pub struct Board {
    width: i32,
    height: i32,
    cells: Vec<Vec<i32>>,
    current_tetromino: Option<Tetromino>,
    stored_tetromino: Option<Tetromino>,
    placement_timer: i32,
    drop_timer: i32,
    drop_time: i32,
    random: Box<dyn Randomizer>,
    pub paused: bool,
    pub lost: bool
}

impl Board {
    pub fn new(random: Box<dyn Randomizer>, width: i32, height: i32) -> Board {
        Board {
            width,
            height,
            cells: vec![vec![0; width as usize]; height as usize],
            current_tetromino: None,
            stored_tetromino: None,
            placement_timer: 0,
            drop_timer: 0,
            drop_time: 10,
            random,
            paused: false,
            lost: false
        }
    }

    pub fn draw(&self) -> Vec<i32> {
        let mut result = Vec::new();
        for row in &self.cells {
            for cell in row {
                result.push(*cell);
            }
        }
        if let Some(tetromino) = &self.current_tetromino {
            // A copy of the current tetromino should also be drawn at the bottom of the screen for the hard drop preview
            let mut preview_tetromino = tetromino.clone();

            while !self.check_collision(&preview_tetromino) {
                preview_tetromino.move_down();
            }
            let collision = preview_tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        result[((preview_tetromino.pos_y + y as i32) * self.width + preview_tetromino.pos_x + x as i32) as usize] = -1;
                    }
                }
            }
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        result[((tetromino.pos_y + y as i32) * self.width + tetromino.pos_x + x as i32) as usize] = tetromino.get_color();
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
    
    pub fn check_tetris(&mut self) {
        let mut rows_to_remove = Vec::new();
        for (y, row) in self.cells.iter().enumerate() {
            let mut full = true;
            for cell in row {
                if *cell == 0 {
                    full = false;
                    break;
                }
            }
            if full {
                rows_to_remove.push(y);
            }
        }
        for row in rows_to_remove {
            self.cells.remove(row);
            self.cells.insert(0, vec![0; self.width as usize]);
        }
    }

    pub fn tick(&mut self) {
        if self.paused {
            return;
        }
        self.check_tetris();
        if self.drop_timer >= self.drop_time {
            self.drop_timer = 0;
            self.drop();
        } else {
            self.drop_timer += 1;
        }
    }

    fn drop(&mut self) {
        if let Some(tetromino) = &self.current_tetromino {
            if self.check_collision(tetromino) {
                self.placement_timer += 1;
                if self.placement_timer >= 3 {
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
                        if tetromino.pos_y + y as i32 == 0 {
                            self.lost = true;
                            return;
                        }
                        self.cells[(tetromino.pos_y + y as i32) as usize][(tetromino.pos_x + x as i32) as usize] = tetromino.get_color();
                    }
                }
            }
        }
        self.current_tetromino = None;
    }

    pub fn hard_drop(&mut self) {
        if self.paused {
            return;
        }
        if self.current_tetromino.is_some() {
            while !self.check_collision(self.current_tetromino.as_ref().unwrap()) {
                self.current_tetromino.as_mut().unwrap().move_down();
            }
            self.place();
        }
    }

    pub fn store(&mut self) {
        if self.paused {
            return;
        }
        if self.current_tetromino.is_some() {
            if self.stored_tetromino.is_some() {
                let mut temp = self.stored_tetromino.take().unwrap();
                temp.pos_x = 4;
                temp.pos_y = 0;
                self.stored_tetromino = Some(self.current_tetromino.take().unwrap());
                self.current_tetromino = Some(temp);
            } else {
                self.stored_tetromino = Some(self.current_tetromino.take().unwrap());
            }
        } else if self.stored_tetromino.is_some() {
            self.current_tetromino = Some(self.stored_tetromino.take().unwrap());
        }
    }

    fn check_collision(&self, tetromino: &Tetromino) -> bool {
        let collision = tetromino.get_collision();
        for (y, row) in collision.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 1 {
                    if tetromino.pos_y + y as i32 + 1 >= self.height {
                        return true;
                    }
                    if self.cells[(tetromino.pos_y + y as i32 + 1) as usize][(tetromino.pos_x + x as i32) as usize] > 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn left(&mut self) {
        if self.paused {
            return;
        }
        if let Some(tetromino) = &mut self.current_tetromino {
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        if tetromino.pos_x + x as i32 == 0 {
                            return;
                        }
                        if self.cells[(tetromino.pos_y + y as i32) as usize][(tetromino.pos_x + x as i32 - 1) as usize] > 0 {
                            return;
                        }
                    }
                }
            }
            tetromino.move_left();
        }
    }

    pub fn right(&mut self) {
        if self.paused {
            return;
        }
        if let Some(tetromino) = &mut self.current_tetromino {
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        if tetromino.pos_x + x as i32 + 1 >= self.width {
                            return;
                        }
                        if self.cells[(tetromino.pos_y + y as i32) as usize][(tetromino.pos_x + x as i32 + 1) as usize] > 0 {
                            return;
                        }
                    }
                }
            }
            tetromino.move_right();
        }
    }

    pub fn down(&mut self) {
        if self.paused {
            return;
        }
        if let Some(tetromino) = &mut self.current_tetromino {
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        if tetromino.pos_y + y as i32 + 1 >= self.height {
                            self.placement_timer = 10;
                            return;
                        }
                        if self.cells[(tetromino.pos_y + y as i32 + 1) as usize][(tetromino.pos_x + x as i32) as usize] > 0 {
                            return;
                        }
                    }
                }
            }
            tetromino.move_down();
        }
    }

    pub fn rotate_right(&mut self) {
        if self.paused {
            return;
        }
        if let Some(tetromino) = &mut self.current_tetromino {
            tetromino.rotate(1);
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        if tetromino.pos_x + x as i32 >= self.width {
                            tetromino.move_left();
                            return;
                        }
                        if (tetromino.pos_x + x as i32) < 0 {
                            tetromino.move_right();
                            return;
                        }
                        if tetromino.pos_y + y as i32 >= self.height {
                            tetromino.move_up();
                            return;
                        }
                        if self.cells[(tetromino.pos_y + y as i32) as usize][(tetromino.pos_x + x as i32) as usize] > 0 {
                            tetromino.rotate(-1);
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn rotate_left(&mut self) {
        if self.paused {
            return;
        }
        if let Some(tetromino) = &mut self.current_tetromino {
            tetromino.rotate(-1);
            let collision = tetromino.get_collision();
            for (y, row) in collision.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == 1 {
                        if tetromino.pos_x + x as i32 >= self.width {
                            tetromino.move_left();
                            return;
                        }
                        if (tetromino.pos_x + x as i32) < 0 {
                            tetromino.move_right();
                            return;
                        }
                        if tetromino.pos_y + y as i32 >= self.height {
                            tetromino.move_up();
                            return;
                        }
                        if self.cells[(tetromino.pos_y + y as i32) as usize][(tetromino.pos_x + x as i32) as usize] > 0 {
                            tetromino.rotate(1);
                            return;
                        }
                    }
                }
            }
        }
    }
}