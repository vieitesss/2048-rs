use crossterm::{
    cursor, queue,
    terminal::{Clear, ClearType},
};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::io::{self, stdout, Write};

pub fn get_random_empty_cell(data: &[Vec<u32>]) -> (usize, usize) {
    let empty_cells = get_empty_cells(data);

    assert!(!empty_cells.is_empty());

    empty_cells[rand_in_range(0, empty_cells.len())]
}

pub fn get_random_bool(prob: f64) -> bool {
    Bernoulli::new(prob)
        .unwrap()
        .sample(&mut rand::thread_rng())
}

pub fn get_empty_cells(data: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut empty = Vec::new();

    for (i, row) in data.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                empty.push((i, j))
            }
        }
    }

    empty
}

pub fn get_non_zeros(vector: &[u32]) -> Vec<u32> {
    let mut non_zeros = Vec::new();

    for &x in vector.iter() {
        if x != 0 {
            non_zeros.push(x);
        }
    }

    non_zeros
}

fn rand_in_range(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}

pub fn clear_screen() -> Result<(), io::Error> {
    queue!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0),)?;

    stdout().flush().expect("could not flush stdout");

    Ok(())
}

pub fn get_window_size() -> (u16, u16) {
    crossterm::terminal::size().expect("could not get window size")
}
