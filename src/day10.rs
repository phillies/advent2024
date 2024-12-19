use std::{collections::HashSet, hash::Hash};

use crate::{
    my_io::read_input_to_vector,
    shared_objects::{Direction, Grid, Position},
};

fn ascend(grid: &Grid<u8>, visited_peaks: &mut HashSet<Position>, position: &Position) -> i64 {
    if visited_peaks.contains(position) {
        return 0;
    }

    let current_height = grid.get(position);
    if current_height.is_none() {
        0
    } else {
        let &current_height = current_height.unwrap();
        if current_height == 9 {
            visited_peaks.insert(*position);
            1
        } else {
            Direction::all()
                .iter()
                .map(|direction| {
                    let new_position = *position + *direction;
                    if grid.get(&new_position) == Some(&(current_height + 1)) {
                        ascend(grid, visited_peaks, &new_position)
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

fn ascend_unbound(grid: &Grid<u8>, position: &Position) -> i64 {
    let current_height = grid.get(position);
    if current_height.is_none() {
        0
    } else {
        let &current_height = current_height.unwrap();
        if current_height == 9 {
            1
        } else {
            Direction::all()
                .iter()
                .map(|direction| {
                    let new_position = *position + *direction;
                    if grid.get(&new_position) == Some(&(current_height + 1)) {
                        ascend_unbound(grid, &new_position)
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

fn make_grid(input_data: &Vec<String>) -> Grid<u8> {
    let mut grid = Grid::new(input_data[0].len(), input_data.len(), 0);
    for (y, line) in input_data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set_xy(x, y, c.to_digit(10).unwrap() as u8);
        }
    }
    grid
}

pub fn solve(filename: &String) -> (i64, i64) {
    let input = read_input_to_vector(filename);
    day_10(&input)
}

fn day_10(input_data: &Vec<String>) -> (i64, i64) {
    let mut result_1 = 0;
    let mut result_2 = 0;

    let grid = make_grid(input_data);

    let mut start_positions = HashSet::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let position = Position {
                x: x as i32,
                y: y as i32,
            };
            if grid.get(&position) == Some(&0) {
                start_positions.insert(position);
            }
        }
    }
    result_1 = start_positions
        .iter()
        .map(|pos| {
            let mut peaks: HashSet<Position> = HashSet::new();
            let result = ascend(&grid, &mut peaks, pos);
            result
        })
        .sum();

    result_2 = start_positions
        .iter()
        .map(|pos| ascend_unbound(&grid, pos))
        .sum();

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day10() {
        let result_1 = 36;
        let result_2 = 81;
        let input = vec![
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];
        let (output_1, output_2) = day_10(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day10(b: &mut Bencher) {
        let filename = "data/day10.txt";
        let input = read_input_to_vector(filename);
        b.iter(|| {
            day_10(&input);
        });
    }
}
