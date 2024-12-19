use rayon::prelude::*;
use std::collections::HashSet;

use crate::my_io::read_input_to_vector;
use crate::shared_objects::{Direction, Position, SparseGrid};

pub fn solve(filename: &String) -> (i64, i64) {
    let input = read_input_to_vector(filename);
    day_06(&input)
}

fn guard_walk(sparse_grid: &SparseGrid, initial_position: &Position) -> Option<i64> {
    let mut position = *initial_position;
    let mut direction = Direction::Up;
    let mut visited_positions_and_directions: HashSet<(Position, Direction)> = HashSet::new();

    loop {
        // Store the current position and direction as visited

        let mut next_position = position;
        while sparse_grid.is_free_tile(&next_position) {
            // If we have been here before, we are stuck in a loop
            if visited_positions_and_directions.contains(&(next_position, direction)) {
                return None;
            }
            visited_positions_and_directions.insert((next_position, direction));
            position = next_position;
            next_position = next_position + direction;
        }

        if !sparse_grid.is_inside(&next_position) {
            break;
        }

        direction = direction.next();
    }

    Some(
        visited_positions_and_directions
            .iter()
            .map(|(p, _)| p)
            .collect::<HashSet<_>>()
            .len() as i64,
    )
}

fn day_06(input_data: &Vec<String>) -> (i64, i64) {
    let result_1;
    let result_2;

    let data: Vec<Vec<char>> = input_data.iter().map(|s| s.chars().collect()).collect();

    let sparse_grid = input_to_sparse_grid(&data);
    let initial_position = find_start(&data);

    result_1 = guard_walk(&sparse_grid, &initial_position).unwrap();

    let empty_tiles = get_empty_tiles(&data);

    result_2 = empty_tiles
        .par_iter()
        .map(|new_obstacle| {
            let mut new_sparse_grid = sparse_grid.clone();
            new_sparse_grid.set(*new_obstacle, '#');
            // println!("New obstacle at {:?}", new_obstacle);
            guard_walk(&new_sparse_grid, &initial_position)
        })
        .filter(|x| x.is_none())
        .count() as i64;

    (result_1, result_2)
}

fn input_to_sparse_grid(data: &Vec<Vec<char>>) -> SparseGrid {
    let max_row = data.len() as i32;
    let max_col = data[0].len() as i32;
    let mut sparse_grid = SparseGrid::new(max_row, max_col);
    for y in 0..max_row {
        for x in 0..max_col {
            let tile = data[y as usize][x as usize] as char;
            if tile == '#' {
                sparse_grid.set(Position { x, y }, tile);
            }
        }
    }
    sparse_grid
}

fn find_start(data: &Vec<Vec<char>>) -> Position {
    let max_row = data.len();
    let max_col = data[0].len();
    for y in 0..max_row {
        for x in 0..max_col {
            if data[y][x] == '^' {
                return Position {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }
    panic!("No start found!");
}

fn get_empty_tiles(data: &Vec<Vec<char>>) -> HashSet<Position> {
    let mut empty_tiles = HashSet::new();
    let max_row = data.len();
    let max_col = data[0].len();
    for y in 0..max_row as usize {
        for x in 0..max_col as usize {
            if data[y][x] == '.' {
                empty_tiles.insert(Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    empty_tiles
}

fn visualize_sparse_grid(sparse_grid: &SparseGrid) -> String {
    let mut grid = vec![vec!['.'; sparse_grid.max_col as usize]; sparse_grid.max_row as usize];
    for y in 0..sparse_grid.max_row as usize {
        for x in 0..sparse_grid.max_col as usize {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            if sparse_grid.entries.contains_key(&pos) {
                grid[y][x] = '#';
            } else {
                grid[y][x] = '.';
            }
        }
    }
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day06() {
        let result_1 = 41;
        let result_2 = 6;
        let input = vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];
        let (output_1, output_2) = day_06(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day06(b: &mut Bencher) {
        let filename = "data/day06.txt";
        let input = read_input_to_vector(filename);
        b.iter(|| {
            day_06(&input);
        });
    }
}
