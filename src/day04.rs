use crate::my_io::read_input_to_vector;

pub fn solve(filename: &String) -> (i64, i64) {
    let input = read_input_to_vector(filename);
    day_04(&input)
}

fn day_04(input_data: &Vec<String>) -> (i64, i64) {
    let data: Vec<Vec<char>> = input_data.iter().map(|s| s.chars().collect()).collect();
    let mut xmas_count = 0;

    // Assumes that all rows are of the same length
    let row_count = data.len();
    let col_count = data[0].len();
    for row in 0..row_count {
        for col in 0..col_count {
            if data[row][col] == 'X' {
                xmas_count += find_xmas(&data, row, col);
            }
        }
    }

    let mut x_mas_count = 0;
    // As we need to have 1 char in any direction, we can skip the first and last row and column
    for row in 1..row_count - 1 {
        for col in 1..col_count - 1 {
            if data[row][col] == 'A' && find_x_mas(&data, row, col) {
                x_mas_count += 1;
            }
        }
    }

    (xmas_count, x_mas_count)
}

// Starting from an X, find the word "XMAS" in all directions
fn find_xmas(data: &Vec<Vec<char>>, row_pos: usize, col_pos: usize) -> i64 {
    let mut found = 0;
    let max_row = data.len();
    let max_col = data[0].len();

    // Search forward (we start on X so we can skip the first letter)
    if col_pos < max_col - 3 {
        if data[row_pos][col_pos + 1] == 'M'
            && data[row_pos][col_pos + 2] == 'A'
            && data[row_pos][col_pos + 3] == 'S'
        {
            found += 1;
        }
    }

    // Search backward
    if col_pos >= 3 {
        if data[row_pos][col_pos - 1] == 'M'
            && data[row_pos][col_pos - 2] == 'A'
            && data[row_pos][col_pos - 3] == 'S'
        {
            found += 1;
        }
    }

    // Search down
    if row_pos < max_row - 3 {
        if data[row_pos + 1][col_pos] == 'M'
            && data[row_pos + 2][col_pos] == 'A'
            && data[row_pos + 3][col_pos] == 'S'
        {
            found += 1;
        }
    }

    // Search up
    if row_pos >= 3 {
        if data[row_pos - 1][col_pos] == 'M'
            && data[row_pos - 2][col_pos] == 'A'
            && data[row_pos - 3][col_pos] == 'S'
        {
            found += 1;
        }
    }

    // Search diagonally down-right
    if row_pos < max_row - 3 && col_pos < max_col - 3 {
        if data[row_pos + 1][col_pos + 1] == 'M'
            && data[row_pos + 2][col_pos + 2] == 'A'
            && data[row_pos + 3][col_pos + 3] == 'S'
        {
            found += 1;
        }
    }

    // Search diagonally up-right
    if row_pos >= 3 && col_pos < max_col - 3 {
        if data[row_pos - 1][col_pos + 1] == 'M'
            && data[row_pos - 2][col_pos + 2] == 'A'
            && data[row_pos - 3][col_pos + 3] == 'S'
        {
            found += 1;
        }
    }

    // Search diagonally down-left
    if row_pos < max_row - 3 && col_pos >= 3 {
        if data[row_pos + 1][col_pos - 1] == 'M'
            && data[row_pos + 2][col_pos - 2] == 'A'
            && data[row_pos + 3][col_pos - 3] == 'S'
        {
            found += 1;
        }
    }

    // Search diagonally up-left
    if row_pos >= 3 && col_pos >= 3 {
        if data[row_pos - 1][col_pos - 1] == 'M'
            && data[row_pos - 2][col_pos - 2] == 'A'
            && data[row_pos - 3][col_pos - 3] == 'S'
        {
            found += 1;
        }
    }

    found
}

// Starting from an A, finding MAS on the diagonals
// As we always need to have 1 char in any direction, we can skip the check for
// boundaries and assume the caller does the check
fn find_x_mas(data: &Vec<Vec<char>>, row_pos: usize, col_pos: usize) -> bool {
    // Search down-right
    let down_diagonal_match = (data[row_pos - 1][col_pos - 1] == 'S'
        && data[row_pos + 1][col_pos + 1] == 'M')
        || (data[row_pos + 1][col_pos + 1] == 'S' && data[row_pos - 1][col_pos - 1] == 'M');
    // Search up-right
    let up_diagonal_match = (data[row_pos + 1][col_pos - 1] == 'S'
        && data[row_pos - 1][col_pos + 1] == 'M')
        || (data[row_pos - 1][col_pos + 1] == 'S' && data[row_pos + 1][col_pos - 1] == 'M');

    // Only a match if we have a match in both diagonals
    down_diagonal_match && up_diagonal_match
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day04() {
        let result_1 = 18;
        let result_2 = 9;
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];
        let (output_1, output_2) = day_04(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day04(b: &mut Bencher) {
        let filename = "data/day04.txt";
        let input = read_input_to_vector(filename);
        b.iter(|| {
            day_04(&input);
        });
    }
}
