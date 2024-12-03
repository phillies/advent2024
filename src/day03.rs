use std::{
    sync::{Arc, Mutex},
    thread,
};

use once_cell::sync::Lazy;
use rayon::prelude::*;

use regex::Regex;

// Find all occurrences of `mul(n,m)` where n and m are integers
// and we return the integers n and m as captured groups
static MUL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

pub fn solve(filename: &String) -> (i64, i64) {
    let input = std::fs::read_to_string(filename).expect("Could not read file!");
    day_03(&input)
}

fn calculate_sum_of_mul(input_data: &String) -> i64 {
    // This lets the function sleep for 1 ms to simulate a slow calculation
    // uncomment to try it out
    // thread::sleep(std::time::Duration::from_millis(1));

    MUL_REGEX
        .captures_iter(input_data)
        .filter_map(|cap| {
            // The captures contains the 2 blocks of digits
            let first = cap[1].parse::<i64>().unwrap();
            let second = cap[2].parse::<i64>().unwrap();

            Some(first * second)
        })
        .sum()
}

// Find all substrings between `do()` and `don't()` where we assume that
// we start with a `do()` sequence.
fn get_substrings(input_data: &String) -> Vec<String> {
    let mut do_substrings = Vec::new();

    let mut start_index = 0;
    let mut do_enabled = true;
    let mut end_index;
    while start_index < input_data.len() {
        if do_enabled {
            end_index = input_data[start_index..]
                .find("don't()")
                // we add 7 because we don't need the length of `don't()`
                .map(|i| start_index + i + 7);
            // if we don't find a match, we take the rest of the input data
            let end_index = end_index.unwrap_or(input_data.len());

            let substring = input_data[start_index..end_index].to_string();
            do_substrings.push(substring);

            start_index = end_index;
            do_enabled = false;
        } else {
            end_index = input_data[start_index..]
                .find("do()")
                // we add 4 because we don't need the length of `do()`
                .map(|i| start_index + i + 4);
            let end_index = end_index.unwrap_or(input_data.len());
            start_index = end_index;
            do_enabled = true;
        }
    }
    do_substrings
}

// Manually create threads and run each substring in a separate thread
// Use a mutex to lock the sum variable and Arc to share it between threads
fn calculate_conditional_sum_of_mul_manual_multithreading(input_data: &String) -> i64 {
    let do_substrings = get_substrings(input_data);

    let sum = Arc::new(Mutex::new(0));
    let handles: Vec<_> = do_substrings
        .into_iter()
        .map(|s| {
            let sum = Arc::clone(&sum);
            thread::spawn(move || {
                let result = calculate_sum_of_mul(&s);
                let mut sum_reference = sum.lock().unwrap();
                *sum_reference += result;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *sum.lock().unwrap();
    result
}

// Use rayon to parallelize the calculation of the sum
fn calculate_conditional_sum_of_mul(input_data: &String) -> i64 {
    let do_substrings = get_substrings(input_data);

    do_substrings
        // this is the rayon parallel iterator, will take care of everything
        .par_iter()
        .map(|s| calculate_sum_of_mul(s))
        .sum()
}

// Simple sequential calculation of the sum of products
fn calculate_conditional_sum_of_mul_sequentially(input_data: &String) -> i64 {
    let do_substrings = get_substrings(input_data);

    do_substrings.iter().map(|s| calculate_sum_of_mul(s)).sum()
}

fn day_03(input_data: &String) -> (i64, i64) {
    let result_1 = calculate_sum_of_mul(input_data);

    let result_2 = calculate_conditional_sum_of_mul(input_data);

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day03() {
        let result_1 = 161;
        let result_2 = 48;
        let input1 =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let input2 =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let output_1 = calculate_sum_of_mul(&input1);
        let output_2 = calculate_conditional_sum_of_mul(&input2);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day03_calc_sum_of_mul(b: &mut Bencher) {
        let filename = "data/day03.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        b.iter(|| {
            calculate_sum_of_mul(&input);
        });
    }

    #[bench]
    fn bench_day03_substrings_parallel(b: &mut Bencher) {
        let filename = "data/day03.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        MUL_REGEX.as_str();

        b.iter(|| {
            calculate_conditional_sum_of_mul(&input);
        });
    }

    #[bench]
    fn bench_day03_substrings_sequentially(b: &mut Bencher) {
        let filename = "data/day03.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        MUL_REGEX.as_str();

        b.iter(|| {
            calculate_conditional_sum_of_mul_sequentially(&input);
        });
    }

    #[bench]
    fn bench_day03_substrings_test_manual_implementation(b: &mut Bencher) {
        let filename = "data/day03.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        MUL_REGEX.as_str();

        b.iter(|| {
            calculate_conditional_sum_of_mul_manual_multithreading(&input);
        });
    }
}
