use crossterm::{
    cursor::{MoveToColumn, MoveToRow},
    event::{read, Event, KeyCode, KeyEvent},
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::{
    fmt::Display,
    io::{self, stdout, Write},
    thread::sleep,
    time::Duration,
};

#[derive(Debug)]
struct Matrix {
    data: [[u32; 4]; 4],
    changed: bool,
}

// TODO: test movements
impl Matrix {
    fn new() -> Matrix {
        Matrix {
            data: [[0; 4]; 4],
            changed: false,
        }
    }

    fn spawn_number(&mut self) -> Result<(), ()> {
        let cell = self.get_random_empty_cell()?;

        // set the value of the random empty cell to 2 or 4 with ratio 65:35
        let random_value = Bernoulli::new(0.75)
            .unwrap()
            .sample(&mut rand::thread_rng());

        writeln!(stdout(), "{:?}", cell).unwrap();

        self.data[cell.0][cell.1] = if random_value { 2 } else { 4 };

        Ok(())
    }

    fn get_random_empty_cell(&self) -> Result<(usize, usize), ()> {
        let empty_cells = self.get_empty_cells();

        if empty_cells.len() == 0 {
            Err(())
        } else {
            Ok(empty_cells[rand::thread_rng().gen_range(0..empty_cells.len())])
        }
    }

    fn get_empty_cells(&self) -> Vec<(usize, usize)> {
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

    fn update(&mut self) -> Result<(), ()> {
        self.spawn_number()?;
        queue!(
            stdout(),
            Clear(ClearType::All),
            MoveToRow(0),
            MoveToColumn(0)
        )
        .unwrap();
        write!(stdout(), "{}", self).expect("could not write update");

        Ok(())
    }

    fn move_up(&mut self) {
        for i in 0..4 {
            let mut swap = false;
            for j in 0..4 {
                if self.data[3 - j][i] != 0 {
                    swap = true;
                } else if swap {
                    self.data[3 - j][i] = self.data[3 - j + 1][i];
                    self.data[3 - j + 1][i] = 0;
                    self.changed = true;
                }
            }
        }
    }

    fn move_right(&mut self) {
        for i in 0..4 {
            let mut swap = false;
            for j in 0..4 {
                if self.data[i][j] != 0 {
                    swap = true;
                } else if swap {
                    self.data[i][j] = self.data[i][j - 1];
                    self.data[i][j - 1] = 0;
                    self.changed = true;
                }
            }
        }
    }

    fn move_down(&mut self) {
        for i in 0..4 {
            let mut swap = false;
            for j in 0..4 {
                if self.data[j][i] != 0 {
                    swap = true;
                } else if swap {
                    self.data[j][i] = self.data[j - 1][i];
                    self.data[j - 1][i] = 0;
                    self.changed = true;
                }
            }
        }
    }

    fn move_left(&mut self) {
        for i in 0..4 {
            let mut swap = false;
            for j in 0..4 {
                if self.data[i][3 - j] != 0 {
                    swap = true;
                } else if swap {
                    self.data[i][3 - j] = self.data[i][3 - j + 1];
                    self.data[i][3 - j + 1] = 0;
                    self.changed = true;
                }
            }
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            for j in 0..4 {
                write!(f, "{} ", self.data[i][j])?;
            }
            write!(f, "\r\n")?;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut matrix = Matrix::new();

    enable_raw_mode()?;

    matrix.update().unwrap();

    let mut exit = false;
    loop {
        match read()? {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Left => matrix.move_left(),
                KeyCode::Right => matrix.move_right(),
                KeyCode::Up => matrix.move_up(),
                KeyCode::Down => matrix.move_down(),
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
            match matrix.update() {
                Ok(_) => {}
                Err(_) => {
                    write!(stdout(), "You lose!\r\n")?;
                    sleep(Duration::from_secs(3));
                    exit = true;
                }
            }
            matrix.changed = false;
        }

        if exit {
            break;
        }
    }

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
