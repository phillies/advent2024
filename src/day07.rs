use rayon::prelude::*;

use crate::my_io::read_input_to_vector;

fn parse_input_string(input_string: &String) -> (i64, Vec<i64>) {
    let splits = input_string.split(":").collect::<Vec<&str>>();
    let target_value = splits[0].parse::<i64>().unwrap();
    let values = splits[1]
        .trim()
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (target_value, values)
}

fn calculate_tree(previous_values: &Vec<i64>, value: i64) -> Vec<i64> {
    let mut result = vec![];
    previous_values.iter().for_each(|&p| {
        let new_value = p * value;
        result.push(new_value);
        let new_value = p + value;
        result.push(new_value);
    });

    result
}

fn calculate_tree_part_2(previous_values: &Vec<i64>, value: i64) -> Vec<i64> {
    let mut result = vec![];
    previous_values.iter().for_each(|&p| {
        let new_value = p * value;
        result.push(new_value);
        let new_value = p + value;
        result.push(new_value);
        let new_value = format!("{}{}", p, value).parse::<i64>().unwrap();
        result.push(new_value);
    });

    result
}

pub fn solve(filename: &String) -> (i64, i64) {
    let input = read_input_to_vector(filename);
    day_07(&input)
}

fn check_for_valid_combination(
    combinations: &Vec<(i64, Vec<i64>)>,
    calculate_function: fn(&Vec<i64>, i64) -> Vec<i64>,
) -> Vec<bool> {
    combinations
        .par_iter()
        .map(|(target_value, values)| {
            let mut sums = vec![values[0]];
            values.iter().skip(1).for_each(|v| {
                sums = calculate_function(&sums, *v);
            });
            sums.contains(target_value)
        })
        .collect()
}

fn get_total_sum(combinations: &Vec<(i64, Vec<i64>)>, validity: &Vec<bool>) -> i64 {
    combinations
        .iter()
        .zip(validity)
        .map(
            |((target_value, _), valid)| {
                if *valid {
                    *target_value
                } else {
                    0
                }
            },
        )
        .sum()
}

fn day_07(input_data: &Vec<String>) -> (i64, i64) {
    let combinations = input_data
        .iter()
        .map(|s| parse_input_string(s))
        .collect::<Vec<(i64, Vec<i64>)>>();

    let validity = check_for_valid_combination(&combinations, calculate_tree);

    let result_1 = get_total_sum(&combinations, &validity);

    let validity_2 = check_for_valid_combination(&combinations, calculate_tree_part_2);

    let result_2 = get_total_sum(&combinations, &validity_2);

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day07() {
        let result_1 = 3749;
        let result_2 = 11387;
        let input = vec![
            "190: 10 19".to_string(),
            "3267: 81 40 27".to_string(),
            "83: 17 5".to_string(),
            "156: 15 6".to_string(),
            "7290: 6 8 6 15".to_string(),
            "161011: 16 10 13".to_string(),
            "192: 17 8 14".to_string(),
            "21037: 9 7 18 13".to_string(),
            "292: 11 6 16 20".to_string(),
        ];
        let (output_1, output_2) = day_07(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day07(b: &mut Bencher) {
        let filename = "data/day07.txt";
        let input = read_input_to_vector(filename);
        b.iter(|| {
            day_07(&input);
        });
    }
}
