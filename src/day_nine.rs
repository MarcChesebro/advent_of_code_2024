use std::fs;
use crate::day_nine::FileData::Free;

pub fn part_one() -> isize {
    let contents = fs::read_to_string("inputs/day_nine_input.txt")
        .expect("Should have been able to read the file");

    let mut is_data_block = true;
    let mut filesystem: Vec<Option<isize>> = Vec::new();
    let mut current_id = 0isize;

    for c in contents.chars() {
        match String::from(c).parse::<isize>() {
            Ok(block_size) => {
                if is_data_block {
                    for _ in 0..block_size {
                        filesystem.push(Some(current_id));
                    }
                    current_id += 1;
                } else {
                    for _ in 0..block_size {
                        filesystem.push(None);
                    }
                }
                is_data_block = !is_data_block;
            },
            Err(_) => break,
        }
    }

    let mut placement_index = 0;
    for i in (0..filesystem.len()).rev() {
        if placement_index >= i {
            break;
        }
        let value = filesystem[i];

        match value {
            Some(_) => {
                while placement_index < i {
                    match filesystem[placement_index] {
                        Some(_) => placement_index += 1,
                        None => {
                            filesystem[placement_index] = value;
                            filesystem[i] = None;
                            placement_index += 1;
                            break;
                        }
                    }
                }
            }
            None => continue,
        }
    }

    calculate_checksum(&filesystem)
}

fn calculate_checksum(filesystem: &Vec<Option<isize>>) -> isize {
    filesystem.iter()
        .enumerate()
        .map(|(position, data)| {
            match data {
                Some(id) => id * position as isize,
                None => 0,
            }
        })
        .sum()
}

#[allow(dead_code)]
fn print_filesystem(filesystem: &Vec<Option<isize>>) {
    let file_string = filesystem.iter()
        .map(|data| {
            match data {
                Some(id) => {
                    char::from_digit(*id as u32, 10).unwrap()
                },
                None => '.',
            }
        })
        .collect::<String>();

    println!("{file_string}");
}

#[allow(dead_code)]
fn print_file_blocks(file_blocks: &Vec<FileData>) {
    print_filesystem(&file_blocks_to_filesystem(file_blocks));
}

#[derive(Clone, Copy)]
enum FileData {
    Data((isize, isize)),
    Free(isize),
}

pub fn part_two() -> isize {
    let contents = fs::read_to_string("inputs/day_nine_input.txt")
        .expect("Should have been able to read the file");

    let mut is_data_block = true;
    let mut filesystem: Vec<FileData> = Vec::new();
    let mut current_id = 0isize;

    for c in contents.chars() {
        match String::from(c).parse::<isize>() {
            Ok(block_size) => {
                if is_data_block {
                    filesystem.push(FileData::Data((current_id, block_size)));
                    current_id += 1;
                } else {
                    filesystem.push(FileData::Free(block_size));
                }
                is_data_block = !is_data_block;
            },
            Err(_) => break,
        }
    }

    for movement_index in (0..filesystem.len()).rev() {
        let file_data = filesystem[movement_index];

        match file_data {
            FileData::Data((_, block_size)) => {
                for placement_index in 0..filesystem.len() {
                    if placement_index > movement_index {
                        break;
                    }
                    let file_block = filesystem[placement_index];

                    match file_block {
                        Free(free_block_size) => {
                            if block_size <= free_block_size {
                                filesystem[placement_index] = file_data;
                                filesystem[movement_index] = Free(block_size);
                                filesystem.insert(placement_index + 1, Free(free_block_size - block_size));
                                break;
                            }
                        }
                        FileData::Data(_) => {}
                    }
                }
            }
            FileData::Free(_) => {}
        }
    }

    calculate_checksum(&file_blocks_to_filesystem(&filesystem))
}

fn file_blocks_to_filesystem(blocks: &Vec<FileData>) -> Vec<Option<isize>> {
    blocks.iter()
        .map(|file_data| {
            let mut block_values = Vec::new();

            match file_data {
                FileData::Data((id, block_size)) => {
                    for _ in 0..*block_size {
                        block_values.push(Some(*id));
                    }
                }
                FileData::Free(block_size) => {
                    for _ in 0..*block_size {
                        block_values.push(None);
                    }
                }
            }

            block_values
        })
        .flatten()
        .collect::<Vec<Option<isize>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 6_332_189_866_718);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 6_353_648_390_778);
    }
}