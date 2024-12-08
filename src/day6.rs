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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
    starting_position: Coords2D,
    guard_position: Coords2D,
    direction: Direction,
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
            starting_position: guard_position,
            guard_position,
            direction: Direction::Up,
        })
    }
}

impl Map {
    fn reset(&mut self) {
        self.direction = Direction::Up;
        self.guard_position = self.starting_position;
    }

    fn step<F: FnMut(Coords2D, Direction) -> bool>(&mut self, mut callback: F) -> bool {
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
            self.step(callback)
        } else {
            if callback(next_pos, self.direction) {
                return false;
            }
            self.guard_position = next_pos;
            true
        }
    }

    fn simulate_and_get_unique_visited_tiles(&mut self) -> HashSet<Coords2D> {
        let mut visited_tiles = HashSet::from([self.guard_position]);

        while self.step(|coords2d: Coords2D, _| {
            visited_tiles.insert(coords2d);
            false
        }) {}

        visited_tiles
    }

    fn count_number_of_unique_visited_tiles(&mut self) -> usize {
        self.simulate_and_get_unique_visited_tiles().len()
    }

    fn with_additional_obstacle(&self, new_obstacle_pos: Coords2D) -> Self {
        let mut new_tiles = self.tiles.clone();
        new_tiles
            .set(
                new_obstacle_pos.row,
                new_obstacle_pos.column,
                Tile::Obstacle,
            )
            .expect("Should be valid coords since we got it from `tiles");
        Self {
            starting_position: self.starting_position,
            guard_position: self.starting_position,
            tiles: new_tiles,
            direction: Direction::Up,
        }
    }

    fn simulate_and_check_if_cycle(&mut self) -> bool {
        let mut unique_tiles_and_dirs = HashSet::from([(self.guard_position, self.direction)]);
        let mut is_cycle = false;

        while self.step(|coords, direction| {
            let is_new_tile = unique_tiles_and_dirs.insert((coords, direction));
            is_cycle = !is_new_tile;
            is_cycle
        }) {}

        is_cycle
    }

    fn count_number_of_added_obstacles_that_result_in_cycle(&mut self) -> usize {
        let unique_tiles = self.simulate_and_get_unique_visited_tiles();
        let mut cnt = 0;

        for potential_obstacle_tile in unique_tiles {
            if potential_obstacle_tile == self.starting_position {
                // Cannot place new obstacle on guard's starting position
                continue;
            }

            let mut new_map = self.with_additional_obstacle(potential_obstacle_tile);
            let does_result_in_cycle = new_map.simulate_and_check_if_cycle();
            if does_result_in_cycle {
                cnt += 1;
            }
        }

        cnt
    }
}

fn main() {
    let input = read_file_to_string("input/day6.txt");
    let mut map = Map::try_from(input.as_str()).unwrap();

    let number_of_unique_visited_tiles = map.count_number_of_unique_visited_tiles();
    println!("The number of unique tiles the guard visits is {number_of_unique_visited_tiles}");

    map.reset();

    let number_of_obstacles_that_cause_a_cycle =
        map.count_number_of_added_obstacles_that_result_in_cycle();
    println!("The number of obstacles we can place that causes a cycle is {number_of_obstacles_that_cause_a_cycle}");
}
