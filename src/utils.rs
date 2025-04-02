use crossterm::{
    cursor, queue,
    terminal::{Clear, ClearType},
};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::io::{self, stdout, Write};

pub fn rev(vector: &[u32]) -> Vec<u32> {
    vector.iter().rev().copied().collect()
}

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
    data.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell == 0)
                .map(move |(j, _)| (i, j))
        })
        .collect()
}

pub fn get_non_zeros(vector: &[u32]) -> Vec<u32> {
    vector.iter().filter(|&&x| x != 0).copied().collect()
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
