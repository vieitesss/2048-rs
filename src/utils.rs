use rand::Rng;

pub fn rev(vector: &[u32]) -> Vec<u32> {
    vector.iter().rev().copied().collect()
}

pub fn get_random_empty_cell(data: &[Vec<u32>], width: usize) -> (usize, usize) {
    let empty_cells = get_empty_cells(data, width);

    assert!(!empty_cells.is_empty());

    empty_cells[rand_in_range(0, empty_cells.len())]
}

pub fn get_empty_cells(data: &[Vec<u32>], width: usize) -> Vec<(usize, usize)> {
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

fn rand_in_range(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}
