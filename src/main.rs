#![allow(dead_code)]
#![allow(unused_variables)]

use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    fmt::Display,
    io::{self, stdout, Write},
};

mod utils;

#[derive(Copy, Clone)]
enum ZerosTo {
    Right,
    Left,
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<u32>>,
    changed: bool,
    width: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Starting,
    Running,
    GameOver,
}

trait GameTrait {
    fn update(&mut self) -> io::Result<()>;
    fn handle_input(&mut self) -> io::Result<()>;
    fn draw(&self);
    fn exit(&self);
}

#[derive(Debug)]
struct Game {
    matrix: Matrix,
    state: State,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            matrix: Matrix::default(),
            state: State::Running,
        };

        game.update.unwrap();

        game
    }
}

impl GameTrait for Game {
    fn update(&mut self) -> io::Result<()> {
        utils::clear_screen()?;
        self.matrix.spawn_number();
        writeln!(stdout(), "{}\r", self).expect("could not write update");

        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<()> {
        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    self.matrix.shift(code);
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.state = State::GameOver;
                }
                _ => {}
            },
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }

        if self.matrix.changed {
            self.update()?;
            self.matrix.changed = false;
        } else if self.matrix.has_no_moves() {
            self.state = State::GameOver;
        }

        Ok(())
    }

    fn draw(&self) {
        writeln!(stdout(), "{}\r", self).expect("could not write update");
    }

    fn exit(&self) {
        todo!()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
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

trait MatrixTrait {
    fn spawn_number(&mut self);
    fn update_vector(&self, vector: &[u32], dir: KeyCode, zeros_to: ZerosTo) -> (Vec<u32>, bool);
    fn shift(&mut self, dir: KeyCode);
    fn merge(&self, numbers: &[u32], dir: KeyCode) -> Vec<u32>;
    fn move_zeros(&self, numbers: &[u32], dir: ZerosTo) -> Vec<u32>;
    fn has_no_moves(&self) -> bool;
}

impl MatrixTrait for Matrix {
    fn spawn_number(&mut self) {
        let cell = utils::get_random_empty_cell(&self.data, self.width);
        let random_value = utils::get_random_bool(0.75);

        self.data[cell.0][cell.1] = if random_value { 2 } else { 4 };
    }

    fn update_vector(&self, vector: &[u32], dir: KeyCode, zeros_to: ZerosTo) -> (Vec<u32>, bool) {
        let merged = self.merge(vector, dir);
        let moved = self.move_zeros(&merged, zeros_to);

        (moved.clone(), moved != *vector)
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
                    // let merged = self.merge(numbers, dir);
                    // let moved = self.move_zeros(&merged, zeros_to);

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

                    // let merged = self.merge(&numbers, dir);
                    // let moved = self.move_zeros(&merged, zeros_to);

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
        let mut non_zero: Vec<_> = numbers.iter().filter(|&&x| x != 0).copied().collect();
        let count = non_zero.len();

        if non_zero.is_empty() || count == 1 {
            return numbers.to_vec();
        }

        let width = self.width;
        let mut moved = vec![0; width];

        // revert the non-zero numbers
        let mut revert = false;
        match dir {
            KeyCode::Left | KeyCode::Up => {}
            KeyCode::Right | KeyCode::Down => {
                non_zero = utils::rev(&non_zero);
                revert = true;
            }
            _ => panic!("invalid direction"),
        }

        let mut index = 0;
        while index < count {
            if index == count - 1 || non_zero[index] != non_zero[index + 1] {
                moved[index] = non_zero[index];
                index += 1;
            } else if non_zero[index] == non_zero[index + 1] {
                moved[index] = non_zero[index] * 2;
                index += 2;
            }
        }

        if revert {
            moved = utils::rev(&moved);
        }

        moved
    }

    fn move_zeros(&self, numbers: &[u32], dir: ZerosTo) -> Vec<u32> {
        let non_zeros: Vec<u32> = numbers.iter().filter(|&&x| x != 0).copied().collect();

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
                    .copy_from_slice(&non_zeros[..(width - (width - non_zeros.len()))]);
            }
        }

        moved
    }

    fn has_no_moves(&self) -> bool {
        if utils::get_empty_cells(&self.data, self.width).is_empty() {
            return false;
        }

        let width = self.width;
        for i in 0..width {
            for j in 0..width {
                if i < 3 && self.data[i][j] == self.data[i + 1][j] {
                    return false;
                }
                if j < 3 && self.data[i][j] == self.data[i][j + 1] {
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

fn main() -> io::Result<()> {
    execute!(stdout(), Hide)?;
    enable_raw_mode()?;

    let mut matrix = Matrix::default();
    matrix.update().unwrap();

    let mut exit = false;
    while !exit {
        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    matrix.shift(code);
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    exit = true;
                }
                _ => {
                    continue;
                }
            },
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }

        if matrix.changed {
            matrix.update()?;
            matrix.changed = false;
        } else if matrix.has_no_moves() {
            writeln!(stdout(), "Game over!\r")?;
            exit = true;
        }
    }

    execute!(stdout(), Show)?;
    disable_raw_mode()
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use super::ZerosTo;
    use crate::KeyCode;

    macro_rules! move_zeros {
        ($init:expr, $zeros_to:expr, $result:expr) => {
            assert_eq!(Matrix::move_zeros(&$init, $zeros_to), $result)
        };
    }

    macro_rules! merge {
        ($init:expr, $key:expr, $result:expr) => {
            assert_eq!(Matrix::merge(&$init, $key), $result)
        };
    }

    #[test]
    fn test_move_zeros_right() {
        move_zeros!([0, 0, 0, 0], ZerosTo::Right, [0, 0, 0, 0]);
        move_zeros!([2, 0, 0, 0], ZerosTo::Right, [2, 0, 0, 0]);
        move_zeros!([0, 2, 0, 0], ZerosTo::Right, [2, 0, 0, 0]);
        move_zeros!([0, 0, 2, 0], ZerosTo::Right, [2, 0, 0, 0]);
        move_zeros!([0, 0, 0, 2], ZerosTo::Right, [2, 0, 0, 0]);
        move_zeros!([2, 2, 0, 0], ZerosTo::Right, [2, 2, 0, 0]);
        move_zeros!([2, 0, 2, 0], ZerosTo::Right, [2, 2, 0, 0]);
        move_zeros!([2, 0, 0, 2], ZerosTo::Right, [2, 2, 0, 0]);
        move_zeros!([0, 2, 2, 0], ZerosTo::Right, [2, 2, 0, 0]);
        move_zeros!([0, 2, 0, 2], ZerosTo::Right, [2, 2, 0, 0]);
        move_zeros!([0, 0, 2, 2], ZerosTo::Right, [2, 2, 0, 0]);
        move_zeros!([2, 2, 2, 0], ZerosTo::Right, [2, 2, 2, 0]);
        move_zeros!([2, 2, 0, 2], ZerosTo::Right, [2, 2, 2, 0]);
        move_zeros!([2, 2, 2, 2], ZerosTo::Right, [2, 2, 2, 2]);

        move_zeros!([0, 2, 4, 0], ZerosTo::Right, [2, 4, 0, 0]);
        move_zeros!([0, 4, 2, 0], ZerosTo::Right, [4, 2, 0, 0]);
        move_zeros!([2, 4, 0, 0], ZerosTo::Right, [2, 4, 0, 0]);
        move_zeros!([0, 0, 2, 4], ZerosTo::Right, [2, 4, 0, 0]);
    }

    #[test]
    fn test_move_zeros_left() {
        move_zeros!([0, 0, 0, 0], ZerosTo::Left, [0, 0, 0, 0]);
        move_zeros!([2, 0, 0, 0], ZerosTo::Left, [0, 0, 0, 2]);
        move_zeros!([0, 2, 0, 0], ZerosTo::Left, [0, 0, 0, 2]);
        move_zeros!([0, 0, 2, 0], ZerosTo::Left, [0, 0, 0, 2]);
        move_zeros!([0, 0, 0, 2], ZerosTo::Left, [0, 0, 0, 2]);
        move_zeros!([2, 2, 0, 0], ZerosTo::Left, [0, 0, 2, 2]);
        move_zeros!([2, 0, 2, 0], ZerosTo::Left, [0, 0, 2, 2]);
        move_zeros!([2, 0, 0, 2], ZerosTo::Left, [0, 0, 2, 2]);
        move_zeros!([0, 2, 2, 0], ZerosTo::Left, [0, 0, 2, 2]);
        move_zeros!([0, 2, 0, 2], ZerosTo::Left, [0, 0, 2, 2]);
        move_zeros!([0, 0, 2, 2], ZerosTo::Left, [0, 0, 2, 2]);
        move_zeros!([2, 2, 2, 0], ZerosTo::Left, [0, 2, 2, 2]);
        move_zeros!([2, 2, 0, 2], ZerosTo::Left, [0, 2, 2, 2]);
        move_zeros!([2, 2, 2, 2], ZerosTo::Left, [2, 2, 2, 2]);

        move_zeros!([0, 2, 4, 0], ZerosTo::Left, [0, 0, 2, 4]);
        move_zeros!([0, 4, 2, 0], ZerosTo::Left, [0, 0, 4, 2]);

        move_zeros!([2, 4, 0, 0], ZerosTo::Left, [0, 0, 2, 4]);
        move_zeros!([0, 0, 2, 4], ZerosTo::Left, [0, 0, 2, 4]);
    }

    #[test]
    fn test_merge_left() {
        merge!([2, 2, 0, 0], KeyCode::Left, [4, 0, 0, 0]);
        merge!([0, 2, 2, 0], KeyCode::Left, [4, 0, 0, 0]);
        merge!([2, 0, 0, 2], KeyCode::Left, [4, 0, 0, 0]);
        merge!([0, 2, 0, 2], KeyCode::Left, [4, 0, 0, 0]);
        merge!([0, 2, 0, 2], KeyCode::Left, [4, 0, 0, 0]);

        merge!([2, 2, 4, 0], KeyCode::Left, [4, 0, 4, 0]);
        merge!([2, 4, 2, 0], KeyCode::Left, [2, 4, 2, 0]);
        merge!([4, 2, 2, 0], KeyCode::Left, [4, 4, 0, 0]);

        merge!([2, 2, 2, 0], KeyCode::Left, [4, 0, 2, 0]);
        merge!([2, 2, 2, 2], KeyCode::Left, [4, 0, 4, 0]);
    }

    #[test]
    fn test_merge_right() {
        merge!([2, 2, 0, 0], KeyCode::Right, [0, 0, 0, 4]);
        merge!([0, 2, 2, 0], KeyCode::Right, [0, 0, 0, 4]);
        merge!([2, 0, 0, 2], KeyCode::Right, [0, 0, 0, 4]);
        merge!([0, 2, 0, 2], KeyCode::Right, [0, 0, 0, 4]);
        merge!([0, 2, 0, 2], KeyCode::Right, [0, 0, 0, 4]);

        merge!([0, 2, 4, 0], KeyCode::Right, [0, 0, 2, 4]);
        merge!([2, 4, 0, 0], KeyCode::Right, [0, 0, 2, 4]);
        merge!([0, 0, 2, 4], KeyCode::Right, [0, 0, 2, 4]);

        merge!([2, 2, 4, 0], KeyCode::Right, [0, 0, 4, 4]);
        merge!([2, 4, 2, 0], KeyCode::Right, [0, 2, 4, 2]);
        merge!([4, 2, 2, 0], KeyCode::Right, [0, 4, 0, 4]);
        merge!([2, 2, 2, 0], KeyCode::Right, [0, 2, 0, 4]);

        merge!([2, 2, 2, 2], KeyCode::Right, [0, 4, 0, 4]);
    }
}
