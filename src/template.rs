use std::fs;

pub fn part_one() -> isize {
    let contents = fs::read_to_string("inputs/day_three_input.txt")
        .expect("Should have been able to read the file");

    todo!()
}

pub fn part_two() -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 1);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 2);
    }
}