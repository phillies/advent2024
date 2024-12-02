/// That's my template for new days. Includes the solve function signature and a test function
pub fn solve(filename: &String) -> (i64, i64) {
    let input_data = std::fs::read_to_string(filename).expect("Could not read file!");
    day_02(&input_data)
}

fn day_02(input_data: &String) -> (i64, i64) {
    let minimum_difference = 1;
    let maximum_difference = 3;
    let mut number_safe_reports = 0;
    let mut number_safe_reports_with_dampener = 0;

    input_data.lines().for_each(|line| {
        let split: Vec<i64> = line
            .split(" ")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        let safe = is_safe(&split, minimum_difference, maximum_difference).0;
        if safe {
            number_safe_reports += 1;
        }

        let safe_with_dampener =
            is_safe_with_dampener(&split, minimum_difference, maximum_difference);
        if safe_with_dampener {
            number_safe_reports_with_dampener += 1;
        }
    });
    (number_safe_reports, number_safe_reports_with_dampener)
}

// Checks if the levels are constantly increasing or decreasing
// and returns a tuple of the result plus the points where the increasing
// or decreasing order is violated.
fn is_safe(
    levels: &Vec<i64>,
    minimum_difference: i64,
    maximum_difference: i64,
) -> (bool, usize, usize) {
    let mut break_index_increasing = 0;
    let mut break_index_decreasing = 0;

    let constantly_increasing = levels.windows(2).enumerate().all(|(index, w)| {
        let diff = w[1] - w[0];
        break_index_increasing = index;
        diff >= minimum_difference && diff <= maximum_difference
    });
    let constantly_decreasing = levels.windows(2).enumerate().all(|(index, w)| {
        let diff = w[0] - w[1];
        break_index_decreasing = index;
        diff >= minimum_difference && diff <= maximum_difference
    });
    (
        constantly_decreasing || constantly_increasing,
        break_index_increasing,
        break_index_decreasing,
    )
}

// The violation of the increasing or decreasing order can be fixed by
// removing the element before or after the break_index
fn remove_before_position(levels: &Vec<i64>, position: usize) -> Vec<i64> {
    levels[..position]
        .to_vec()
        .iter()
        .chain(levels[(position + 1)..].iter())
        .cloned()
        .collect()
}

fn remove_after_position(levels: &Vec<i64>, position: usize) -> Vec<i64> {
    levels[..(position + 1)]
        .to_vec()
        .iter()
        .chain(levels[(position + 2)..].iter())
        .cloned()
        .collect()
}

fn is_safe_with_dampener(
    levels: &Vec<i64>,
    minimum_difference: i64,
    maximum_difference: i64,
) -> bool {
    let (undampend_safe, unsafe_after_position_increasing, unsafe_after_position_decreasing) =
        is_safe(levels, minimum_difference, maximum_difference);

    if undampend_safe {
        return true;
    }

    // We check if we can fix the levels by removing the element before or after the violating
    // level jump for both increasing and decreasing order, as the direction might be misleading
    // when we remove the first or second element (a seemingly increasing order can become decreasing
    // if we e.g. remove the second element from 5 6 3 2 1)
    is_safe(
        &remove_before_position(levels, unsafe_after_position_increasing),
        minimum_difference,
        maximum_difference,
    )
    .0 || is_safe(
        &remove_after_position(levels, unsafe_after_position_increasing),
        minimum_difference,
        maximum_difference,
    )
    .0 || is_safe(
        &remove_before_position(levels, unsafe_after_position_decreasing),
        minimum_difference,
        maximum_difference,
    )
    .0 || is_safe(
        &remove_after_position(levels, unsafe_after_position_decreasing),
        minimum_difference,
        maximum_difference,
    )
    .0
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day02() {
        let result_1 = 2;
        let result_2 = 4;
        let input = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ]
        .join("\n");
        let (output_1, output_2) = day_02(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day02(b: &mut Bencher) {
        let filename = "data/day02.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        b.iter(|| {
            day_02(&input);
        });
    }
}
