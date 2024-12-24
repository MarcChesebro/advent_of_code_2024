use std::collections::VecDeque;
use std::fs;

const DIRECTION_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0, -1),
    ( 0,  1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1),
];

#[derive(Debug)]
struct CharMatrix {
    chars: Vec<char>,
    pub width: isize,
    pub height: isize,
}

impl CharMatrix {
    fn new(input: &str) -> CharMatrix {
        CharMatrix {
            chars: process_input(input),
            width: input.find('\n').unwrap() as isize,
            height: (input.chars().filter(|c| *c == '\n').count() + 1) as isize,
        }
    }

    fn char_location_list(&self) -> VecDeque<((isize, isize), &char)> {
        let mut list = VecDeque::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let c = self.get(x, y).unwrap();

                list.push_back(((x, y), c));
            }
        }

        list
    }

    fn iter(&self) -> CharMatrixIterator {
        CharMatrixIterator::new(self)
    }

    fn get(&self, x: isize, y: isize) -> Option<&char> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None
        }
        let index = x + y * self.width;
        self.chars.get(index as usize)
    }

    fn is_char_at(&self, c: &char, x: isize, y: isize) -> bool {
        match self.get(x, y) {
            Some(x) => c == x,
            _ => false,
        }
    }

    fn count_from_pattern(&self, patterns: &Vec<Vec<((isize, isize), char)>>, target_char: char)  -> isize {
        self.iter()
            .filter(|((_x, _y), c)| {
                **c == target_char
            })
            .map(|((x, y), _c)| {
                patterns.iter()
                    .map(|offset_list: &Vec<((isize, isize), char)>| {
                        offset_list.iter()
                            .all(|((x_offset, y_offset), c)| {
                                self.is_char_at(&c, x + x_offset, y + y_offset)
                            })
                    })
                    .filter(|b| *b )
                    .count() as isize
            })
            .sum()
    }
}

struct CharMatrixIterator<'a>  {
    char_location_list: VecDeque<((isize, isize), &'a char)>,
}

impl<'a> CharMatrixIterator<'a> {
    fn new(char_matrix: &CharMatrix) -> CharMatrixIterator {
        CharMatrixIterator {
            char_location_list: char_matrix.char_location_list()
        }
    }
}

impl<'a> Iterator for CharMatrixIterator<'a> {
    type Item = ((isize, isize), &'a char);

    fn next(&mut self) -> Option<Self::Item> {
        self.char_location_list.pop_front()
    }
}

fn process_input(input: &str) -> Vec<char> {
    input
        // filter out whitespace and collect into Vec<char>
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>()
}

pub fn part_one() -> isize {
    let contents = fs::read_to_string("inputs/day_four_input.txt")
        .expect("Should have been able to read the file");

    let m = CharMatrix::new(&*contents);
    let word = "XMAS";

    let word_patterns= patterns_from_word(word);

    let first_letter = word.chars().next().unwrap();

    let word_count= m.count_from_pattern(&word_patterns, first_letter);

    println!("Word count: {word_count}");
    word_count
}

fn patterns_from_word(word: &str) -> Vec<Vec<((isize, isize), char)>> {
    let word_length = word.len();

    DIRECTION_OFFSETS.iter()
        .map(|(x, y)| {
            (0..word_length)
                .map(|n| n as isize)
                .map(|distance| {
                    (*x * distance, *y * distance)
                })
                .zip(word.chars())
                .collect()
        })
        .collect()
}

pub fn part_two() -> isize {
    let contents = fs::read_to_string("inputs/day_four_input.txt")
        .expect("Should have been able to read the file");

    let m = CharMatrix::new(&*contents);

    let patterns = vec![
        vec!(
            ((-1,  1), 'M'),
            ((-1, -1), 'M'),
            (( 1,  1), 'S'),
            (( 1, -1), 'S'),
            (( 0,  0), 'A'),
        ),
        vec!(
            ((-1,  1), 'M'),
            (( 1,  1), 'M'),
            ((-1, -1), 'S'),
            (( 1, -1), 'S'),
            (( 0,  0), 'A'),
        ),
        vec!(
            (( 1,  1), 'M'),
            (( 1,  -1), 'M'),
            ((-1,  1), 'S'),
            ((-1, -1), 'S'),
            (( 0,  0), 'A'),
        ),
        vec!(
            ((-1, -1), 'M'),
            (( 1, -1), 'M'),
            ((-1,  1), 'S'),
            (( 1,  1), 'S'),
            (( 0,  0), 'A'),
        ),
    ];

    let center_letter = 'A';

    let word_count= m.count_from_pattern(&patterns, center_letter);

    println!("Word count: {word_count}");
    word_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 2593);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 1950);
    }
}