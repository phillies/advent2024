use std::collections::HashSet;

use crate::{
    my_io::read_input_to_vector,
    shared_objects::{Position, SparseGrid},
};
use prime_factorization::Factorization;

pub fn solve(filename: &String) -> (i64, i64) {
    let input = read_input_to_vector(filename);
    day_08(&input)
}

fn get_antinodes_with_2x_distance(
    sparse_grid: &SparseGrid,
    antenna_1: &Position,
    antenna_2: &Position,
) -> Vec<Position> {
    let distance_vector = *antenna_1 - *antenna_2;
    let mut antinodes = vec![];

    let antinode = *antenna_1 + distance_vector;
    if sparse_grid.is_inside(&antinode) {
        antinodes.push(antinode);
    }
    let antinode = *antenna_2 - distance_vector;
    if sparse_grid.is_inside(&antinode) {
        antinodes.push(antinode);
    }
    antinodes
}

/// Calculates the shortest distance vector that wil always end on a grid entry by removing the
/// common factors e.g. for a distance vector of (2, -4) that would be (1, -2)
fn get_shortest_distance_vector_on_grid(distance_vector: &Position) -> Position {
    let x_factors = Factorization::<u32>::run(distance_vector.x.abs() as u32);
    let y_factors = Factorization::<u32>::run(distance_vector.y.abs() as u32);

    // We add 1 as factor so if x_factors and y_factors are identical and to keep
    // the sign of the distance vector
    let mut new_x_factors: Vec<i32> = vec![distance_vector.x.signum()];
    let mut new_y_factors: Vec<i32> = y_factors
        .factors
        .clone()
        .iter()
        .map(|x| *x as i32)
        .collect();
    new_y_factors.push(1 * distance_vector.y.signum());

    x_factors.factors.iter().for_each(|factor| {
        let factor = *factor as i32;
        if new_y_factors.contains(&factor) {
            new_y_factors.remove(new_y_factors.iter().position(|x| *x == factor).unwrap());
        } else {
            new_x_factors.push(factor);
        }
    });

    Position {
        x: new_x_factors.iter().product(),
        y: new_y_factors.iter().product(),
    }
}

fn get_antinodes_with_1x_distance(
    sparse_grid: &SparseGrid,
    antenna_1: &Position,
    antenna_2: &Position,
) -> Vec<Position> {
    let new_distance = get_shortest_distance_vector_on_grid(&(*antenna_1 - *antenna_2));
    let mut antinodes = vec![];

    // There could be an antinode between antenna_1 and antenna_2, so we
    // start from antenna_2 and add the new distance until we reach the
    // border of the grid. But we need to skip the antinode at antenna_1
    let mut antinode = *antenna_2 + new_distance;
    while sparse_grid.is_inside(&antinode) {
        if antinode != *antenna_1 {
            antinodes.push(antinode);
            // println!("Antinode+: {:?}", antinode);
        }
        antinode = antinode + new_distance;
    }

    antinode = *antenna_2 - new_distance;
    while sparse_grid.is_inside(&antinode) {
        antinodes.push(antinode);
        // println!("Antinode-: {:?}", antinode);
        antinode = antinode - new_distance;
    }

    antinodes
}

fn get_2x_antinodes(sparse_grid: &SparseGrid, unique_entries: &Vec<char>) -> i64 {
    // we do not want to count the same antinode twice
    let mut antinodes = HashSet::new();
    unique_entries.iter().for_each(|entry| {
        let positions = sparse_grid.get_positions(*entry);
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let local_antinodes =
                    get_antinodes_with_2x_distance(&sparse_grid, &positions[i], &positions[j]);
                antinodes.extend(local_antinodes);
            }
        }
    });

    antinodes.len() as i64
}

fn get_1x_antinodes(sparse_grid: &SparseGrid, unique_entries: &Vec<char>) -> i64 {
    // we do not want to count the same antinode twice
    let mut antinodes = HashSet::new();
    unique_entries.iter().for_each(|entry| {
        let positions = sparse_grid.get_positions(*entry);
        // Each antenna is an antinode
        antinodes.extend(positions.clone());
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let local_antinodes =
                    get_antinodes_with_1x_distance(&sparse_grid, &positions[i], &positions[j]);
                antinodes.extend(local_antinodes);
            }
        }
    });

    antinodes.len() as i64
}

fn day_08(input_data: &Vec<String>) -> (i64, i64) {
    let result_1;
    let result_2;

    let input_chars = input_data
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let sparse_grid = SparseGrid::from_input_data(&input_chars);

    // Different antenna types
    let unique_entries = sparse_grid.get_unique_entries();

    result_1 = get_2x_antinodes(&sparse_grid, &unique_entries);

    result_2 = get_1x_antinodes(&sparse_grid, &unique_entries);

    (result_1, result_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day08() {
        let result_1 = 14;
        let result_2 = 34;
        let input = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];
        let (output_1, output_2) = day_08(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day08(b: &mut Bencher) {
        let filename = "data/day08.txt";
        let input = read_input_to_vector(filename);
        b.iter(|| {
            day_08(&input);
        });
    }
}
