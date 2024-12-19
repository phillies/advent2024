use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy)]
struct FileInfo {
    position: usize,
    length: usize,
}

#[derive(Debug, Clone)]
struct Disk {
    data: Vec<i32>,
    file_table: BTreeMap<usize, FileInfo>,
    empty_blocks: BTreeMap<usize, usize>,
}

impl Disk {
    fn move_file(&mut self, file_id: usize, new_position: usize) {
        let file_info = self.file_table.get(&file_id).unwrap();
        let length = file_info.length;
        let old_position = file_info.position;

        if !self.empty_blocks.contains_key(&new_position) {
            panic!(
                "Position {} is not the beginning of an empty block",
                new_position
            );
        }
        if self.empty_blocks[&new_position] < length {
            panic!(
                "Not enough space at position {} to move file {}",
                new_position, file_id
            );
        }

        for ii in 0..length {
            self.data.swap(old_position + ii, new_position + ii);
        }

        self.file_table.insert(
            file_id,
            FileInfo {
                position: new_position,
                length,
            },
        );

        if self.empty_blocks[&new_position] != length {
            self.empty_blocks.insert(
                new_position + length,
                self.empty_blocks[&new_position] - length,
            );
        }
        self.empty_blocks.remove(&new_position);
    }
    fn space_after(&self, file_id: usize) -> usize {
        let this_file = self
            .file_table
            .get(&file_id)
            .unwrap_or_else(|| panic!("File {} not found", file_id));

        *self
            .empty_blocks
            .get(&(this_file.position + this_file.length))
            .unwrap_or(&0)
    }
    fn print_disk(&self) -> String {
        self.data
            .iter()
            .map(|&x| {
                if x >= 0 {
                    x.to_string()
                } else {
                    ".".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("")
            + format!(" containing {} files", self.file_table.len()).as_str()
    }
}

pub fn solve(filename: &String) -> (i64, i64) {
    let input = std::fs::read_to_string(filename).expect("Could not read file!");
    day_09(&input)
}

fn calculate_checksum(disk: &Disk) -> i64 {
    disk.data
        .iter()
        .enumerate()
        .map(|(index, &entry)| {
            if entry >= 0 {
                index as i64 * entry as i64
            } else {
                0
            }
        })
        .sum()
}

fn unravel_disk(input_data: &String) -> Disk {
    let mut file_table = BTreeMap::new();
    let mut empty_blocks = BTreeMap::new();

    let data = input_data
        .chars()
        .enumerate()
        .map(|(index, entry)| {
            let value = entry.to_digit(10).unwrap() as usize;
            if index % 2 == 0 {
                let id = (index / 2) as i32;
                vec![id; value]
            } else {
                vec![-1; value]
            }
        })
        // here we use fold not reduce so the if statement catches the first entry as well
        // otherwise acc would already contain the file with ID 0
        .fold(Vec::new(), |mut acc, entry| {
            if entry.first() > Some(&-1) {
                file_table.insert(
                    (*entry.first().unwrap()) as usize,
                    FileInfo {
                        position: acc.len(),
                        length: entry.len(),
                    },
                );
            } else if entry.first() == Some(&-1) {
                empty_blocks.insert(acc.len(), entry.len());
            }
            entry.iter().for_each(|v| acc.push(*v));
            acc
        });

    Disk {
        data,
        file_table,
        empty_blocks,
    }
}

fn reorder_blocks(input_data: &String) -> Disk {
    let mut disk = unravel_disk(input_data);
    let data = &mut disk.data;
    let mut back_pointer = data.len() - 1;
    let mut front_pointer = 0;

    'outer: loop {
        while data.get(front_pointer) != Some(&-1) {
            front_pointer += 1;
            if front_pointer >= back_pointer {
                break 'outer;
            }
        }
        while data.get(front_pointer) == Some(&-1) {
            data[front_pointer] = *data.get(back_pointer).unwrap();
            data[back_pointer] = -1;
            back_pointer -= 1;
            while data.get(back_pointer) == Some(&-1) {
                back_pointer -= 1;
            }
            if back_pointer <= front_pointer {
                break 'outer;
            }
        }
    }
    disk
}

fn reorder_files(input_data: &String) -> Disk {
    let mut disk = unravel_disk(input_data);

    for file_id in (0..disk.file_table.len()).rev() {
        let file_info = disk.file_table.get(&file_id).unwrap();
        let length = file_info.length;
        let mut front_pointer = 0;
        loop {
            while front_pointer < disk.data.len()
                && disk.data[front_pointer] == -1
                && disk.data[front_pointer] != file_id as i32
            {
                front_pointer += 1;
            }
            if front_pointer >= disk.data.len() || disk.data[front_pointer] == file_id as i32 {
                break;
            }

            let file_id_at_pointer = disk.data[front_pointer] as usize;
            let file_at_pointer = disk.file_table.get(&file_id_at_pointer).unwrap();

            if disk.space_after(file_id_at_pointer) >= length {
                disk.move_file(file_id, file_at_pointer.position + file_at_pointer.length);
                break;
            } else {
                front_pointer = file_at_pointer.position + file_at_pointer.length;
            }
        }
    }

    disk
}

fn day_09(input_data: &String) -> (i64, i64) {
    let disk = reorder_blocks(input_data);

    let checksum_1 = calculate_checksum(&disk);

    let disk = reorder_files(input_data);

    let checksum_2 = calculate_checksum(&disk);

    (checksum_1, checksum_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_day09() {
        let result_1 = 1928;
        let result_2 = 2858;
        let input = "2333133121414131402".to_string();
        let (output_1, output_2) = day_09(&input);
        assert_eq!(result_1, output_1);
        assert_eq!(result_2, output_2);
    }

    #[bench]
    fn bench_day09(b: &mut Bencher) {
        let filename = "data/day09.txt";
        let input = std::fs::read_to_string(filename).expect("Could not read file!");
        b.iter(|| {
            day_09(&input);
        });
    }
}
