#![feature(array_chunks)]
#![feature(test)]
mod my_io;
mod shared_objects;
use std::{collections::HashMap, env, fs};

use once_cell::sync::Lazy;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

static DAY_MAP: Lazy<HashMap<&'static str, for<'a> fn(&'a String) -> (i64, i64)>> =
    Lazy::new(|| {
        let mut m: HashMap<&'static str, for<'a> fn(&'a String) -> (i64, i64)> = HashMap::new();
        m.insert("day01", day01::solve);
        m.insert("day02", day02::solve);
        m.insert("day03", day03::solve);
        m.insert("day04", day04::solve);
        m.insert("day05", day05::solve);
        m.insert("day06", day06::solve);
        m.insert("day07", day07::solve);
        m.insert("day08", day08::solve);
        m.insert("day09", day09::solve);
        m.insert("day10", day10::solve);
        m.insert("day11", day11::solve);
        // m.insert("day12", day12::solve);
        // m.insert("day13", day13::solve);
        // m.insert("day14", day14::solve);
        // m.insert("day15", day15::solve);
        // m.insert("day16", day16::solve);
        // m.insert("day17", day17::solve);
        // m.insert("day18", day18::solve);
        // m.insert("day19", day19::solve);
        // m.insert("day20", day20::solve);
        // m.insert("day21", day21::solve);
        // m.insert("day22", day22::solve);
        // m.insert("day23", day23::solve);
        // m.insert("day24", day24::solve);
        // m.insert("day25", day25::solve);
        m
    });

fn run(day: &String, input: &String) {
    let part_1;
    let part_2;

    if let Some(day_function) = DAY_MAP.get(day.as_str()) {
        (part_1, part_2) = day_function(input);
    } else {
        println!("Day {} not implemented yet", day);
        return;
    }
    println!(
        "{}: Part 1: {}, Part 2: {}",
        day.to_uppercase(),
        part_1,
        part_2
    );
}
fn main() {
    let args = env::args().collect::<Vec<String>>();
    // run all days
    if args.len() < 2 {
        let directory_path = "data";
        let mut data_files = vec![];

        // Read the directory contents
        if let Ok(entries) = fs::read_dir(directory_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Get the file name
                    let file_name = entry.file_name();
                    data_files.push(file_name);
                }
            }
        } else {
            println!("Failed to read directory");
        }

        for file in data_files {
            let file_name = file.to_str().unwrap();
            let day = file_name.split(".").collect::<Vec<&str>>()[0];
            let input = format!("{}/{}", directory_path, file_name);
            run(&day.to_string(), &input);
        }
    // run a specific day
    } else if args.len() == 2 {
        let day = args.get(1).expect("Please specify day as first argument!");
        let input = format!("data/{}.txt", day);

        run(&day, &input);
    } else if args.len() >= 3 {
        let day = args.get(1).expect("Please specify day as first argument!");
        let input = args[2..].join(" ");

        run(&day, &input);
    }
}
