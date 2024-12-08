use advent_of_code_2024::{read_file_to_string, Coords2D};
use anyhow::Context;
use array2d::Array2D;
use std::collections::{HashMap, HashSet};

struct Map {
    tiles: Array2D<char>,
    antennas: HashMap<char, Vec<Coords2D>>,
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tiles = value
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let tiles =
            Array2D::from_rows(&tiles).context("Malformed input: rows are not the same length")?;

        let mut antennas = HashMap::<char, Vec<_>>::new();

        for (row, col) in tiles.indices_row_major() {
            let tile = tiles.get(row, col).expect("Should be a valid index");
            if *tile == '.' {
                continue;
            }

            antennas
                .entry(*tile)
                .or_default()
                .push(Coords2D::new(row, col));
        }

        Ok(Self { tiles, antennas })
    }
}

impl Map {
    fn in_bounds(&self, row: isize, col: isize) -> bool {
        let num_rows = self.tiles.num_rows() as isize;
        let num_columns = self.tiles.num_columns() as isize;
        row >= 0 && row < num_rows && col >= 0 && col < num_columns
    }

    fn get_antinodes_for_pair(
        &self,
        antenna_a: Coords2D,
        antenna_b: Coords2D,
    ) -> (Option<Coords2D>, Option<Coords2D>) {
        let v_row = antenna_b.row as isize - antenna_a.row as isize;
        let v_col = antenna_b.column as isize - antenna_a.column as isize;

        let v_inv_row = -v_row;
        let v_inv_col = -v_col;

        let antinode_b_row = antenna_b.row as isize + v_row;
        let antinode_b_col = antenna_b.column as isize + v_col;

        let antinode_a_row = antenna_a.row as isize + v_inv_row;
        let antinode_a_col = antenna_a.column as isize + v_inv_col;

        let antinode_a = if self.in_bounds(antinode_a_row, antinode_a_col) {
            Some(Coords2D::new(
                antinode_a_row as usize,
                antinode_a_col as usize,
            ))
        } else {
            None
        };

        let antinode_b = if self.in_bounds(antinode_b_row, antinode_b_col) {
            Some(Coords2D::new(
                antinode_b_row as usize,
                antinode_b_col as usize,
            ))
        } else {
            None
        };

        (antinode_a, antinode_b)
    }

    fn get_pairs_for_antenna(&self, antenna: char) -> HashSet<(Coords2D, Coords2D)> {
        let Some(positions) = self.antennas.get(&antenna) else {
            return HashSet::new();
        };

        let mut ret = HashSet::new();

        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                ret.insert((positions[i], positions[j]));
            }
        }

        ret
    }

    fn get_antinodes_for_antenna(&self, antenna: char) -> HashSet<Coords2D> {
        let pairs = self.get_pairs_for_antenna(antenna);
        let mut res = HashSet::new();
        for (a, b) in pairs {
            let (antinode_a, antinode_b) = self.get_antinodes_for_pair(a, b);
            if let Some(antinode_a) = antinode_a {
                res.insert(antinode_a);
            }
            if let Some(antinode_b) = antinode_b {
                res.insert(antinode_b);
            }
        }

        res
    }

    fn get_antinode_positions(&self) -> HashSet<Coords2D> {
        self.antennas
            .keys()
            .flat_map(|antenna| self.get_antinodes_for_antenna(*antenna))
            .collect()
    }

    fn get_num_of_antinodes(&self) -> usize {
        self.get_antinode_positions().len()
    }

    fn get_antinodes_for_antenna_with_resonant_harmonics(
        &self,
        antenna: char,
    ) -> HashSet<Coords2D> {
        let pairs = self.get_pairs_for_antenna(antenna);
        let mut res = HashSet::new();

        for (antenna_a, antenna_b) in pairs {
            let v_row = antenna_b.row as isize - antenna_a.row as isize;
            let v_col = antenna_b.column as isize - antenna_a.column as isize;

            let mut walker_row = antenna_a.row as isize;
            let mut walker_col = antenna_a.column as isize;
            while self.in_bounds(walker_row, walker_col) {
                res.insert(Coords2D::new(walker_row as usize, walker_col as usize));
                walker_row += v_row;
                walker_col += v_col;
            }

            let mut walker_row = antenna_a.row as isize;
            let mut walker_col = antenna_a.column as isize;
            while self.in_bounds(walker_row, walker_col) {
                res.insert(Coords2D::new(walker_row as usize, walker_col as usize));
                walker_row -= v_row;
                walker_col -= v_col;
            }
        }

        res
    }

    fn get_antinodes_with_resonant_harmonics(&self) -> HashSet<Coords2D> {
        self.antennas
            .keys()
            .flat_map(|antenna| self.get_antinodes_for_antenna_with_resonant_harmonics(*antenna))
            .collect()
    }

    fn get_num_of_antinodes_with_resonant_harmonics(&self) -> usize {
        self.get_antinodes_with_resonant_harmonics().len()
    }
}

fn main() {
    let input = read_file_to_string("input/day8.txt");
    let map = Map::try_from(input.as_str()).unwrap();

    let num_of_antinodes = map.get_num_of_antinodes();
    println!("The number of antinodes is {num_of_antinodes}");

    let num_of_antinodes_with_resonant_harmonics =
        map.get_num_of_antinodes_with_resonant_harmonics();
    println!("The number of antinodes with resonant harmonics is {num_of_antinodes_with_resonant_harmonics}");
}
