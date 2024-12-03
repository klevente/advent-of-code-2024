use regex::Regex;
use advent_of_code_2024::read_file_to_string;

fn calculate_mul_sum(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(&input)
        .map(|instruction| {
            let num1 = instruction.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let num2 = instruction.get(2).unwrap().as_str().parse::<i32>().unwrap();
            (num1, num2)
        })
        .map(|(lhs, rhs)| lhs * rhs).sum::<i32>()
}

fn calculate_mul_sum_with_conditionals(input: &str) -> i32 {
    let re = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    let mut ptr = input;

    while !ptr.is_empty() {
        if let Some(instruction) = re.captures(ptr) {
            let match_len = instruction.get(0).unwrap().len();
            let lhs = instruction.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let rhs = instruction.get(2).unwrap().as_str().parse::<i32>().unwrap();

            if enabled {
                sum += lhs * rhs;
            }

            ptr = &ptr[match_len..];
        } else if ptr.starts_with("do()") {
            enabled = true;
            ptr = &ptr[4..];
        } else if ptr.starts_with("don't") {
            enabled = false;
            ptr = &ptr[5..];
        } else {
            ptr = &ptr[1..];
        }
    }

    sum
}

fn main() {
    let input = read_file_to_string("input/day3.txt");

    let mul_sum = calculate_mul_sum(&input);
    println!("The sum of all multiplication instructions is {mul_sum}");

    let mul_sum_conditionals = calculate_mul_sum_with_conditionals(&input);
    println!("The sum of all multiplication instructions with conditionals is {mul_sum_conditionals}");
}