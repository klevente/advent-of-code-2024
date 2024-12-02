use advent_of_code_2024::read_file_lines_as;
use itertools::Itertools;

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(levels: Vec<i32>) -> Self {
        Self { levels }
    }

    fn with_level_removed(&self, idx: usize) -> Self {
        let new_levels = self
            .levels
            .iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| *i != idx)
            .map(|(_, l)| l)
            .collect();

        Self { levels: new_levels }
    }

    fn is_safe(&self) -> bool {
        if self.levels.len() < 2 {
            return true;
        }

        let first = self.levels[0];
        let second = self.levels[1];

        if first == second {
            return false;
        }

        let ascending = first < second;

        !self.levels.iter().tuple_windows().any(|(&prev, &current)| {
            (ascending && ((current - prev) < 1 || (current - prev) > 3))
                || (!ascending && ((prev - current) < 1 || (prev - current) > 3))
        })
    }

    fn is_safe_with_dampener(&self) -> bool {
        (0..self.levels.len())
            .map(|i| self.with_level_removed(i))
            .any(|report| report.is_safe())
    }
}

fn calculate_num_of_safe_reports(reports: &[Report]) -> usize {
    reports.iter().filter(|report| report.is_safe()).count()
}

fn calculate_num_of_safe_reports_with_dampener(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| report.is_safe_with_dampener())
        .count()
}

fn main() {
    let reports = read_file_lines_as("input/day2.txt", |line| {
        let levels = line
            .split_whitespace()
            .map(|v| v.parse().expect("value should be a number"))
            .collect();
        Report::new(levels)
    });
    let num_of_safe_reports = calculate_num_of_safe_reports(&reports);
    println!("The number of safe reports is {num_of_safe_reports}");

    let num_of_safe_reports_with_dampener = calculate_num_of_safe_reports_with_dampener(&reports);
    println!("The number of safe reports with dampener is {num_of_safe_reports_with_dampener}")
}
