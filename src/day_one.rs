use std::fs;

pub fn part_one() -> isize {

    // get the lists from the txt file
    let (mut list_one, mut list_two) = process_lists("inputs/day_one_input.txt");

    // sort the lists
    list_one.sort();
    list_two.sort();

    // re-zip the lists and sum up the differences
    let total: isize = list_one
        .iter()
        .zip(list_two.iter())
        .map(|(x, y)| {
            (x - y).abs()
        })
        .sum();

    println!("Total difference: {total}");
    total
}

fn process_lists(file: &str) -> (Vec<isize>, Vec<isize>) {
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");

    contents
        .split_whitespace()
        // parse into integers
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
        // collect the integers into tuples of two
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        // unzip into the two lists
        .unzip()
}

pub fn part_two() -> isize {

    // get the lists from the txt file
    let (list_one, list_two) = process_lists("inputs/day_one_input.txt");

    let total = list_one
        .iter()
        .map(|x| {
            let count = count_values(*x, &list_two);
            x * count as isize
        })
        .sum();

    println!("Similarity score: {total}");
    total
}

fn count_values(value: isize, list: &Vec<isize>) -> usize {
    list.iter().filter(|x| **x == value).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 1660292isize);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 22776016isize);
    }
}
