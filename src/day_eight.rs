use std::collections::HashMap;
use std::fs;
use std::ops::{Add, Sub};
use itertools::Itertools;

pub fn part_one() -> isize {
    let antenna_map = build_antenna_map("inputs/day_eight_input.txt");

    // take every antenna group and count the number of in bounds antinodes
    antenna_map.antenna_groups.iter()
        .map(|(_, positions)| {
            // for every group calculate all the position pairs
            positions
                .iter()
                .map(|p1| {
                    positions.iter().map(|p2| (p1, p2)).collect::<Vec<(&Position, &Position)>>()
                })
                // flatten them into a list of pairs
                .flatten()
                // filter out pairs of the same point
                .filter(|(p1, p2)| p1 != p2)
                // find the antinode by taking the difference of the 2 points
                .map(|(p1, p2)| {
                    let difference = p2 - p1;
                    p1 - &difference
                })
                // filter out of bounds points and count what is left
                .filter(|antinode| antenna_map.is_in_bounds(antinode))
                .collect::<Vec<Position>>()
        })
        .flatten()
        .unique()
        .count() as isize
}

pub fn part_two() -> isize {
    let antenna_map = build_antenna_map("inputs/day_eight_input.txt");

    // take every antenna group and count the number of in bounds antinodes
    antenna_map.antenna_groups.iter()
        .map(|(_, positions)| {
            // for every group calculate all the position pairs
            positions
                .iter()
                .map(|p1| {
                    positions.iter().map(|p2| (p1, p2)).collect::<Vec<(&Position, &Position)>>()
                })
                // flatten them into a list of pairs
                .flatten()
                // filter out pairs of the same point
                .filter(|(p1, p2)| p1 != p2)
                // find the antinode by taking the difference of the 2 points and also calculating
                // all harmonics within the map
                .map(|(p1, p2)| {
                    let difference = p2 - p1;
                    let mut new_point = p1 - &difference;
                    let mut antinodes = Vec::<Position>::new();
                    antinodes.push(*p1);

                    while antenna_map.is_in_bounds(&new_point) {
                        antinodes.push(new_point);
                        new_point = &new_point - &difference;
                    }
                    antinodes
                })
                .flatten()
                // filter out of bounds points and count what is left
                .filter(|antinode| antenna_map.is_in_bounds(antinode))
                .collect::<Vec<Position>>()
        })
        .flatten()
        .unique()
        .count() as isize
}

fn build_antenna_map(file: &str) -> AntennaMap {
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");

    AntennaMap::new(&contents)
}

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position {
            x,
            y,
        }
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: &Position) -> Self::Output {
        Position::new(
            self.x - rhs.x,
            self.y - rhs.y
        )
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Self::Output {
        Position::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

#[derive(Debug)]
struct AntennaMap {
    antenna_groups: HashMap<char, Vec<Position>>,
    height: isize,
    width: isize,
}

impl AntennaMap {
    fn new(map_string: &str) -> AntennaMap {
        let height = map_string.chars().filter(|c| *c == '\n').count() as isize + 1;
        let width = map_string.find('\n').unwrap() as isize;
        let map_rows = map_string.split('\n');
        let mut antenna_groups: HashMap<char, Vec<Position>> = HashMap::new();

        let mut y = 0;
        for row in map_rows {
            for (x, char) in row.chars().enumerate() {
                if char != '.' {
                    if !antenna_groups.keys().contains(&char) {
                        antenna_groups.insert(char, vec![Position::new(x as isize, y)]);
                    } else {
                        antenna_groups.get_mut(&char).unwrap().push(Position::new(x as isize, y));
                    }
                }
            }
            y += 1;
        }

        AntennaMap {
            antenna_groups,
            height,
            width,
        }
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        position.x >= 0 && position.y >= 0 && position.x < self.width && position.y < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 285);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 944);
    }
}