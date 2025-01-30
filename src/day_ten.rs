use std::fs;
use std::ops::Add;
use itertools::Itertools;

pub fn part_one() -> isize {
    let terrain_map = TerrainMap::new("inputs/day_ten_input.txt");

    terrain_map
        .iter()
        // find the trailheads (tiles with a 0)
        .filter(|(_position, value)| *value == 0 )
        // calculate the score of each trailhead and sum them up
        .map(|(position, _value)| {
            terrain_map.score_trailhead(&position)
        })
        .sum()
}

pub fn part_two() -> isize {
    let terrain_map = TerrainMap::new("inputs/day_ten_input.txt");

    terrain_map
        .iter()
        // find the trailheads (tiles with a 0)
        .filter(|(_position, value)| *value == 0 )
        // calculate the rating of each trailhead and sum them up
        .map(|(position, _value)| {
            terrain_map.rate_trailhead(&position)
        })
        .sum()
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

#[allow(dead_code)]
struct TerrainMap {
    pub tiles: Vec<isize>,
    pub height: isize,
    pub width: isize
}

impl TerrainMap {
    fn new(file: &str) -> TerrainMap {
        let content = fs::read_to_string(file)
            .expect("Should have been able to read the file");

        let height = content.chars().filter(|c| *c == '\n').count() as isize + 1;
        let width = content.find('\n').unwrap() as isize;

        let tiles = content
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect::<Vec<isize>>();

        TerrainMap {
            tiles,
            height,
            width,
        }
    }

    fn get(&self, position: &Position) -> Option<isize> {
        if self.is_in_bounds(position) {
            self.tiles.get(self.index_from_position(position)).copied()
        } else {
            None
        }
    }

    fn index_from_position(&self, position: &Position) -> usize {
        (position.y * self.width + position.x) as usize
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        position.x >= 0 && position.y >= 0 && position.x < self.width && position.y < self.height
    }

    fn next_position(&self, position: &Position) -> Option<Position> {
        let current_index = self.index_from_position(position);
        let next_index = current_index + 1;

        if next_index >= self.tiles.len() {
            return None
        }
        let next_x = next_index as isize % self.width;
        let next_y = next_index as isize / self.width;

        Some(Position::new(next_x, next_y))
    }

    fn iter(&self) -> TerrainMapIterator {
        TerrainMapIterator::new(self)
    }

    fn score_trailhead(&self, position: &Position) -> isize {
        let mut trail_end_positions = vec![];
        self.score_trailhead_step(position, 0, &mut trail_end_positions);

        trail_end_positions.iter().unique().count() as isize
    }

    fn rate_trailhead(&self, position: &Position) -> isize {
        let mut trail_end_positions = vec![];
        self.score_trailhead_step(position, 0, &mut trail_end_positions);

        trail_end_positions.iter().count() as isize
    }

    fn score_trailhead_step(&self, current_position: &Position, current_elevation: isize, trail_end_list: &mut Vec<Position>) {
        if !self.is_in_bounds(current_position) {
            return
        }

        let next_elevation = current_elevation + 1;

        let positions = [
            get_north_position(current_position),
            get_east_position(current_position),
            get_south_position(current_position),
            get_west_position(current_position)
        ];

        for position in positions {
            match self.get(&position) {
                Some(elevation) => {
                    if elevation == next_elevation {
                        if next_elevation == 9 {
                            trail_end_list.push(position);
                        } else if self.is_in_bounds(&position) {
                            self.score_trailhead_step(&position, next_elevation, trail_end_list);
                        }
                    }
                },
                None => (),
            }
        }
    }
}

fn get_north_position(position: &Position) -> Position {
    Position::new(position.x, position.y + 1)
}

fn get_south_position(position: &Position) -> Position {
    Position::new(position.x, position.y - 1)
}

fn get_west_position(position: &Position) -> Position {
    Position::new(position.x - 1, position.y)
}

fn get_east_position(position: &Position) -> Position {
    Position::new(position.x + 1, position.y)
}

struct TerrainMapIterator<'a> {
    position: Position,
    terrain_map: &'a TerrainMap,
}

impl TerrainMapIterator<'_> {
    fn new(terrain_map: &TerrainMap) -> TerrainMapIterator {
        TerrainMapIterator {
            position: Position::new(0, 0),
            terrain_map,
        }
    }
}

impl Iterator for TerrainMapIterator<'_> {
    type Item = (Position, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next_position) = self.terrain_map.next_position(&self.position) else {
            return None;
        };

        let next_value = self.terrain_map.get(&next_position).unwrap();
        self.position = next_position;
        Some((next_position, next_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 638);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 1289);
    }
}