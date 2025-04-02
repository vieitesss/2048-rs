use std::{
    fmt::Display,
    io::stdout,
};

use crossterm::{cursor, event::KeyCode, execute};

use crate::utils;

#[derive(Debug)] pub struct Matrix {
    pub data: Vec<Vec<u32>>,
    pub changed: bool,
    pub width: usize,
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            data: vec![vec![0; 4]; 4],
            changed: false,
            width: 4,
        }
    }
}

impl Matrix {
    pub fn get_width_on_draw(&self) -> (usize, usize) {
        let width = self.width;
        (6 * width + 1, 2 * width + 1)
    }

    fn move_to_next_line(&self) {
        execute!(
            stdout(),
            cursor::MoveDown(1),
            cursor::MoveLeft(self.get_width_on_draw().0 as u16),
        )
        .expect("could not move cursor");
    }
}

pub trait MatrixTrait {
    fn spawn_number(&mut self);
    fn update_vector(&self, vector: &[u32], dir: KeyCode) -> (Vec<u32>, bool);
    fn shift(&mut self, dir: KeyCode);
    fn merge(&self, numbers: &[u32], dir: KeyCode) -> Vec<u32>;
    fn has_no_moves(&self) -> bool;
}

impl MatrixTrait for Matrix {
    fn spawn_number(&mut self) {
        let cell = utils::get_random_empty_cell(&self.data);
        let random_value = utils::get_random_bool(0.75);

        self.data[cell.0][cell.1] = if random_value { 2 } else { 4 };
    }

    fn update_vector(&self, vector: &[u32], dir: KeyCode) -> (Vec<u32>, bool) {
        let merged = self.merge(vector, dir);
        let changed = merged != *vector;

        (merged, changed)
    }

    fn shift(&mut self, dir: KeyCode) {
        let width = self.width;

        match dir {
            KeyCode::Right | KeyCode::Left => {
                for i in 0..width {
                    let (moved, changed) = self.update_vector(&self.data[i], dir);

                    if changed {
                        self.data[i] = moved;
                        self.changed = true;
                    }
                }
            }
            KeyCode::Up | KeyCode::Down => {
                for j in 0..width {
                    let mut numbers = vec![0; width];
                    numbers
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, x)| *x = self.data[i][j]);

                    let (moved, changed) = self.update_vector(&numbers, dir);

                    if changed {
                        for (i, _) in moved.iter().enumerate() {
                            self.data[i][j] = moved[i];
                        }
                        self.changed = true;
                    }
                }
            }
            _ => panic!("invalid direction"),
        }
    }

    fn merge(&self, numbers: &[u32], dir: KeyCode) -> Vec<u32> {
        let mut non_zeros = utils::get_non_zeros(numbers);
        let count = non_zeros.len();

        if count == 0 {
            return numbers.to_vec();
        }

        let width = self.width;
        let mut moved = vec![0; width];

        // revert the non-zero numbers
        let mut revert = false;
        match dir {
            KeyCode::Left | KeyCode::Up => {}
            KeyCode::Right | KeyCode::Down => {
                non_zeros = utils::rev(&non_zeros);
                revert = true;
            }
            _ => panic!("invalid direction"),
        }

        let mut index = 0;
        let mut non_zero = 0;
        while non_zero < count {
            if non_zero == count - 1 || non_zeros[non_zero] != non_zeros[non_zero + 1] {
                moved[index] = non_zeros[non_zero];
                non_zero += 1;
            } else {
                moved[index] = non_zeros[non_zero] * 2;
                non_zero += 2;
            }
            index += 1;
        }

        if revert {
            moved = utils::rev(&moved);
        }

        moved
    }

    fn has_no_moves(&self) -> bool {
        if !utils::get_empty_cells(&self.data).is_empty() {
            return false;
        }

        let width = self.width;
        for i in 0..width {
            for j in 0..width {
                if i < width - 1 && self.data[i][j] == self.data[i + 1][j] {
                    return false;
                }
                if j < width - 1 && self.data[i][j] == self.data[i][j + 1] {
                    return false;
                }
            }
        }

        true
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width;
        write!(f, "┌─────┬─────┬─────┬─────┐")?;
        self.move_to_next_line();
        for i in 0..width {
            write!(f, "│")?;
            for j in 0..width {
                let current = self.data[i][j];
                if current == 0 {
                    write!(f, "  .  │")?;
                } else {
                    write!(f, "{:^5}│", current)?;
                }
            }
            self.move_to_next_line();
            if i != width - 1 {
                write!(f, "├─────┼─────┼─────┼─────┤")?;
                self.move_to_next_line();
            }
        }

        writeln!(f, "└─────┴─────┴─────┴─────┘\r")?;
        Ok(())
    }
}
