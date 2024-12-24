use std::fs;

pub fn part_one() -> usize {
    // get the lists from the txt file
    let contents = fs::read_to_string("inputs/day_two_input.txt")
        .expect("Should have been able to read the file");

    let safe_report_count = contents
        // split into lines
        .split("\n")
        // split and parse numbers collecting a Vec<isize>
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        // calculate if the report is safe, filter out unsafe reports, and count the remaining safe
        // ones
        .map(|report| {
            is_safe(&report)
        }).filter(|x| *x)
        .count();

    println!("Safe report count: {safe_report_count}");
    safe_report_count
}

fn is_safe(report: &Vec<isize>) -> bool {
    (is_increasing(report) || is_decreasing(report)) && is_small_delta(report)
}

fn is_small_delta(report: &Vec<isize>) -> bool {
    compare_to_previous(
        report,
        |value, previous| {
            (value - previous).abs() <= 3
        })
}

fn is_increasing(report: &Vec<isize>) -> bool {
    compare_to_previous(
        report,
        |value, previous| {
        value > previous
    })
}

fn is_decreasing(report: &Vec<isize>) -> bool {
    compare_to_previous(
        report,
        |value, previous| {
            value < previous
        })
}

fn compare_to_previous<F>(list: &Vec<isize>, predicate: F)  ->  bool
where F: Fn(&isize, &isize) -> bool {

    let mut report_iter = list.iter();
    let mut previous = report_iter.next().unwrap();
    for value in report_iter {
        if !predicate(value, previous) {
            return false
        }
        previous = value;
    }
    true
}

pub fn part_two() -> usize {
    // get the lists from the txt file
    let contents = fs::read_to_string("inputs/day_two_input.txt")
        .expect("Should have been able to read the file");

    let safe_report_count = contents
        // split into lines
        .split("\n")
        // split and parse numbers collecting a Vec<isize>
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        // expand each report to the set of possible reports
        .map(|x| {
            expand_report(&x)
        })
        // calculate if any of the reports in the set is safe. This means the original report was
        // safe.
        .map(|reports| {
            reports
                .iter()
                .any(|report| {
                    is_safe(report)
                })
        })
        // filter out unsafe reports, and count the remaining safe ones
        .filter(|x| *x )
        .count();

    println!("Safe report count: {safe_report_count}");
    safe_report_count
}

fn expand_report(report: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut reports = Vec::new();
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        reports.push(new_report);
    }

    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_increasing_test() {
        let report = vec![1, 2, 3, 4, 5];

        assert!(is_increasing(&report));
    }

    #[test]
    fn is_increasing_test_2() {
        let report = vec![1, 2, 2, 4, 5];

        assert!(!is_increasing(&report));
    }

    #[test]
    fn is_increasing_test_3() {
        let report = vec![1, 2, 3, 4, 1];

        assert!(!is_increasing(&report));
    }

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 282);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 349);
    }
}
