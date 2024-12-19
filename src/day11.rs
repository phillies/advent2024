use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn solve(filename: &String) -> (i64, i64) {
    let input = std::fs::read_to_string(filename).expect("Could not read file!");
    // let input = read_input_to_vector(filename);
    day_11(&input)
}

fn blink(input_row: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    input_row.iter().for_each(|row| {
        if row.parse::<u64>() == Ok(0) {
            result.push("1".to_string());
        } else if (row.len() % 2) == 0 {
            let left_half = row
                .chars()
                .take(row.len() / 2)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
                .to_string();
            let right_half = row
                .chars()
                .skip(row.len() / 2)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
                .to_string();
            result.push(left_half);
            result.push(right_half);
        } else {
            result.push((row.parse::<u64>().unwrap() * 2024).to_string());
        }
    });
    result
}

fn parallel_blink(
    input_row: &Vec<String>,
    subset_size: usize,
    minimum_split_number: usize,
) -> Vec<String> {
    if input_row.len() <= minimum_split_number * subset_size {
        return blink(input_row);
    }

    let chunks = input_row.chunks(subset_size);
    let results = chunks
        .par_bridge()
        .map(|chunk| blink(&chunk.to_vec()))
        .collect::<Vec<_>>();

    results.concat()
}

fn blink_n_times(input_row: &Vec<String>, n: u64) -> Vec<String> {
    let mut result = input_row.clone();
    for ii in 0..n {
        // Parallelize the blinking
        result = parallel_blink(&result, 3000, 5);
        println!("Done {}", ii);
    }
    result
}

// fn day_11(input_data: &Vec<String>) -> (i64, i64) {
fn day_11(input_data: &String) -> (i64, i64) {
    let mut result_1 = 0;
    let mut result_2 = 0;

    let first_row = input_data
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let blink_25 = blink_n_times(&first_row, 25);

    result_1 = blink_25.len() as i64;

    let blink_75 = blink_n_times(&blink_25, 50);

    result_2 = blink_75.len() as i64;

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day11() {
        let result_1 = 55312;
        let result_2 = 0;
        let input = "125 17".to_string();
        let (output_1, output_2) = day_11(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day11(b: &mut Bencher) {
        let filename = "data/day11.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        // let input = read_input_to_vector(filename);
        b.iter(|| {
            day_11(&input);
        });
    }
}
