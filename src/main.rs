#![feature(array_chunks)]
#![feature(test)]
mod my_io;
use std::{env, fs};

mod day01;
mod day02;
mod day03;

fn run(day: &String, input: &String) {
    let part_1;
    let part_2;
    match day.as_str() {
        "day01" => {
            (part_1, part_2) = day01::solve(input);
        }
        "day02" => {
            (part_1, part_2) = day02::solve(input);
        }
        "day03" => {
            (part_1, part_2) = day03::solve(input);
        }
        _ => {
            println!("No solution for this {} yet!", day);
            return;
        }
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
