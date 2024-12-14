use array2d::Array2D;
use std::{convert::TryInto, fs::read_to_string, path::Path};

pub fn read_file_to_string(path: impl AsRef<Path>) -> String {
    read_to_string(path).unwrap()
}

pub fn read_file_lines(path: impl AsRef<Path>) -> Vec<String> {
    read_file_lines_as(path, str::to_string)
}

pub fn read_file_lines_as<T>(path: impl AsRef<Path>, f: fn(&str) -> T) -> Vec<T> {
    let contents = read_file_to_string(path);
    contents.lines().map(f).collect()
}

pub fn read_file_lines_filter_as<T>(path: impl AsRef<Path>, f: fn(&str) -> Option<T>) -> Vec<T> {
    let contents = read_file_to_string(path);
    contents.lines().filter_map(f).collect()
}

pub fn read_file_lines_extract_first(path: impl AsRef<Path>) -> (String, Vec<String>) {
    let mut input = read_file_lines(path)
        .iter()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.to_owned())
            }
        })
        .collect::<Vec<_>>();
    let first = input.remove(0);

    (first, input)
}

pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn print_2d_array<T: Clone + std::fmt::Display>(array: &Array2D<T>) {
    for row in array.rows_iter() {
        for column in row.into_iter() {
            print!("{column}",);
        }
        println!();
    }
}

pub fn print_u8_2d_array_with_delim(array: &Array2D<u8>) {
    for row in array.rows_iter() {
        print!("|");
        for column in row.into_iter() {
            print!("{column:2}|",);
        }
        println!();
    }
}

pub fn print_usize_2d_array_with_delim(array: &Array2D<usize>) {
    for row in array.rows_iter() {
        print!("|");
        for column in row.into_iter() {
            print!("{column:5}|",);
        }
        println!();
    }
}

pub fn read_2d_char_array(s: &str) -> Array2D<char> {
    let elements = &*s
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Array2D::from_rows(elements).unwrap()
}

pub fn parse_2d_number_grid(s: &str) -> Array2D<u8> {
    let elements = &*s
        .lines()
        .map(|l| {
            l.chars()
                .map(|d| d.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Array2D::from_rows(elements).unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coords2D {
    pub row: usize,
    pub column: usize,
}

impl Coords2D {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn move_up(&self) -> Self {
        Self::new(self.row - 1, self.column)
    }

    pub fn move_up_right(&self) -> Self {
        Self::new(self.row - 1, self.column + 1)
    }

    pub fn move_right(&self) -> Self {
        Self::new(self.row, self.column + 1)
    }

    pub fn move_down_right(&self) -> Self {
        Self::new(self.row + 1, self.column + 1)
    }

    pub fn move_down(&self) -> Self {
        Self::new(self.row + 1, self.column)
    }

    pub fn move_down_left(&self) -> Self {
        Self::new(self.row + 1, self.column - 1)
    }

    pub fn move_left(&self) -> Self {
        Self::new(self.row, self.column - 1)
    }

    pub fn move_up_left(&self) -> Self {
        Self::new(self.row - 1, self.column - 1)
    }
}

impl std::fmt::Display for Coords2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row={}, col={})", self.row, self.column)
    }
}

impl From<(usize, usize)> for Coords2D {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub trait GetNeighbors<T> {
    fn get_neighboring_indices(&self, position: Coords2D) -> Vec<Coords2D>;

    fn get_neighboring_indices_no_diagonal(&self, position: Coords2D) -> Vec<Coords2D>;

    fn get_neighboring_values(&self, position: Coords2D) -> Vec<&T>;
}

impl<T> GetNeighbors<T> for Array2D<T> {
    fn get_neighboring_indices(&self, position: Coords2D) -> Vec<Coords2D> {
        let Coords2D { row, column } = position;
        let num_columns = self.row_len();
        let num_rows = self.column_len();

        let mut neighbors = Vec::with_capacity(8);

        if row > 0 {
            neighbors.push(Coords2D::new(row - 1, column));
        }
        if row > 0 && column > 0 {
            neighbors.push(Coords2D::new(row - 1, column - 1));
        }
        if column > 0 {
            neighbors.push(Coords2D::new(row, column - 1));
        }
        if row < num_rows - 1 && column > 0 {
            neighbors.push(Coords2D::new(row + 1, column - 1))
        }
        if row < num_rows - 1 {
            neighbors.push(Coords2D::new(row + 1, column));
        }
        if row < num_rows - 1 && column < num_columns - 1 {
            neighbors.push(Coords2D::new(row + 1, column + 1));
        }
        if column < num_columns - 1 {
            neighbors.push(Coords2D::new(row, column + 1));
        }
        if row > 0 && column < num_columns - 1 {
            neighbors.push(Coords2D::new(row - 1, column + 1))
        }

        neighbors
    }

    fn get_neighboring_indices_no_diagonal(&self, position: Coords2D) -> Vec<Coords2D> {
        let Coords2D { row, column } = position;
        let num_columns = self.row_len();
        let num_rows = self.column_len();

        let mut neighbors = Vec::with_capacity(4);

        if row > 0 {
            neighbors.push(Coords2D::new(row - 1, column));
        }
        if column > 0 {
            neighbors.push(Coords2D::new(row, column - 1));
        }
        if row < num_rows - 1 {
            neighbors.push(Coords2D::new(row + 1, column));
        }
        if column < num_columns - 1 {
            neighbors.push(Coords2D::new(row, column + 1));
        }

        neighbors
    }

    fn get_neighboring_values(&self, position: Coords2D) -> Vec<&T> {
        self.get_neighboring_indices(position)
            .into_iter()
            .map(|c| {
                self.get(c.row, c.column)
                    .expect("get_neighboring_indices should return only valid indices")
            })
            .collect()
    }
}
