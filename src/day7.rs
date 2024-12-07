use advent_of_code_2024::read_file_lines_as;

fn num_of_digits_of_base_10_number(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn can_calibration_be_made_true_with_addition_and_multiplication(
    result: u64,
    values: &[u64],
) -> bool {
    if values.is_empty() {
        return result == 0;
    }

    fn check(result: u64, values: &[u64], current: u64, idx: usize) -> bool {
        if idx == values.len() {
            return current == result;
        }
        if current > result {
            return false;
        }
        let n = values[idx];
        let added = current + n;
        let multiplied = current * n;

        let with_addition = check(result, values, added, idx + 1);
        let with_multiplication = check(result, values, multiplied, idx + 1);

        with_addition || with_multiplication
    }

    check(result, values, values[0], 1)
}

fn can_calibration_be_made_true_with_addition_multiplication_and_concatenation(
    result: u64,
    values: &[u64],
) -> bool {
    if values.is_empty() {
        return result == 0;
    }

    fn check(result: u64, values: &[u64], current: u64, idx: usize) -> bool {
        if idx == values.len() {
            return current == result;
        }
        if current > result {
            return false;
        }
        let n = values[idx];
        let added = current + n;
        let multiplied = current * n;
        let concatenated = {
            // 12 || 345 = 12000 + 345 = 12 * (floor(log10(345)) + 1) + 345
            let n_num_of_digits = num_of_digits_of_base_10_number(n);
            let to_shift = 10u64.pow(n_num_of_digits);
            current * to_shift + n
        };

        let with_addition = check(result, values, added, idx + 1);
        let with_multiplication = check(result, values, multiplied, idx + 1);
        let with_concatenation = check(result, values, concatenated, idx + 1);

        with_addition || with_multiplication || with_concatenation
    }

    check(result, values, values[0], 1)
}

fn calculate_sum_of_calculations_made_true_with_addition_and_multiplication(
    calculations: &[(u64, Vec<u64>)],
) -> u64 {
    calculations
        .into_iter()
        .filter(|(result, values)| {
            can_calibration_be_made_true_with_addition_and_multiplication(*result, values)
        })
        .map(|(result, _)| result)
        .sum()
}

fn calculate_sum_of_calculations_made_true_with_addition_multiplication_and_concatenation(
    calculations: &[(u64, Vec<u64>)],
) -> u64 {
    calculations
        .into_iter()
        .filter(|(result, values)| {
            can_calibration_be_made_true_with_addition_multiplication_and_concatenation(
                *result, values,
            )
        })
        .map(|(result, _)| result)
        .sum()
}

fn main() {
    let calculations = read_file_lines_as("input/day7.txt", |line| {
        let (result, values_str) = line.split_once(": ").unwrap();
        let result = result.parse::<u64>().unwrap();
        let values = values_str
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        (result, values)
    });

    let sum_of_calculations_made_true_with_addition_and_multiplication =
        calculate_sum_of_calculations_made_true_with_addition_and_multiplication(&calculations);
    println!("The sum of calculations that can be made true with addition and multiplication is {sum_of_calculations_made_true_with_addition_and_multiplication}");

    let sum_of_calculations_made_true_with_addition_multiplication_and_concatenation =
        calculate_sum_of_calculations_made_true_with_addition_multiplication_and_concatenation(
            &calculations,
        );
    println!("The sum of calculations that can be made true with addition, multiplication and concatenation is {sum_of_calculations_made_true_with_addition_multiplication_and_concatenation}");
}
