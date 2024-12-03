use regex::Regex;

/// That's my template for new days. Includes the solve function signature and a test function
pub fn solve(filename: &String) -> (i64, i64) {
    let input = std::fs::read_to_string(filename).expect("Could not read file!");
    day_03(&input)
}

fn calculate_sum_of_mul(input_data: &String) -> i64 {
    let mut result = 0;

    // Find all occurrences of `mul( n , m )` where n and m are integers
    // and whitespace is allowed between the numbers (not sure if needed)
    let re = Regex::new(r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)").unwrap();

    for mat in re.find_iter(input_data) {
        // Split the match at the `,`` which leaves `mul(n` and `m)`, then split each
        // block at the brackets to get the numbers
        let first = mat.as_str().split(",").collect::<Vec<&str>>()[0]
            .split("(")
            .collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap();
        let second = mat.as_str().split(",").collect::<Vec<&str>>()[1]
            .split(")")
            .collect::<Vec<&str>>()[0]
            .parse::<i64>()
            .unwrap();

        result += first * second;
    }

    result
}

fn calculate_conditional_sum_of_mul(input_data: &String) -> i64 {
    let mut do_substrings = Vec::new();

    let mut start_index = 0;
    let mut do_enabled = true;
    let mut end_index;
    while start_index < input_data.len() {
        if do_enabled {
            // we could add +7 here to have the regex search string without the don't(), but meh
            end_index = input_data[start_index..]
                .find("don't()")
                .map(|i| start_index + i);
            // if we don't find a match, we take the rest of the input data
            let end_index = end_index.unwrap_or(input_data.len());
            let substring = input_data[start_index..end_index].to_string();
            do_substrings.push(substring);
            start_index = end_index;
            do_enabled = false;
        } else {
            end_index = input_data[start_index..]
                .find("do()")
                .map(|i| start_index + i);
            let end_index = end_index.unwrap_or(input_data.len());
            start_index = end_index;
            do_enabled = true;
        }
    }
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
    fn bench_day03_part1(b: &mut Bencher) {
        let filename = "data/day03.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        b.iter(|| {
            calculate_sum_of_mul(&input);
        });
    }
    #[bench]
    fn bench_day03_part2(b: &mut Bencher) {
        let filename = "data/day03.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");

        b.iter(|| {
            calculate_conditional_sum_of_mul(&input);
        });
    }
}
