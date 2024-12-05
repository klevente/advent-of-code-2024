use advent_of_code_2024::read_file_to_string;
use std::collections::HashSet;

fn is_valid_update(update: &Vec<u32>, rules: &HashSet<(u32, u32)>) -> bool {
    for (i, _) in update.iter().enumerate() {
        let n = update[i];
        for j in 0..i {
            let m = update[j];
            if rules.contains(&(n, m)) {
                return false;
            }
        }
    }

    true
}

fn middle_of_vec<T: Copy>(v: &Vec<T>) -> T {
    v[v.len() / 2]
}

fn calculate_sum_of_middle_values_of_valid_updates(
    updates: &[Vec<u32>],
    rules: &HashSet<(u32, u32)>,
) -> u32 {
    updates
        .iter()
        .filter(|u| is_valid_update(u, &rules))
        .map(|u| middle_of_vec(&u))
        .sum::<u32>()
}

fn check_validity_at(
    update: &Vec<u32>,
    at: usize,
    rules: &HashSet<(u32, u32)>,
) -> Option<(usize, usize)> {
    let n = update[at];
    for i in 0..at {
        let m = update[i];
        if rules.contains(&(n, m)) {
            return Some((i, at));
        }
    }

    None
}

fn fix_invalid_update(update: &Vec<u32>, rules: &HashSet<(u32, u32)>) -> Vec<u32> {
    let mut fixed = update.clone();
    let mut i = 0;
    loop {
        if i == update.len() {
            break;
        }
        if let Some(to_swap) = check_validity_at(&fixed, i, rules) {
            fixed.swap(to_swap.0, to_swap.1);
            // Need to go back and re-check from the position we placed the later value to,
            // as we could've broken the array in some other way
            i = to_swap.0;
        } else {
            i += 1;
        }
    }

    fixed
}

fn calculate_sum_of_middle_values_of_fixed_updates(
    updates: &[Vec<u32>],
    rules: &HashSet<(u32, u32)>,
) -> u32 {
    updates
        .iter()
        .filter(|u| !is_valid_update(u, &rules))
        .map(|u| fix_invalid_update(u, rules))
        .map(|u| middle_of_vec(&u))
        .sum::<u32>()
}

fn main() {
    let input = read_file_to_string("input/day5.txt");
    let (rules, updates) = input
        .split_once("\r\n\r\n")
        .expect("input should be properly formatted");

    let rules = rules
        .lines()
        .map(|l| {
            let (a, b) = l
                .split_once('|')
                .expect("input should be properly formatted");
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect::<HashSet<_>>();

    let updates = updates
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum_of_middle_values_of_valid_updates =
        calculate_sum_of_middle_values_of_valid_updates(&updates, &rules);
    println!(
        "The sum of middle values of valid updates is {sum_of_middle_values_of_valid_updates}"
    );

    let sum_of_middle_values_of_fixed_updates =
        calculate_sum_of_middle_values_of_fixed_updates(&updates, &rules);
    println!(
        "The sum of middle values of fixed updates is {sum_of_middle_values_of_fixed_updates}"
    );
}
