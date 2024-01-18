use std::{
    fmt::Display,
    io::{stdout, Write},
};

use crossterm::event::KeyCode;

use crate::utils;

#[derive(Copy, Clone)]
pub enum ZerosTo {
    Right,
    Left,
}

#[derive(Debug)]
pub struct Matrix {
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

pub trait MatrixTrait {
    fn spawn_number(&mut self);
    fn update_vector(&self, vector: &[u32], dir: KeyCode, zeros_to: ZerosTo) -> (Vec<u32>, bool);
    fn shift(&mut self, dir: KeyCode);
    fn merge(&self, numbers: &[u32], dir: KeyCode) -> Vec<u32>;
    fn move_zeros(&self, numbers: &[u32], dir: ZerosTo) -> Vec<u32>;
    fn has_no_moves(&self) -> bool;
}

impl MatrixTrait for Matrix {
    fn spawn_number(&mut self) {
        let cell = utils::get_random_empty_cell(&self.data);
        let random_value = utils::get_random_bool(0.75);

        self.data[cell.0][cell.1] = if random_value { 2 } else { 4 };
    }

    fn update_vector(&self, vector: &[u32], dir: KeyCode, zeros_to: ZerosTo) -> (Vec<u32>, bool) {
        let merged = self.merge(vector, dir);
        let moved = self.move_zeros(&merged, zeros_to);
        let changed = moved != *vector;

        (moved, changed)
    }

    fn shift(&mut self, dir: KeyCode) {
        let width = self.width;

        let zeros_to = match dir {
            KeyCode::Up | KeyCode::Left => ZerosTo::Right,
            KeyCode::Right | KeyCode::Down => ZerosTo::Left,
            _ => panic!("invalid direction"),
        };

        match dir {
            KeyCode::Right | KeyCode::Left => {
                for i in 0..width {
                    let (moved, changed) = self.update_vector(&self.data[i], dir, zeros_to);

                    if changed {
                        self.data[i] = moved;
                        self.changed = true;
                    }
                }
            }
            KeyCode::Up | KeyCode::Down => {
                for i in 0..width {
                    let mut numbers = vec![0; width];
                    numbers
                        .iter_mut()
                        .enumerate()
                        .for_each(|(j, x)| *x = self.data[j][i]);

                    let (moved, changed) = self.update_vector(&numbers, dir, zeros_to);

                    if changed {
                        for (j, _) in moved.iter().enumerate().take(width) {
                            self.data[j][i] = moved[j];
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

        if non_zeros.is_empty() || count == 1 {
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
        while index < count {
            if index == count - 1 || non_zeros[index] != non_zeros[index + 1] {
                moved[index] = non_zeros[index];
                index += 1;
            } else if non_zeros[index] == non_zeros[index + 1] {
                moved[index] = non_zeros[index] * 2;
                index += 2;
            }
        }

        if revert {
            moved = utils::rev(&moved);
        }

        moved
    }

    fn move_zeros(&self, numbers: &[u32], dir: ZerosTo) -> Vec<u32> {
        let non_zeros = utils::get_non_zeros(numbers);

        if non_zeros.is_empty() {
            return numbers.to_vec();
        }

        let width = self.width;
        let mut moved = vec![0; width];

        match dir {
            ZerosTo::Right => {
                moved[..non_zeros.len()].copy_from_slice(&non_zeros[..]);
            }
            ZerosTo::Left => {
                moved[(width - non_zeros.len())..width]
                    .copy_from_slice(&non_zeros[..]);
            }
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
                    writeln!(stdout(), "No empty cells").expect("could not write update");
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
        for i in 0..width {
            for j in 0..width {
                let current = self.data[i][j];
                if current == 0 {
                    write!(f, "{:<5}", ".".to_string())?;
                } else {
                    write!(f, "{:<5}", current)?;
                }
            }
            write!(f, "\r\n")?;
        }

        Ok(())
    }
}
