use crossterm::{
    cursor::{Hide, MoveToColumn, MoveToRow, Show},
    event::{read, Event, KeyCode, KeyEvent},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::{
    fmt::Display,
    io::{self, stdout, Write},
};

#[derive(Copy, Clone)]
enum ZerosTo {
    Right,
    Left,
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<u32>>,
    changed: bool,
}

// TODO: test movements
impl Matrix {
    fn new() -> Matrix {
        Matrix {
            data: vec![vec![0; 4]; 4],
            changed: false,
        }
    }

    fn spawn_number(&mut self) {
        let cell = self.get_random_empty_cell();

        let random_value = Bernoulli::new(0.75)
            .unwrap()
            .sample(&mut rand::thread_rng());

        writeln!(stdout(), "{:?}", cell).unwrap();

        self.data[cell.0][cell.1] = if random_value { 2 } else { 4 };
    }

    fn get_random_empty_cell(&self) -> (usize, usize) {
        let empty_cells = self.get_empty_cells();

        assert!(empty_cells.len() > 0);

        empty_cells[rand::thread_rng().gen_range(0..empty_cells.len())]
    }

    fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        assert_eq!(self.data.len(), 4);

        let mut empty_cells = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.data[i][j] == 0 {
                    empty_cells.push((i, j));
                }
            }
        }
        empty_cells
    }

    fn update(&mut self) -> io::Result<()> {
        self.spawn_number();
        queue!(
            stdout(),
            Clear(ClearType::All),
            MoveToRow(0),
            MoveToColumn(0)
        )
        .unwrap();
        writeln!(stdout(), "{}\r", self).expect("could not write update");

        Ok(())
    }

    fn rev(vector: &[u32]) -> Vec<u32> {
        vector.iter().rev().map(|x| *x).collect()
    }

    fn shift(&mut self, dir: KeyCode) {
        assert_eq!(self.data.len(), 4);

        let zeros_to = match dir {
            KeyCode::Up | KeyCode::Left => ZerosTo::Right,
            KeyCode::Right | KeyCode::Down => ZerosTo::Left,
            _ => panic!("invalid direction"),
        };

        match dir {
            KeyCode::Right | KeyCode::Left => {
                for i in 0..4 {
                    let numbers = &self.data[i];
                    let merged = Matrix::merge(&numbers, dir);
                    let moved = Matrix::move_zeros(&merged, zeros_to);

                    if moved != *numbers {
                        self.data[i] = moved;
                        self.changed = true;
                    }
                }
            }
            KeyCode::Up | KeyCode::Down => {
                for i in 0..4 {
                    let mut numbers = [0; 4];
                    for j in 0..4 {
                        numbers[j] = self.data[j][i];
                    }

                    let merged = Matrix::merge(&numbers, dir);
                    let moved = Matrix::move_zeros(&merged, zeros_to);

                    if moved != numbers {
                        for j in 0..4 {
                            self.data[j][i] = moved[j];
                        }
                        self.changed = true;
                    }
                }
            }
            _ => panic!("invalid direction"),
        }
    }

    fn merge(numbers: &[u32], dir: KeyCode) -> Vec<u32> {
        assert_eq!(numbers.len(), 4);
        let mut non_zero: Vec<_> = numbers.iter().filter(|&&x| x != 0).map(|x| *x).collect();
        let count = non_zero.len();

        if non_zero.is_empty() || count == 1 {
            return numbers.to_vec();
        }

        let mut moved = vec![0; 4];

        // revert the non-zero numbers
        let mut revert = false;
        match dir {
            KeyCode::Left | KeyCode::Up => {}
            KeyCode::Right | KeyCode::Down => {
                non_zero = Matrix::rev(&non_zero);
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
            moved = Matrix::rev(&moved);
        }

        moved
    }

    fn move_zeros(numbers: &[u32], dir: ZerosTo) -> Vec<u32> {
        assert_eq!(numbers.len(), 4);

        let non_zeros: Vec<u32> = numbers.iter().filter(|&&x| x != 0).map(|x| *x).collect();

        if non_zeros.len() == 0 {
            return numbers.to_vec();
        }

        let mut moved = vec![0; 4];

        match dir {
            ZerosTo::Right => {
                for i in 0..non_zeros.len() {
                    moved[i] = non_zeros[i];
                }
            }
            ZerosTo::Left => {
                let mut index = 0;
                for i in 4 - non_zeros.len()..4 {
                    moved[i] = non_zeros[index];
                    index += 1;
                }
            }
        }

        moved
    }

    fn has_no_moves(&self) -> bool {
        assert_eq!(self.data.len(), 4);
        if self.get_empty_cells().len() > 0 {
            return false;
        }

        for i in 0..4 {
            for j in 0..4 {
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
        for i in 0..4 {
            for j in 0..4 {
                let current = self.data[i][j];
                if current == 0 {
                    write!(f, "{:<4}", ".".to_string())?;
                } else {
                    write!(f, "{:<4}", current)?;
                }
            }
            write!(f, "\r\n")?;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    execute!(stdout(), Hide)?;
    let mut matrix = Matrix::new();

    enable_raw_mode()?;

    matrix.update().unwrap();

    let mut exit = false;
    while !exit {
        match read()? {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
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
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
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
