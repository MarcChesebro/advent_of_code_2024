use std::collections::VecDeque;
use std::fs;

pub fn part_one() -> isize {
    let calibration_tests = build_calibration_tests("inputs/day_seven_input.txt");

    let operations = [
        Operation::Add,
        Operation::Multiply
    ];

    let calibration_total = run_calibration_tests(calibration_tests, &operations);

    println!("{:?}", calibration_total);
    calibration_total
}

pub fn part_two() -> isize {
    let calibration_tests = build_calibration_tests("inputs/day_seven_input.txt");

    let operations = [
        Operation::Add,
        Operation::Multiply,
        Operation::Concatenate,
    ];

    let calibration_total = run_calibration_tests(calibration_tests, &operations);

    println!("{:?}", calibration_total);
    calibration_total
}

fn build_calibration_tests(file: &str) -> Vec<CalibrationTest> {
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");

    contents
        .split('\n')
        .map(|x| CalibrationTest::new(x))
        .collect()
}

fn run_calibration_tests(mut calibration_tests: Vec<CalibrationTest>, operations: &[Operation]) -> isize {
    calibration_tests
        .iter_mut()
        .map(|test| {
            // build a tree of all the possible operation combinations
            let first_value = test.test_values.pop_front().unwrap();
            let mut tree_head = OperationNode::new(None, first_value);
            tree_head.build_children(test.test_values.clone(), operations);

            // calculate all the possible totals of every operation combination
            let mut totals_list = Vec::<isize>::new();
            tree_head.calculate_totals(0, &mut totals_list);

            (totals_list.contains(&test.answer), test.answer)
        })
        // filter out only those that can be solved and sum up the answer values
        .filter(|(b, _x)| *b)
        .map(|(_b, x)| x)
        .sum()
}

struct CalibrationTest {
    pub answer: isize,
    pub test_values: VecDeque<isize>,
}

impl CalibrationTest {
    fn new(contents: &str) -> CalibrationTest {
        let mut values = contents.split(": ");

        let answer = values.next().unwrap().parse::<isize>().unwrap();
        let operation_values = values.next().unwrap();

        let test_values = operation_values
            .split(' ')
            .map(|x| {
                x.parse::<isize>().unwrap()
            })
            .collect();

        CalibrationTest {
            answer,
            test_values,
        }
    }
}

#[derive(Debug)]
struct OperationNode {
    operation: Option<Operation>,
    value: isize,
    total: isize,
    children: Vec<OperationNode>,
}

impl OperationNode {
    fn new(operation: Option<Operation>, value: isize) -> OperationNode {
        OperationNode {
            operation,
            value,
            total: 0,
            children: Vec::new(),
        }
    }

    fn build_children(&mut self, mut children: VecDeque<isize>, operations: &[Operation]) {
        let new_value = match children.pop_front() {
            Some(value) => value,
            None => {
                return
            },
        };

        for operation in operations {
            self.children.push(OperationNode::new(Some(*operation), new_value));
        }

        for child in &mut self.children {
            child.build_children(children.clone(), operations);
        }
    }

    fn calculate_totals(&mut self, total: isize, totals_list: &mut Vec<isize>) {
        match &self.operation {
            Some(operation) => self.total = operation.perform(total, self.value),
            None => self.total = self.value,
        }

        for child in self.children.iter_mut() {
            child.calculate_totals(self.total, totals_list);
        }

        if self.is_leaf() {
            totals_list.push(self.total);
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
    Concatenate
}

impl Operation {
    fn perform(&self, lhs: isize, rhs: isize) -> isize {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Multiply => lhs * rhs,
            Operation::Concatenate => concatenate(lhs, rhs),
        }
    }
}

fn concatenate(lhs: isize, rhs: isize) -> isize {
    let mut new_value = lhs.to_string();
    new_value.push_str(rhs.to_string().as_str());
    new_value.parse::<isize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 3_312_271_365_652);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 509_463_489_296_712);
    }
}