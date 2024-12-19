/// That's my template for new days. Includes the solve function signature and a test function
use crate::my_io::read_input_to_vector;

pub fn solve(filename: &String) -> (i64, i64) {
    let input = std::fs::read_to_string(filename).expect("Could not read file!");
    // let input = read_input_to_vector(filename);
    day_xx(&input)
}

// fn day_xx(input_data: &Vec<String>) -> (i64, i64) {
fn day_xx(input_data: &String) -> (i64, i64) {
    let mut result_1 = 0;
    let mut result_2 = 0;

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_dayxx() {
        let result_1 = 0;
        let result_2 = 0;
        let input = vec![
            "...".to_string(),
            "...".to_string(),
            "...".to_string(),
            "...".to_string(),
        ]
        .join("\n");
        let (output_1, output_2) = day_xx(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_dayxx(b: &mut Bencher) {
        let filename = "data/dayxx.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        // let input = read_input_to_vector(filename);
        b.iter(|| {
            day_xx(&input);
        });
    }
}
