use std::collections::HashMap;
use std::fs;
use rayon::prelude::*;

pub fn part_one() -> isize {
    let mut stones = parse_stones("inputs/day_eleven_input.txt");
    println!("starting state: {:?}", stones);

    for _ in 0..25 {
        stones = blink(&mut stones);
    }
    stones.iter().count() as isize
}

pub fn part_two() -> isize {
    let stones_raw = parse_stones("inputs/day_eleven_input.txt");

    let mut stones: HashMap<isize, isize> = stones_raw.iter().map(|stone| (*stone, 1isize)).collect();

    for _ in 0..75 {
        stones = blink_map(&mut stones);
    }
    stones.iter().map(|(_, count)| count).sum()
}

fn parse_stones(file: &str) -> Vec<isize> {
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");

    contents
        .split(' ')
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
}

fn blink(stones: &mut Vec<isize>) -> Vec<isize> {
    stones
        .par_iter()
        .map(|stone| {
            if *stone == 0 {
                vec![1isize]
            } else {
                let stone_string = stone.to_string();
                if stone_string.len() % 2 == 0 {
                    let midpoint = stone_string.len() / 2;
                    let first_half = stone_string.get(0..midpoint).unwrap()
                        .parse::<isize>().unwrap();
                    let second_half = stone_string.get(midpoint..).unwrap()
                        .parse::<isize>().unwrap();

                    vec![first_half, second_half]
                } else {
                    vec![stone * 2024]
                }
            }
        })
        .flatten()
        .collect::<Vec<isize>>()
}

fn blink_map(stones: &HashMap<isize, isize>) -> HashMap<isize, isize> {
    let new_values = stones
        .par_iter()
        .map(|(stone, number)| {
            if *stone == 0 {
                vec![(1isize, *number)]
            } else {
                let stone_string = stone.to_string();
                if stone_string.len() % 2 == 0 {
                    let midpoint = stone_string.len() / 2;
                    let first_half = stone_string.get(0..midpoint).unwrap()
                        .parse::<isize>().unwrap();
                    let second_half = stone_string.get(midpoint..).unwrap()
                        .parse::<isize>().unwrap();

                    vec![(first_half, *number), (second_half, *number)]
                } else {
                    vec![(stone * 2024, *number)]
                }
            }
        })
        .flatten()
        .collect::<Vec<(isize, isize)>>();


    let mut new_map: HashMap<isize, isize> = HashMap::new();
    for (key, value) in new_values.iter() {
        if new_map.contains_key(key) {
            *new_map.get_mut(key).unwrap() += value;
        } else {
            new_map.insert(*key, *value);
        }
    }

    new_map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 203_228);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 240_884_656_550_923);
    }
}