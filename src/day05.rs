use std::collections::HashMap;

use crate::my_io::read_input_to_vector;
use rayon::prelude::*;

pub fn solve(filename: &String) -> (i64, i64) {
    let input = read_input_to_vector(filename);
    day_05(&input)
}

fn day_05(input_data: &Vec<String>) -> (i64, i64) {
    let result_1;
    let result_2;

    // Split input into the rules and the print orders
    let rules = input_data
        .iter()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            s.split("|")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    // We list all the pages which must follow a certain page as a hashmap of vectors
    let mut rules_map: HashMap<i64, Vec<i64>> = HashMap::new();
    rules.iter().for_each(|r| {
        rules_map.entry(r[0]).or_insert_with(Vec::new).push(r[1]);
    });

    let print_orders = input_data
        .iter()
        .skip(rules.len() + 1)
        .map(|s| {
            s.split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut correct_print_orders = vec![];
    let mut incorrect_print_orders = vec![];

    // We don't parallelize here because the check is not that expensive
    // and the shared vectors cannot be borrowed as mutuable in the parallel loop
    // because writing to them would not be thread safe
    print_orders.iter().for_each(|po| {
        if is_valid_print_order(po, &rules_map) {
            correct_print_orders.push(po);
        } else {
            incorrect_print_orders.push(po);
        }
    });

    // This is way to cheap to parallelize
    result_1 = correct_print_orders
        .iter()
        .map(|po| po[(po.len() - 1) / 2])
        .sum();

    // The fixing is quite expensive, as it goes through the print orders multiple times
    // so this is the perfect candidate for parallelization
    result_2 = incorrect_print_orders
        .par_iter()
        .map(|po| fix_print_order(po, &rules_map))
        .map(|po| po[(po.len() - 1) / 2])
        .sum();

    (result_1, result_2)
}

fn is_valid_print_order(print_order: &Vec<i64>, rules_map: &HashMap<i64, Vec<i64>>) -> bool {
    let mut valid = true;
    'outer: for i in 0..print_order.len() {
        if let Some(must_update_after) = rules_map.get(&print_order[i]) {
            for j in i + 1..print_order.len() {
                // all following numbers must be in the rules list
                if !must_update_after.contains(&print_order[j]) {
                    valid = false;
                    break 'outer;
                }
            }
            for j in 0..i {
                // all previous numbers must not be in the rules list
                if must_update_after.contains(&print_order[j]) {
                    valid = false;
                    break 'outer;
                }
            }
        }
    }

    valid
}

fn fix_print_order(print_order: &Vec<i64>, rules_map: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    let mut fixed_print_order = print_order.clone();
    'fixing: loop {
        for i in 0..fixed_print_order.len() {
            if let Some(must_update_after) = rules_map.get(&fixed_print_order[i]) {
                for j in 0..i {
                    // if the order is not correct, we swap the elements until it is :-)
                    if must_update_after.contains(&fixed_print_order[j]) {
                        fixed_print_order.swap(i, j);
                        continue 'fixing;
                    }
                }
            }
        }
        break;
    }

    fixed_print_order
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_io::read_input_to_vector;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day05() {
        let result_1 = 143;
        let result_2 = 123;
        let input = vec![
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
            "".to_string(),
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ];
        let (output_1, output_2) = day_05(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day05(b: &mut Bencher) {
        let filename = "data/day05.txt";
        let input = read_input_to_vector(filename);
        b.iter(|| {
            day_05(&input);
        });
    }
}
