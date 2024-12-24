use std::fs;
use regex::Regex;

const MULTIPLY_REGEX: &str = r"mul\(([0-9]+),([0-9]+)\)";
const ALL_REGEX: &str = r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))";

pub fn part_one() -> isize {
    let contents = fs::read_to_string("inputs/day_three_input.txt")
        .expect("Should have been able to read the file");

    let re = Regex::new(MULTIPLY_REGEX).unwrap();

    let captures = re.captures_iter(&*contents).map(|x| x.extract());

    captures
        .map(|capture| {
            let (_, [one, two]) = capture;
            one.parse::<isize>().unwrap() * two.parse::<isize>().unwrap()
        })
        .sum()
}

pub fn part_two() -> isize {
    let contents = fs::read_to_string("inputs/day_three_input.txt")
        .expect("Should have been able to read the file");

    let all_regex = Regex::new(ALL_REGEX).unwrap();

    let captures = all_regex.captures_iter(&*contents).map(|x| x.extract());

    let mut enabled = true;
    let mut sum = 0isize;

    for c in captures {
        let (x, [_]) = c;
        match x {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    sum += process_multiply(x)
                }
            },
        }
    }
    sum
}

fn process_multiply(string: &str) -> isize {
    let multiply_regex = Regex::new(MULTIPLY_REGEX).unwrap();
    let captures= multiply_regex.captures(string).unwrap();

    let one = &captures[1].parse::<isize>().unwrap();
    let two = &captures[2].parse::<isize>().unwrap();

    one * two
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 164_730_528);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 70_478_672);
    }
}
