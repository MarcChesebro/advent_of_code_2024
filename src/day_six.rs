use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs;
use std::time::Instant;

pub fn part_one() -> isize {
    let contents = fs::read_to_string("inputs/day_six_input.txt")
        .expect("Should have been able to read the file");

    let mut patrol_matrix = PatrolMatrix::new(&*contents);

    let mut step_count = 0;
    let now = Instant::now();
    while patrol_matrix.find_guard().is_some() {
        step_count += 1;
        patrol_matrix.run_patrol_step();
    }
    let elapsed = now.elapsed();

    let seen_count = patrol_matrix.count_seen();
    println!("The guard saw {seen_count} spaces in {step_count} steps.\nIt took {:.2?}.", elapsed);

    seen_count as isize
}

pub fn part_two() -> isize {
    let contents = fs::read_to_string("inputs/day_six_input.txt")
        .expect("Should have been able to read the file");

    let mut patrol_matrix = PatrolMatrix::new(&*contents);

    while patrol_matrix.find_guard().is_some() {
        patrol_matrix.run_patrol_step();
    }

    let seen_count = patrol_matrix.count_seen();
    println!("The guard saw {seen_count} spaces");

    seen_count as isize
}

type PatrolPosition = ((isize, isize), PatrolObject);

struct PatrolMatrix {
    pub patrol_objects: Vec<PatrolObject>,
    pub patrol_turns: Vec<PatrolPosition>,
    pub width: isize,
    pub height: isize,
    pub ended_in_loop: bool,
}


impl PatrolMatrix {
    fn new(input: &str) -> PatrolMatrix {
        PatrolMatrix {
            patrol_objects: process_input(input),
            patrol_turns: Vec::new(),
            width: input.find('\n').unwrap() as isize,
            height: (input.chars().filter(|c| *c == '\n').count() + 1) as isize,
            ended_in_loop: false,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<&PatrolObject> {
        if !self.is_in_bounds((x, y)) {
            return None
        }

        let index = x + y * self.width;
        self.patrol_objects.get(index as usize)
    }

    fn set(&mut self, x: isize, y: isize, object: PatrolObject) {
        if !self.is_in_bounds((x, y)) {
            return
        }

        let index = x + y * self.width;

        // Some(std::mem::replace(&mut self.patrol_objects[index as usize], &object));
        self.patrol_objects[index as usize] = object
    }

    fn iter(&self) -> PatrolMatrixIterator {
        PatrolMatrixIterator::new(self)
    }

    fn is_in_bounds(&self, position: (isize, isize)) -> bool {
        let (x, y) = position;
        !(x < 0 || x >= self.width || y < 0 || y >= self.height)
    }

    fn run_patrol_step(&mut self) {
        let guard = self.find_guard();

        if guard.is_none() {
            return;
        }

        let (guard_position, guard_object) = guard.unwrap();

        let PatrolObject::Guard(ref direction) = guard_object else { todo!() };

        let forward_position = position_forward(guard_position, &direction);
        let object_forward = self.get(forward_position.0, forward_position.1);

        match object_forward {
            // if there is an Obstacle in front of the guard turn right
            Some(PatrolObject::Obstacle) => {
                // first check if we are in an infinite loop and if we are then return early
                if self.is_stuck_in_loop((guard_position, guard_object.clone())) {
                    self.set(guard_position.0, guard_position.1, PatrolObject::Seen);
                    self.ended_in_loop = true;
                    return
                }

                // first store the turn so that if we get here again we know we are stuck in a loop
                self.patrol_turns.push((guard_position, guard_object.clone()));
                self.set(guard_position.0, guard_position.1, PatrolObject::Guard(direction_to_right(&direction)));
            },
            // otherwise move forward and set the previous position to seen
            _ => {
                let (x, y) = position_forward(guard_position, &direction);
                self.set(x, y, PatrolObject::Guard(direction.clone()));
                self.set(guard_position.0, guard_position.1, PatrolObject::Seen);
            }
        };
    }

    fn find_guard(&self) -> Option<PatrolPosition>{
        self.iter().find(|(_, p)| {
            match p {
                PatrolObject::Guard(_) => true,
                _ => false,
            }
        })
    }

    fn count_seen(&self) -> usize {
        self.iter()
            .filter(|(_, p)| {
                match p {
                    PatrolObject::Seen => true,
                    _ => false
                }
            }).count()
    }

    fn object_position_list(&self) -> VecDeque<PatrolPosition> {
        let mut list = VecDeque::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y).unwrap();

                list.push_back(((x, y), c.clone()));
            }
        }

        list
    }

    fn is_stuck_in_loop(&self, patrol_turn: PatrolPosition) -> bool {
        let ((x1, y1), p1) = patrol_turn;

        self.patrol_turns.iter().any(|e| {
            let ((x2, y2), p2) = e;
            x1 == *x2 && y1 == *y2 && p1 == *p2
        })
    }
}

impl Display for PatrolMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let mut display = String::new();
        for ((x, _y), patrol_object) in self.iter() {
            display.push(position_to_char(&patrol_object));

            if x % self.width == self.width - 1 {
                display.push('\n');
            }
        }
        write!(f, "{}", display)
    }
}

fn position_to_char(patrol_object: &PatrolObject) -> char {
    match patrol_object {
        PatrolObject::Unseen => '.',
        PatrolObject::Seen => 'X',
        PatrolObject::Obstacle => '#',
        PatrolObject::Guard(Direction::Up) => '^',
        PatrolObject::Guard(Direction::Right) => '>',
        PatrolObject::Guard(Direction::Left) => '<',
        PatrolObject::Guard(Direction::Down) => '⌄',
    }
}

fn position_forward(position: (isize, isize), direction: &Direction) -> (isize, isize) {
     match direction {
        Direction::Up    => (position.0,     position.1 - 1),
        Direction::Down  => (position.0,     position.1 + 1),
        Direction::Right => (position.0 + 1, position.1    ),
        Direction::Left  => (position.0 - 1, position.1    ),
    }
}

fn direction_to_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up    => Direction::Right,
        Direction::Down  => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Left  => Direction::Up,
    }
}

#[derive(Clone, PartialEq)]
enum PatrolObject {
    Guard(Direction),
    Obstacle,
    Unseen,
    Seen
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn process_input(input: &str) -> Vec<PatrolObject> {
    input
        // filter out whitespace and collect into Vec<char>
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            match c {
                '^' => PatrolObject::Guard(Direction::Up),
                '#' => PatrolObject::Obstacle,
                _ => PatrolObject::Unseen,
            }
        })
        .collect::<Vec<PatrolObject>>()
}

struct PatrolMatrixIterator {
    patrol_object_list: VecDeque<PatrolPosition>,
}

impl<'a> PatrolMatrixIterator {
    fn new(patrol_matrix: &PatrolMatrix) -> PatrolMatrixIterator {
        PatrolMatrixIterator {
            patrol_object_list: patrol_matrix.object_position_list()
        }
    }
}

impl<'a> Iterator for PatrolMatrixIterator {
    type Item = PatrolPosition;

    fn next(&mut self) -> Option<Self::Item> {
        self.patrol_object_list.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 5305);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 2);
    }
}