use advent_of_code_2024::{parse_2d_number_grid, read_file_to_string, Coords2D, GetNeighbors};
use array2d::Array2D;
use std::collections::HashSet;

fn find_trailhead_score(grid: &Array2D<u8>, pos: Coords2D) -> usize {
    if grid.get(pos.row, pos.column).is_none_or(|v| *v != 0) {
        return 0;
    }

    let mut visited = HashSet::new();
    let mut peaks = HashSet::new();

    fn step(
        grid: &Array2D<u8>,
        pos: Coords2D,
        previous: Coords2D,
        visited: &mut HashSet<Coords2D>,
        peaks: &mut HashSet<Coords2D>,
    ) {
        if visited.contains(&pos) {
            return;
        }

        let current_height = grid[(pos.row, pos.column)];
        let previous_height = grid[(previous.row, previous.column)];
        if current_height != previous_height + 1 {
            return;
        }

        if current_height == 9 {
            peaks.insert(pos);
            return;
        }

        visited.insert(pos);

        for new_pos in grid.get_neighboring_indices_no_diagonal(pos) {
            step(grid, new_pos, pos, visited, peaks);
        }
    }

    for one_pos in grid.get_neighboring_indices_no_diagonal(pos) {
        step(grid, one_pos, pos, &mut visited, &mut peaks);
    }

    peaks.len()
}

fn find_trailhead_rating(grid: &Array2D<u8>, pos: Coords2D) -> usize {
    if grid.get(pos.row, pos.column).is_none_or(|v| *v != 0) {
        return 0;
    }

    let mut visited = HashSet::new();
    let mut result = 0;

    fn step(
        grid: &Array2D<u8>,
        pos: Coords2D,
        previous: Coords2D,
        visited: &mut HashSet<Coords2D>,
        result: &mut usize,
    ) {
        if visited.contains(&pos) {
            return;
        }

        let current_height = grid[(pos.row, pos.column)];
        let previous_height = grid[(previous.row, previous.column)];
        if current_height != previous_height + 1 {
            return;
        }

        if current_height == 9 {
            *result += 1;
            return;
        }

        for new_pos in grid.get_neighboring_indices_no_diagonal(pos) {
            step(grid, new_pos, pos, visited, result);
        }
    }

    for one_pos in grid.get_neighboring_indices_no_diagonal(pos) {
        step(grid, one_pos, pos, &mut visited, &mut result);
    }

    result
}

fn find_trailheads(grid: &Array2D<u8>) -> impl Iterator<Item = Coords2D> + use<'_> {
    grid.indices_row_major().filter_map(|pos| {
        if grid[pos] == 0 {
            Some(pos.into())
        } else {
            None
        }
    })
}

fn find_sum_of_trailhead_scores(grid: &Array2D<u8>) -> usize {
    find_trailheads(&grid)
        .map(|pos| find_trailhead_score(&grid, pos))
        .sum()
}

fn find_sum_of_trailhead_ratings(grid: &Array2D<u8>) -> usize {
    find_trailheads(&grid)
        .map(|pos| find_trailhead_rating(&grid, pos))
        .sum()
}

fn main() {
    let input = read_file_to_string("input/day10.txt");

    let grid = parse_2d_number_grid(&input);

    let sum_of_trailhead_scores = find_sum_of_trailhead_scores(&grid);
    println!("The sum of trailhead scores is {sum_of_trailhead_scores}");

    let sum_of_trailhead_ratings = find_sum_of_trailhead_ratings(&grid);
    println!("The sum of trailhead ratings is {sum_of_trailhead_ratings}");
}
