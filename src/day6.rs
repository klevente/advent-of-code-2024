use advent_of_code_2024::{read_file_to_string, Coords2D};
use anyhow::Context;
use array2d::Array2D;
use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Tile {
    Empty,
    Obstacle,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Obstacle => '#',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Array2D<Tile>,
    guard_position: Coords2D,
    direction: Direction,
    visited_coords: HashSet<Coords2D>,
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let guard_row_idx = value
            .lines()
            .position(|l| l.contains('^'))
            .context("Guard character (^) not found")?;
        let guard_col_idx = value
            .lines()
            .nth(guard_row_idx)
            .expect("Guard character should exist as we just checked it")
            .chars()
            .position(|c| c == '^')
            .expect("Guard character should exist as we just checked it");

        let tiles = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' | '^' => Ok(Tile::Empty),
                        '#' => Ok(Tile::Obstacle),
                        c => anyhow::bail!("Unexpected character in input: {c}"),
                    })
                    .collect::<anyhow::Result<Vec<_>>>()
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let tiles =
            Array2D::from_rows(&tiles).context("Malformed input: rows are not the same length")?;
        let guard_position = Coords2D::new(guard_row_idx, guard_col_idx);
        Ok(Self {
            tiles,
            guard_position,
            direction: Direction::Up,
            visited_coords: HashSet::from([guard_position]),
        })
    }
}

impl Map {
    fn tick(&mut self) -> bool {
        let next_pos = match self.direction {
            Direction::Up => {
                if self.guard_position.row == 0 {
                    return false;
                }
                self.guard_position.move_up()
            }
            Direction::Right => {
                if self.guard_position.column == self.tiles.num_columns() - 1 {
                    return false;
                }
                self.guard_position.move_right()
            }
            Direction::Down => {
                if self.guard_position.row == self.tiles.num_rows() - 1 {
                    return false;
                }
                self.guard_position.move_down()
            }
            Direction::Left => {
                if self.guard_position.column == 0 {
                    return false;
                }
                self.guard_position.move_left()
            }
        };

        let tile = self
            .tiles
            .get(next_pos.row, next_pos.column)
            .expect("Should be a valid index since we checked it above");
        if let Tile::Obstacle = tile {
            self.direction = self.direction.turn();
            self.tick()
        } else {
            self.visited_coords.insert(next_pos);
            self.guard_position = next_pos;
            true
        }
    }

    fn count_number_of_unique_visited_tiles(&mut self) -> usize {
        while self.tick() {}

        self.visited_coords.len()
    }
}

fn main() {
    let input = read_file_to_string("input/day6.txt");
    let mut map = Map::try_from(input.as_str()).unwrap();

    let number_of_unique_visited_tiles = map.count_number_of_unique_visited_tiles();
    println!("The number of unique tiles the guard visits is {number_of_unique_visited_tiles}")
}
