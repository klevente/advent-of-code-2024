use std::collections::HashMap;
use itertools::Itertools;
use advent_of_code_2024::read_file_lines_as;

fn calculate_total_distance(first_column: &Vec<u64>, second_column: &Vec<u64>) -> u64 {
    first_column.iter().sorted().zip(second_column.iter().sorted()).map(|(x, y)| x.abs_diff(*y)).sum()
}

fn calculate_similarity_score(first_column: &Vec<u64>, second_column: &Vec<u64>) -> u64 {
    let freq_map = second_column.iter().fold(HashMap::new(), |mut acc, &elem| {
        *acc.entry(elem).or_insert(0u64) += 1;
        acc
    });

    first_column.iter().fold(0, |acc, &elem| {
        acc + elem * freq_map.get(&elem).unwrap_or(&0)
    })
}

fn main() {
    let input  = read_file_lines_as("input/day1.txt", |line| {
        let mut row = line.split_whitespace().map(|elem| elem.parse::<u64>().expect("elem should be a number"));

        (row.next().expect("row should have 2 elements"), row.next().expect("row should have 2 elements"))
    });

    let (first_column, second_column): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let total_distance = calculate_total_distance(&first_column, &second_column);

    println!("The total distance is: {total_distance}");

    let mut freq_map = HashMap::new();

    for &item in &second_column {
        *freq_map.entry(item).or_insert(0u64) += 1;
    }

    let similarity_score = calculate_similarity_score(&first_column, &second_column);

    println!("The similarity score is: {similarity_score}");
}