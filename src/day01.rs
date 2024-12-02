use std::collections::HashMap;

pub fn solve(filename: &String) -> (i64, i64) {
    let input_data = std::fs::read_to_string(filename).expect("Could not read file!");
    day_01_preallocate(&input_data)
}

// Parse the input line by line and have the vectors grow with each line
// Should be faster for small inputs
pub fn parse_input_vector_increase(input_data: &String) -> (Vec<i64>, Vec<i64>, HashMap<i64, i64>) {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    let mut occurrences: HashMap<i64, i64> = HashMap::new();

    input_data.lines().for_each(|line| {
        let mut split = line.split("   ");
        let first = split.next().unwrap().parse::<i64>().unwrap();
        let second = split.next().unwrap().parse::<i64>().unwrap();
        list_1.push(first);
        list_2.push(second);
        *occurrences.entry(second).or_insert(0) += 1;
    });
    (list_1, list_2, occurrences)
}

// Parse the input first and preallocate the vectors
// Should be faster for larger inputs
pub fn parse_input_preallocate(input_data: &String) -> (Vec<i64>, Vec<i64>, HashMap<i64, i64>) {
    let lines: Vec<&str> = input_data.lines().collect();
    let mut list_1 = vec![0; lines.len()];
    let mut list_2 = vec![0; lines.len()];
    let mut occurrences: HashMap<i64, i64> = HashMap::new();

    lines.iter().enumerate().for_each(|(index, line)| {
        let mut split = line.split("   ");
        let first = split.next().unwrap().parse::<i64>().unwrap();
        let second = split.next().unwrap().parse::<i64>().unwrap();
        list_1[index] = first;
        list_2[index] = second;
        *occurrences.entry(second).or_insert(0) += 1;
    });
    list_1.sort();
    list_2.sort();
    (list_1, list_2, occurrences)
}

fn calculate_results(
    list_1: &Vec<i64>,
    list_2: &Vec<i64>,
    occurrences: &HashMap<i64, i64>,
) -> (i64, i64) {
    let mut sum_dist = 0;

    for i in 0..list_1.len() {
        sum_dist += (list_1[i] - list_2[i]).abs();
    }

    let mut similarity_score = 0;

    list_1.iter().for_each(|&x| {
        if let Some(&count) = occurrences.get(&x) {
            similarity_score += x * count;
        }
    });

    (sum_dist, similarity_score)
}

// Variant which increases the vector size with each read line
fn day_01_grow(input_data: &String) -> (i64, i64) {
    let (mut list_1, mut list_2, occurrences) = parse_input_vector_increase(input_data);

    list_1.sort();
    list_2.sort();

    let (sum_dist, similarity_score) = calculate_results(&list_1, &list_2, &occurrences);

    (sum_dist, similarity_score)
}

// Read the input first and preallocate the vectors
fn day_01_preallocate(input_data: &String) -> (i64, i64) {
    let (mut list_1, mut list_2, occurrences) = parse_input_preallocate(input_data);

    list_1.sort();
    list_2.sort();

    let (sum_dist, similarity_score) = calculate_results(&list_1, &list_2, &occurrences);

    (sum_dist, similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day01() {
        let result_1 = 11;
        let result_2 = 31;
        let input = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ]
        .join("\n");
        let (output_1, output_2) = day_01_grow(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day01_grow(b: &mut Bencher) {
        let input = "data/day01.txt".to_string();
        let input_data = std::fs::read_to_string(&input).expect("Could not read file!");
        b.iter(|| {
            day_01_grow(&input_data);
        });
    }

    #[bench]
    fn bench_day01_preallocate(b: &mut Bencher) {
        let input = "data/day01.txt".to_string();
        let input_data = std::fs::read_to_string(&input).expect("Could not read file!");
        b.iter(|| {
            day_01_preallocate(&input_data);
        });
    }
}
