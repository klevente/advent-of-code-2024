use advent_of_code_2024::{read_2d_char_array, read_file_to_string, Coords};
use array2d::Array2D;
use phf::phf_map;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

const DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

impl Direction {
    fn all_directions_iter() -> impl Iterator<Item = Self> {
        DIRECTIONS.iter().copied()
    }
}

const NEXT_MAP: phf::Map<char, char> = phf_map! {
    'X' => 'M',
    'M' => 'A',
    'A' => 'S'
};

fn step_in_direction(chars: &Array2D<char>, pos: Coords, direction: Direction) -> Option<Coords> {
    let num_columns = chars.row_len();
    let num_rows = chars.column_len();

    match direction {
        Direction::Up => {
            if pos.row == 0 {
                None
            } else {
                Some(Coords::new(pos.row - 1, pos.column))
            }
        }
        Direction::UpRight => {
            if pos.row == 0 || pos.column == num_columns - 1 {
                None
            } else {
                Some(Coords::new(pos.row - 1, pos.column + 1))
            }
        }
        Direction::Right => {
            if pos.column == num_columns - 1 {
                None
            } else {
                Some(Coords::new(pos.row, pos.column + 1))
            }
        }
        Direction::DownRight => {
            if pos.row == num_rows - 1 || pos.column == num_columns - 1 {
                None
            } else {
                Some(Coords::new(pos.row + 1, pos.column + 1))
            }
        }
        Direction::Down => {
            if pos.row == num_rows - 1 {
                None
            } else {
                Some(Coords::new(pos.row + 1, pos.column))
            }
        }
        Direction::DownLeft => {
            if pos.row == num_rows - 1 || pos.column == 0 {
                None
            } else {
                Some(Coords::new(pos.row + 1, pos.column - 1))
            }
        }
        Direction::Left => {
            if pos.column == 0 {
                None
            } else {
                Some(Coords::new(pos.row, pos.column - 1))
            }
        }
        Direction::UpLeft => {
            if pos.row == 0 || pos.column == 0 {
                None
            } else {
                Some(Coords::new(pos.row - 1, pos.column - 1))
            }
        }
    }
}

fn search_word_in_direction_from(
    chars: &Array2D<char>,
    current_pos: Coords,
    direction: Direction,
    char_to_check: char,
) -> bool {
    let Some(current_char) = chars.get(current_pos.row, current_pos.column) else {
        return false;
    };

    if *current_char != char_to_check {
        return false;
    }

    // We've reached the end of the word
    if char_to_check == 'S' {
        return true;
    }

    let Some(next) = step_in_direction(chars, current_pos, direction) else {
        return false;
    };
    let next_char = NEXT_MAP.get(&char_to_check).unwrap();
    search_word_in_direction_from(chars, next, direction, *next_char)
}

fn find_num_of_xmas_in_grid(chars: &Array2D<char>) -> usize {
    chars
        .indices_row_major()
        .map(|(row, column)| {
            let pos = Coords::new(row, column);
            Direction::all_directions_iter()
                .filter(|d| search_word_in_direction_from(&chars, pos, *d, 'X'))
                .count()
        })
        .sum()
}

fn has_cross_mas(chars: &Array2D<char>, pos: Coords) -> bool {
    let Some(current_char) = chars.get(pos.row, pos.column) else {
        return false;
    };

    if *current_char != 'A' {
        return false;
    };

    let Some(&up_right) =
        step_in_direction(chars, pos, Direction::UpRight).and_then(|p| chars.get(p.row, p.column))
    else {
        return false;
    };

    let Some(&down_right) = step_in_direction(chars, pos, Direction::DownRight)
        .and_then(|p| chars.get(p.row, p.column))
    else {
        return false;
    };

    let Some(&down_left) =
        step_in_direction(chars, pos, Direction::DownLeft).and_then(|p| chars.get(p.row, p.column))
    else {
        return false;
    };

    let Some(&up_left) =
        step_in_direction(chars, pos, Direction::UpLeft).and_then(|p| chars.get(p.row, p.column))
    else {
        return false;
    };

    let has_first_arm = up_right == 'M' && down_left == 'S' || up_right == 'S' && down_left == 'M';
    let has_second_arm = up_left == 'M' && down_right == 'S' || up_left == 'S' && down_right == 'M';

    has_first_arm && has_second_arm
}

fn find_num_of_cross_mas_in_grid(chars: &Array2D<char>) -> usize {
    chars
        .indices_row_major()
        .filter(|(row, column)| has_cross_mas(chars, Coords::new(*row, *column)))
        .count()
}

fn main() {
    let chars = read_2d_char_array(&read_file_to_string("input/day4.txt"));
    let num_of_xmas = find_num_of_xmas_in_grid(&chars);
    println!("The number of times XMAS appears in the word search is {num_of_xmas}");

    let num_of_cross_mas = find_num_of_cross_mas_in_grid(&chars);
    println!("The number of cross MAS-es in the word search is {num_of_cross_mas}");
}
